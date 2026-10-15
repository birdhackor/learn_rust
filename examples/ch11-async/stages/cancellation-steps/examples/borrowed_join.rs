use ch11_tasks::Outcome;
use ch11_tasks::collect_one;
use ch11_tasks::controlled_work;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::task::JoinSet;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (started_tx, mut started_rx) = mpsc::channel::<()>(1);
    let (release_tx, release_rx) = mpsc::channel::<()>(1);
    let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
    let mut tasks = JoinSet::new();
    let mut inputs = HashMap::new();
    let abort = tasks.spawn(controlled_work(80, false, started_tx, release_rx));
    inputs.insert(abort.id(), 0);
    let controller = tokio::spawn(async move {
        assert_eq!(started_rx.recv().await, Some(()));
        stop_tx.send(()).await.expect("main 保持停止收端");
    });
    tokio::select! {
        biased;
        event = stop_rx.recv() => assert_eq!(event, Some(())),
        _result = tasks.join_next_with_id() => panic!("release 尚未放行"),
    }
    assert_eq!(tasks.len(), 1);
    assert_eq!(inputs.len(), 1);
    release_tx.send(()).await.expect("task 仍持有放行收端");
    let report = collect_one(&mut tasks, &mut inputs)
        .await
        .expect("原task仍在集合");
    assert_eq!(report.input, 0);
    assert_eq!(report.outcome, Outcome::Value(80));
    match controller.await {
        Ok(()) => (),
        Err(error) => panic!("controller 應正常完成：{error}"),
    }
    assert_eq!(tasks.len(), 0);
    assert_eq!(inputs.len(), 0);
    println!("停止分支先返回；原 task 仍正常完成：{:?}", report.outcome);
}
