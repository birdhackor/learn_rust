use tokio::sync::mpsc;

#[derive(Debug, PartialEq)]
pub enum WorkError {
    Negative(i32),
    ControlClosed,
}

pub async fn controlled_work(
    amount: i32,
    panic_after_release: bool,
    started: mpsc::Sender<()>,
    mut release: mpsc::Receiver<()>,
) -> Result<i32, WorkError> {
    started
        .send(())
        .await
        .expect("本例 controller 持有啟動訊息接收端");
    drop(started);
    match release.recv().await {
        Some(()) => (),
        None => return Err(WorkError::ControlClosed),
    }
    if panic_after_release {
        panic!("task 故意 panic");
    }
    if amount < 0 {
        Err(WorkError::Negative(amount))
    } else {
        Ok(amount)
    }
}
