pub mod outcomes;

use crate::outcomes::Report;
use crate::outcomes::WorkError;
use crate::outcomes::collect_one;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::task::Id;
use tokio::task::JoinSet;

// --8<-- [start:jobs]
pub struct Job {
    pub amount: i32,
    pub panic_after_release: bool,
    pub release: mpsc::Receiver<()>,
}

pub fn prepare_jobs(specs: &[(i32, bool)]) -> (Vec<Job>, Vec<mpsc::Sender<()>>) {
    let mut jobs = Vec::new();
    let mut releases = Vec::new();
    for spec in specs {
        let (sender, receiver) = mpsc::channel::<()>(1);
        jobs.push(Job {
            amount: spec.0,
            panic_after_release: spec.1,
            release: receiver,
        });
        releases.push(sender);
    }
    (jobs, releases)
}
// --8<-- [end:jobs]

// --8<-- [start:controlled-job]
async fn controlled_job(
    input: usize,
    mut job: Job,
    started: mpsc::Sender<usize>,
) -> Result<i32, WorkError> {
    started.send(input).await.expect("controller 保持啟動收端");
    drop(started);
    match job.release.recv().await {
        Some(()) => (),
        None => return Err(WorkError::ControlClosed),
    }
    if job.panic_after_release {
        panic!("task 故意 panic");
    }
    if job.amount < 0 {
        Err(WorkError::Negative(job.amount))
    } else {
        Ok(job.amount)
    }
}
// --8<-- [end:controlled-job]

// --8<-- [start:scheduler-state]
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    ZeroLimit,
}

pub struct Scheduler {
    pending: std::vec::IntoIter<Job>,
    tasks: JoinSet<Result<i32, WorkError>>,
    inputs: HashMap<Id, usize>,
    limit: usize,
    submitted: usize,
    peak_tracked: usize,
}
// --8<-- [end:scheduler-state]

impl Scheduler {
    // --8<-- [start:new-and-fill]
    pub fn new(
        jobs: Vec<Job>,
        limit: usize,
        started: &mpsc::Sender<usize>,
    ) -> Result<Self, ConfigError> {
        if limit == 0 {
            return Err(ConfigError::ZeroLimit);
        }
        let mut scheduler = Self {
            pending: jobs.into_iter(),
            tasks: JoinSet::new(),
            inputs: HashMap::new(),
            limit,
            submitted: 0,
            peak_tracked: 0,
        };
        scheduler.fill(started);
        Ok(scheduler)
    }

    fn fill(&mut self, started: &mpsc::Sender<usize>) {
        while self.tasks.len() < self.limit {
            let job = match self.pending.next() {
                Some(job) => job,
                None => break,
            };
            let input = self.submitted;
            let abort = self
                .tasks
                .spawn(controlled_job(input, job, started.clone()));
            self.inputs.insert(abort.id(), input);
            self.submitted += 1;
            if self.tasks.len() > self.peak_tracked {
                self.peak_tracked = self.tasks.len();
            }
        }
    }
    // --8<-- [end:new-and-fill]

    // --8<-- [start:next-and-counts]
    pub async fn next(&mut self, started: &mpsc::Sender<usize>) -> Option<Report> {
        let report = match collect_one(&mut self.tasks, &mut self.inputs).await {
            Some(report) => report,
            None => return None,
        };
        self.fill(started);
        Some(report)
    }

    pub fn tracked(&self) -> usize {
        self.tasks.len()
    }

    pub fn submitted(&self) -> usize {
        self.submitted
    }

    pub fn peak_tracked(&self) -> usize {
        self.peak_tracked
    }
    // --8<-- [end:next-and-counts]
}

// --8<-- [start:batch]
pub struct Batch {
    pub reports: Vec<Report>,
    pub submitted: usize,
    pub peak_tracked: usize,
}
// --8<-- [end:batch]

