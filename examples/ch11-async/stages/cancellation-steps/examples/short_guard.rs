use std::sync::Mutex;
use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let total = Mutex::new(0);
    let (release_tx, mut release_rx) = mpsc::channel::<()>(1);
    let worker = tokio::spawn(async move {
        let snapshot = {
            let mut guard = total.lock().expect("新建 Mutex 尚未 poison");
            *guard += 80;
            *guard
        };
        assert_eq!(release_rx.recv().await, Some(()));
        snapshot
    });
    release_tx.send(()).await.expect("工作仍持有收端");
    match worker.await {
        Ok(snapshot) => {
            assert_eq!(snapshot, 80);
            println!("等待前 guard 已解鎖；保存的快照={snapshot}");
        }
        Err(_) => panic!("本例工作應正常完成"),
    }
}
