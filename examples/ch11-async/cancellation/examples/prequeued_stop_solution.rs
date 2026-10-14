use ch11_cancellation::models::EndReason;
use ch11_cancellation::models::Outcome;
use ch11_cancellation::models::StopEvent;
use ch11_cancellation::prepare_jobs;
use ch11_cancellation::run_batch;
use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let specs = vec![(80, false), (120, false), (200, false)];
    let (jobs, releases) = prepare_jobs(specs.as_slice());
    let (started_tx, mut started_rx) = mpsc::channel::<usize>(1);
    let (stop_tx, stop_rx) = mpsc::channel::<StopEvent>(1);
    stop_tx
        .send(StopEvent::Requested)
        .await
        .expect("批次持有停止收端");
    let batch = run_batch(jobs, 2, started_tx, stop_rx)
        .await
        .expect("上限為正數");
    assert_eq!(batch.reason, EndReason::Requested);
    assert_eq!(batch.submitted, 0);
    assert_eq!(batch.peak_tracked, 0);
    assert_eq!(batch.reports.len(), 3);
    assert_eq!(started_rx.recv().await, None);
    for report in batch.reports.as_slice() {
        assert_eq!(report.outcome, Outcome::Unstarted);
        println!("input {}：{:?}", report.input, report.outcome);
    }
    for release in releases.as_slice() {
        match release.send(()).await {
            Err(_) => (),
            Ok(()) => panic!("未提交 Job 的收端已清理"),
        }
    }
    println!("提交 0；沒有 task 需要取消");
}
