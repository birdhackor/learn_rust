use ch11_tasks::controlled_work;
use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (started_tx, mut started_rx) = mpsc::channel::<()>(1);
    let (release_tx, release_rx) = mpsc::channel::<()>(1);
    let handle = tokio::spawn(controlled_work(80, false, started_tx, release_rx));
    let expected_id = handle.id();
    assert_eq!(started_rx.recv().await, Some(()));
    handle.abort();
    match handle.await {
        Err(error) => {
            assert!(error.is_cancelled());
            assert_eq!(error.id(), expected_id);
            println!("已等待取消，仍可對應原 task");
        }
        Ok(_) => panic!("本例 release 尚未放行，工作應被取消"),
    }
    match release_tx.send(()).await {
        Err(_) => println!("task 持有的接收端已清理"),
        Ok(()) => panic!("已等待取消，接收端應已清理"),
    }
}
