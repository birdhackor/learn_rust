pub mod models;

use crate::models::Batch;
use crate::models::EndReason;
use crate::models::Outcome;
use crate::models::Report;
use crate::models::StopEvent;
use crate::models::WorkError;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::task::Id;
use tokio::task::JoinError;
use tokio::task::JoinSet;

// --8<-- [start:jobs]
pub struct Job {
    pub amount: i32,
    pub panic_after_release: bool,
    pub release: mpsc::Receiver<()>,
    pub returning: Option<mpsc::Sender<usize>>,
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
            returning: None,
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
    let output = if job.amount < 0 {
        Err(WorkError::Negative(job.amount))
    } else {
        Ok(job.amount)
    };
    if let Some(returning) = job.returning {
        returning.send(input).await.expect("測試保持回傳觀察收端");
    }
    output
}
// --8<-- [end:controlled-job]

// --8<-- [start:classify]
fn classify(
    result: Result<(Id, Result<i32, WorkError>), JoinError>,
    inputs: &mut HashMap<Id, usize>,
) -> Report {
    let (id, outcome) = match result {
        Ok((id, Ok(value))) => (id, Outcome::Value(value)),
        Ok((id, Err(error))) => (id, Outcome::WorkError(error)),
        Err(error) => {
            let id = error.id();
            let outcome = if error.is_cancelled() {
                Outcome::Cancelled
            } else if error.is_panic() {
                Outcome::Panic
            } else {
                Outcome::OtherTaskFailure(error.to_string())
            };
            (id, outcome)
        }
    };
    let input = inputs.remove(&id).expect("每份提交工作都有輸入對照");
    Report { input, outcome }
}
// --8<-- [end:classify]

// --8<-- [start:state]
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    ZeroLimit,
}

struct Runner {
    pending: std::vec::IntoIter<Job>,
    tasks: JoinSet<Result<i32, WorkError>>,
    inputs: HashMap<Id, usize>,
    outcomes: Vec<Option<Outcome>>,
    limit: usize,
    submitted: usize,
    peak_tracked: usize,
}
// --8<-- [end:state]

