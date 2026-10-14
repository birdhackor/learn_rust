use std::sync::Mutex;
use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let total = Mutex::new(0);
    let (release_tx, mut release_rx) = mpsc::channel::<()>(1);
    let worker = tokio::spawn(async move {
        let mut guard = total.lock().expect("新建 Mutex 尚未 poison");
        assert_eq!(release_rx.recv().await, Some(()));
        *guard += 80;
        *guard
    });
    release_tx.send(()).await.expect("工作仍持有收端");
    match worker.await {
        Ok(total) => println!("總額={total}"),
        Err(_) => panic!("本例工作應正常完成"),
    }
}
