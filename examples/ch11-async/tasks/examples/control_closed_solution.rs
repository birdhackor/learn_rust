use ch11_tasks::Outcome;
use ch11_tasks::WorkError;
use ch11_tasks::collect_one;
use ch11_tasks::controlled_work;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::task::JoinSet;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (started_tx, mut started_rx) = mpsc::channel::<()>(1);
    let (release_tx, release_rx) = mpsc::channel::<()>(1);
    let mut tasks = JoinSet::new();
    let mut inputs = HashMap::new();
    let abort = tasks.spawn(controlled_work(80, false, started_tx, release_rx));
    inputs.insert(abort.id(), 0);
    assert_eq!(started_rx.recv().await, Some(()));

    drop(release_tx);
    let report = collect_one(&mut tasks, &mut inputs)
        .await
        .expect("還有 input 0");
    assert_eq!(report.input, 0);
    assert_eq!(report.outcome, Outcome::WorkError(WorkError::ControlClosed));
    println!("input {}：{:?}", report.input, report.outcome);
    match collect_one(&mut tasks, &mut inputs).await {
        None => (),
        Some(_) => panic!("唯一 task 已回收"),
    }
    assert_eq!(inputs.len(), 0);
}