impl Runner {
    // --8<-- [start:prepare-and-fill]
    fn new(jobs: Vec<Job>, limit: usize) -> Result<Self, ConfigError> {
        if limit == 0 {
            return Err(ConfigError::ZeroLimit);
        }
        let mut outcomes = Vec::new();
        for _ in 0..jobs.len() {
            outcomes.push(None);
        }
        Ok(Self {
            pending: jobs.into_iter(),
            tasks: JoinSet::new(),
            inputs: HashMap::new(),
            outcomes,
            limit,
            submitted: 0,
            peak_tracked: 0,
        })
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

    fn record(&mut self, result: Result<(Id, Result<i32, WorkError>), JoinError>) {
        let report = classify(result, &mut self.inputs);
        self.outcomes.as_mut_slice()[report.input] = Some(report.outcome);
    }
    // --8<-- [end:prepare-and-fill]

    // --8<-- [start:stop-and-drain]
    async fn stop_and_finish(mut self, reason: EndReason) -> Batch {
        self.tasks.abort_all();
        while let Some(result) = self.tasks.join_next_with_id().await {
            self.record(result);
        }
        for input in self.submitted..self.outcomes.len() {
            let job = self.pending.next().expect("每份未提交輸入仍在 pending");
            drop(job);
            self.outcomes.as_mut_slice()[input] = Some(Outcome::Unstarted);
        }
        self.into_batch(reason)
    }

    fn into_batch(self, reason: EndReason) -> Batch {
        assert_eq!(self.tasks.len(), 0);
        assert_eq!(self.inputs.len(), 0);
        let mut reports = Vec::new();
        for (input, outcome) in self.outcomes.into_iter().enumerate() {
            let outcome = match outcome {
                Some(outcome) => outcome,
                None => panic!("每份輸入都必須有結果或未提交分類"),
            };
            reports.push(Report { input, outcome });
        }
        Batch {
            reports,
            reason,
            submitted: self.submitted,
            peak_tracked: self.peak_tracked,
        }
    }
    // --8<-- [end:stop-and-drain]
}

// --8<-- [start:stop-reason]
fn stop_reason(event: Option<StopEvent>) -> EndReason {
    match event {
        Some(StopEvent::Requested) => EndReason::Requested,
        Some(StopEvent::SignalFailed(message)) => EndReason::SignalFailed(message),
        None => EndReason::StopSourceClosed,
    }
}
// --8<-- [end:stop-reason]

// --8<-- [start:run-batch]
pub async fn run_batch(
    jobs: Vec<Job>,
    limit: usize,
    started: mpsc::Sender<usize>,
    mut stop: mpsc::Receiver<StopEvent>,
) -> Result<Batch, ConfigError> {
    let mut runner = Runner::new(jobs, limit)?;
    if runner.outcomes.is_empty() {
        return Ok(runner.into_batch(EndReason::Finished));
    }
    loop {
        match stop.try_recv() {
            Ok(event) => return Ok(runner.stop_and_finish(stop_reason(Some(event))).await),
            Err(TryRecvError::Disconnected) => {
                return Ok(runner.stop_and_finish(EndReason::StopSourceClosed).await);
            }
            Err(TryRecvError::Empty) => (),
        }
        runner.fill(&started);
        if runner.tasks.is_empty() {
            return Ok(runner.into_batch(EndReason::Finished));
        }
        tokio::select! {
            biased;
            event = stop.recv() => {
                return Ok(runner.stop_and_finish(stop_reason(event)).await);
            }
            result = runner.tasks.join_next_with_id() => {
                runner.record(result.expect("此時集合至少有一份task"));
            }
        }
    }
}
// --8<-- [end:run-batch]

// --8<-- [start:borrowed-join-demo]
pub async fn borrowed_join_demo() -> Outcome {
    let specs = vec![(80, false)];
    let (jobs, releases) = prepare_jobs(specs.as_slice());
    let mut pending = jobs.into_iter();
    let job = pending.next().expect("有一份輸入");
    let (started_tx, mut started_rx) = mpsc::channel::<usize>(1);
    let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
    let mut tasks = JoinSet::new();
    let mut inputs = HashMap::new();
    let abort = tasks.spawn(controlled_job(0, job, started_tx));
    inputs.insert(abort.id(), 0);
    let controller = tokio::spawn(async move {
        assert_eq!(started_rx.recv().await, Some(0));
        stop_tx.send(()).await.expect("demo 保持停止收端");
    });
    tokio::select! {
        biased;
        event = stop_rx.recv() => assert_eq!(event, Some(())),
        _result = tasks.join_next_with_id() => panic!("release 尚未放行"),
    }
    assert_eq!(tasks.len(), 1);
    assert_eq!(inputs.len(), 1);
    releases.as_slice()[0]
        .send(())
        .await
        .expect("task 仍持有放行收端");
    let result = tasks.join_next_with_id().await.expect("原task仍由集合追蹤");
    let report = classify(result, &mut inputs);
    assert_eq!(report.input, 0);
    assert_eq!(report.outcome, Outcome::Value(80));
    match controller.await {
        Ok(()) => (),
        Err(_) => panic!("受控controller應正常完成"),
    }
    assert_eq!(tasks.len(), 0);
    assert_eq!(inputs.len(), 0);
    report.outcome
}
// --8<-- [end:borrowed-join-demo]

// --8<-- [start:stop-demo]
pub async fn stop_demo() -> Batch {
    let specs = vec![
        (80, false),
        (-1, false),
        (120, true),
        (200, false),
        (30, false),
    ];
    let (jobs, releases) = prepare_jobs(specs.as_slice());
    let (started_tx, mut started_rx) = mpsc::channel::<usize>(2);
    let (stop_tx, stop_rx) = mpsc::channel::<StopEvent>(1);
    let controller = tokio::spawn(async move {
        let first = started_rx.recv().await.expect("第一批兩份");
        let second = started_rx.recv().await.expect("第一批兩份");
        assert!((first == 0 && second == 1) || (first == 1 && second == 0));
        releases.as_slice()[1]
            .send(())
            .await
            .expect("input 1 仍等待");
        assert_eq!(started_rx.recv().await, Some(2));
        releases.as_slice()[2]
            .send(())
            .await
            .expect("input 2 仍等待");
        assert_eq!(started_rx.recv().await, Some(3));
        stop_tx
            .send(StopEvent::Requested)
            .await
            .expect("批次仍監聽停止");
        releases
    });
    let batch = run_batch(jobs, 2, started_tx, stop_rx)
        .await
        .expect("上限為正數");
    let releases = match controller.await {
        Ok(releases) => releases,
        Err(_) => panic!("受控controller應正常完成"),
    };
    for release in releases.as_slice() {
        match release.send(()).await {
            Err(_) => (),
            Ok(()) => panic!("批次已等待並清理每份Job的放行收端"),
        }
    }
    batch
}
// --8<-- [end:stop-demo]

// --8<-- [start:tests]
#[cfg(test)]
mod tests {
    use crate::Runner;
    use crate::borrowed_join_demo;
    use crate::models::EndReason;
    use crate::models::Outcome;
    use crate::models::StopEvent;
    use crate::models::WorkError;
    use crate::prepare_jobs;
    use crate::run_batch;
    use crate::stop_demo;
    use tokio::sync::mpsc;

