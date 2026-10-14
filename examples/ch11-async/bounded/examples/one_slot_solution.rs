use ch11_bounded::Scheduler;
use ch11_bounded::outcomes::Outcome;
use ch11_bounded::outcomes::WorkError;
use ch11_bounded::prepare_jobs;
use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let specs = vec![(80, false), (-1, false), (120, false)];
    let (jobs, releases) = prepare_jobs(specs.as_slice());
    let (started_tx, mut started_rx) = mpsc::channel::<usize>(1);
    let mut scheduler = Scheduler::new(jobs, 1, &started_tx).expect("上限為正數");
    assert_eq!(scheduler.submitted(), 1);
    let mut reports = Vec::new();
    for input in 0..specs.len() {
        assert_eq!(started_rx.recv().await, Some(input));
        assert_eq!(scheduler.tracked(), 1);
        releases.as_slice()[input]
            .send(())
            .await
            .expect("本項已提交");
        let report = scheduler.next(&started_tx).await.expect("本項可完成");
        assert_eq!(report.input, input);
        println!("input {}：{:?}", report.input, report.outcome);
        reports.push(report);
    }
    assert_eq!(reports.as_slice()[0].outcome, Outcome::Value(80));
    assert_eq!(
        reports.as_slice()[1].outcome,
        Outcome::WorkError(WorkError::Negative(-1))
    );
    assert_eq!(reports.as_slice()[2].outcome, Outcome::Value(120));
    assert_eq!(scheduler.peak_tracked(), 1);
    assert_eq!(scheduler.submitted(), 3);
    assert_eq!(scheduler.tracked(), 0);
    match scheduler.next(&started_tx).await {
        None => (),
        Some(_) => panic!("三份工作都已回收"),
    }
    drop(started_tx);
    assert_eq!(started_rx.recv().await, None);
    println!("上限 1；追蹤峰值 1；三份工作全部回收");
}
