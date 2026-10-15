use ch11_tasks_basic::controlled_work;
use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (started_tx, mut started_rx) = mpsc::channel::<()>(1);
    let (release_tx, release_rx) = mpsc::channel::<()>(1);
    let handle = tokio::spawn(controlled_work(80, false, started_tx, release_rx));
    let id = handle.id();
    assert_eq!(started_rx.recv().await, Some(()));
    assert!(!handle.is_finished());
    println!("收到啟動訊息；還沒有 await JoinHandle");
    release_tx.send(()).await.expect("task 仍持有接收端");
    match handle.await {
        Ok(Ok(value)) => println!("已等待 task {id}，正常回傳金額 {value}"),
        other => panic!("本例應正常回傳金額：{other:?}"),
    }
}