    #[tokio::test(flavor = "current_thread")]
    async fn losing_borrowed_join_wait_does_not_drop_its_task() {
        assert_eq!(borrowed_join_demo().await, Outcome::Value(80));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn stop_after_refill_keeps_errors_and_does_not_submit_remaining_input() {
        let batch = stop_demo().await;
        assert_eq!(batch.reason, EndReason::Requested);
        assert_eq!(batch.submitted, 4);
        assert_eq!(batch.peak_tracked, 2);
        assert_eq!(batch.reports.len(), 5);
        let reports = batch.reports.as_slice();
        assert_eq!(reports[0].input, 0);
        assert_eq!(reports[0].outcome, Outcome::Cancelled);
        assert_eq!(reports[1].input, 1);
        assert_eq!(
            reports[1].outcome,
            Outcome::WorkError(WorkError::Negative(-1))
        );
        assert_eq!(reports[2].input, 2);
        assert_eq!(reports[2].outcome, Outcome::Panic);
        assert_eq!(reports[3].input, 3);
        assert_eq!(reports[3].outcome, Outcome::Cancelled);
        assert_eq!(reports[4].input, 4);
        assert_eq!(reports[4].outcome, Outcome::Unstarted);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn prequeued_stop_submits_nothing_and_cleans_unstarted_jobs() {
        let specs = vec![(80, false), (120, false)];
        let (jobs, releases) = prepare_jobs(specs.as_slice());
        let (started_tx, mut started_rx) = mpsc::channel::<usize>(1);
        let (stop_tx, stop_rx) = mpsc::channel::<StopEvent>(1);
        stop_tx
            .send(StopEvent::Requested)
            .await
            .expect("收端仍存在");
        let batch = run_batch(jobs, 2, started_tx, stop_rx)
            .await
            .expect("正上限");
        assert_eq!(batch.reason, EndReason::Requested);
        assert_eq!(batch.submitted, 0);
        assert_eq!(batch.peak_tracked, 0);
        assert_eq!(batch.reports.len(), 2);
        assert_eq!(started_rx.recv().await, None);
        for report in batch.reports.as_slice() {
            assert_eq!(report.outcome, Outcome::Unstarted);
        }
        for release in releases.as_slice() {
            match release.send(()).await {
                Err(_) => (),
                Ok(()) => panic!("所有未提交收端都應已清理"),
            }
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn abort_and_drain_preserves_a_completed_but_uncollected_value() {
        let specs = vec![(80, false), (120, false), (200, false)];
        let (mut jobs, releases) = prepare_jobs(specs.as_slice());
        let (returning_tx, mut returning_rx) = mpsc::channel::<usize>(1);
        jobs.as_mut_slice()[0].returning = Some(returning_tx);
        let (started_tx, mut started_rx) = mpsc::channel::<usize>(2);
        let mut runner = Runner::new(jobs, 2).expect("正上限");
        runner.fill(&started_tx);
        let first = started_rx.recv().await.expect("兩份工作已提交");
        let second = started_rx.recv().await.expect("兩份工作已提交");
        assert!((first == 0 && second == 1) || (first == 1 && second == 0));
        releases.as_slice()[0]
            .send(())
            .await
            .expect("input 0 等待放行");
        assert_eq!(returning_rx.recv().await, Some(0));
        assert_eq!(runner.tasks.len(), 2);
        let batch = runner.stop_and_finish(EndReason::Requested).await;
        assert_eq!(batch.submitted, 2);
        assert_eq!(batch.peak_tracked, 2);
        assert_eq!(batch.reports.len(), 3);
        assert_eq!(batch.reports.as_slice()[0].outcome, Outcome::Value(80));
        assert_eq!(batch.reports.as_slice()[1].outcome, Outcome::Cancelled);
        assert_eq!(batch.reports.as_slice()[2].outcome, Outcome::Unstarted);
        for release in releases.as_slice() {
            match release.send(()).await {
                Err(_) => (),
                Ok(()) => panic!("所有Job收端都應已清理"),
            }
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn signal_failure_and_closed_source_have_distinct_reasons() {
        let specs = vec![(80, false)];
        let (jobs, _releases) = prepare_jobs(specs.as_slice());
        let (started_tx, _started_rx) = mpsc::channel::<usize>(1);
        let (stop_tx, stop_rx) = mpsc::channel::<StopEvent>(1);
        stop_tx
            .send(StopEvent::SignalFailed("受控監聽錯誤".to_owned()))
            .await
            .expect("收端存在");
        let batch = run_batch(jobs, 2, started_tx, stop_rx)
            .await
            .expect("正上限");
        assert_eq!(
            batch.reason,
            EndReason::SignalFailed("受控監聽錯誤".to_owned())
        );
        assert_eq!(batch.submitted, 0);
        assert_eq!(batch.reports.as_slice()[0].outcome, Outcome::Unstarted);

        let (jobs, _releases) = prepare_jobs(specs.as_slice());
        let (started_tx, _started_rx) = mpsc::channel::<usize>(1);
        let (stop_tx, stop_rx) = mpsc::channel::<StopEvent>(1);
        drop(stop_tx);
        let batch = run_batch(jobs, 2, started_tx, stop_rx)
            .await
            .expect("正上限");
        assert_eq!(batch.reason, EndReason::StopSourceClosed);
        assert_eq!(batch.submitted, 0);
        assert_eq!(batch.reports.as_slice()[0].outcome, Outcome::Unstarted);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn empty_input_finishes_with_no_tasks_or_results() {
        let (started_tx, mut started_rx) = mpsc::channel::<usize>(1);
        let (_stop_tx, stop_rx) = mpsc::channel::<StopEvent>(1);
        let batch = run_batch(Vec::new(), 2, started_tx, stop_rx)
            .await
            .expect("正上限");
        assert_eq!(batch.reason, EndReason::Finished);
        assert_eq!(batch.submitted, 0);
        assert_eq!(batch.peak_tracked, 0);
        assert_eq!(batch.reports.len(), 0);
        assert_eq!(started_rx.recv().await, None);
    }
}
// --8<-- [end:tests]