// --8<-- [start:demo-initial]
pub async fn run_demo() -> Batch {
    let specs = vec![
        (80, false),
        (-1, false),
        (120, true),
        (200, false),
        (30, false),
    ];
    let (jobs, releases) = prepare_jobs(specs.as_slice());
    let (started_tx, mut started_rx) = mpsc::channel::<usize>(2);
    let mut scheduler = Scheduler::new(jobs, 2, &started_tx).expect("上限為正數");
    assert_eq!(scheduler.submitted(), 2);
    assert_eq!(scheduler.tracked(), 2);
    let first_started = started_rx.recv().await.expect("第一批有兩份工作");
    let second_started = started_rx.recv().await.expect("第一批有兩份工作");
    assert!(
        (first_started == 0 && second_started == 1) || (first_started == 1 && second_started == 0)
    );
    // --8<-- [end:demo-initial]

    // --8<-- [start:demo-refill]
    let mut reports = Vec::new();
    releases.as_slice()[1]
        .send(())
        .await
        .expect("input 1 已提交");
    let report = scheduler.next(&started_tx).await.expect("input 1 可完成");
    assert_eq!(report.input, 1);
    reports.push(report);
    assert_eq!(scheduler.submitted(), 3);
    assert_eq!(scheduler.tracked(), 2);
    assert_eq!(started_rx.recv().await, Some(2));

    releases.as_slice()[2]
        .send(())
        .await
        .expect("input 2 已提交");
    let report = scheduler.next(&started_tx).await.expect("input 2 可完成");
    assert_eq!(report.input, 2);
    reports.push(report);
    assert_eq!(scheduler.submitted(), 4);
    assert_eq!(scheduler.tracked(), 2);
    assert_eq!(started_rx.recv().await, Some(3));

    releases.as_slice()[3]
        .send(())
        .await
        .expect("input 3 已提交");
    let report = scheduler.next(&started_tx).await.expect("input 3 可完成");
    assert_eq!(report.input, 3);
    reports.push(report);
    assert_eq!(scheduler.submitted(), 5);
    assert_eq!(scheduler.tracked(), 2);
    assert_eq!(started_rx.recv().await, Some(4));
    // --8<-- [end:demo-refill]

    // --8<-- [start:demo-drain]
    releases.as_slice()[0]
        .send(())
        .await
        .expect("input 0 仍在等待");
    let report = scheduler.next(&started_tx).await.expect("input 0 可完成");
    assert_eq!(report.input, 0);
    reports.push(report);
    assert_eq!(scheduler.submitted(), 5);
    assert_eq!(scheduler.tracked(), 1);

    releases.as_slice()[4]
        .send(())
        .await
        .expect("input 4 仍在等待");
    let report = scheduler.next(&started_tx).await.expect("input 4 可完成");
    assert_eq!(report.input, 4);
    reports.push(report);
    assert_eq!(scheduler.tracked(), 0);
    match scheduler.next(&started_tx).await {
        None => (),
        Some(_) => panic!("所有提交的工作都已回收"),
    }
    assert_eq!(scheduler.inputs.len(), 0);
    drop(started_tx);
    assert_eq!(started_rx.recv().await, None);
    Batch {
        reports,
        submitted: scheduler.submitted(),
        peak_tracked: scheduler.peak_tracked(),
    }
}
// --8<-- [end:demo-drain]

// --8<-- [start:tests]
#[cfg(test)]
mod tests {
    use crate::ConfigError;
    use crate::Scheduler;
    use crate::outcomes::Outcome;
    use crate::outcomes::WorkError;
    use crate::prepare_jobs;
    use crate::run_demo;
    use tokio::sync::mpsc;

    #[tokio::test(flavor = "current_thread")]
    async fn bounded_refill_keeps_inputs_after_work_error_and_panic() {
        let batch = run_demo().await;
        assert_eq!(batch.submitted, 5);
        assert_eq!(batch.peak_tracked, 2);
        assert_eq!(batch.reports.len(), 5);
        let reports = batch.reports.as_slice();
        assert_eq!(reports[0].input, 1);
        assert_eq!(
            reports[0].outcome,
            Outcome::WorkError(WorkError::Negative(-1))
        );
        assert_eq!(reports[1].input, 2);
        assert_eq!(reports[1].outcome, Outcome::Panic);
        assert_eq!(reports[2].input, 3);
        assert_eq!(reports[2].outcome, Outcome::Value(200));
        assert_eq!(reports[3].input, 0);
        assert_eq!(reports[3].outcome, Outcome::Value(80));
        assert_eq!(reports[4].input, 4);
        assert_eq!(reports[4].outcome, Outcome::Value(30));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn zero_limit_is_rejected_before_any_task_is_submitted() {
        let specs = vec![(80, false)];
        let (jobs, releases) = prepare_jobs(specs.as_slice());
        let (started_tx, mut started_rx) = mpsc::channel::<usize>(1);
        match Scheduler::new(jobs, 0, &started_tx) {
            Err(error) => assert_eq!(error, ConfigError::ZeroLimit),
            Ok(_) => panic!("零上限必須在提交工作前拒絕"),
        }
        drop(started_tx);
        assert_eq!(started_rx.recv().await, None);
        match releases.as_slice()[0].send(()).await {
            Err(_) => (),
            Ok(()) => panic!("拒絕時，尚未提交Job的收端應已清理"),
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn empty_input_finishes_without_spawning_or_waiting_for_a_message() {
        let (started_tx, mut started_rx) = mpsc::channel::<usize>(1);
        let mut scheduler = Scheduler::new(Vec::new(), 2, &started_tx).expect("上限為正數");
        assert_eq!(scheduler.submitted(), 0);
        assert_eq!(scheduler.tracked(), 0);
        assert_eq!(scheduler.peak_tracked(), 0);
        match scheduler.next(&started_tx).await {
            None => (),
            Some(_) => panic!("空輸入沒有task 結果"),
        }
        drop(started_tx);
        assert_eq!(started_rx.recv().await, None);
    }
}
// --8<-- [end:tests]
