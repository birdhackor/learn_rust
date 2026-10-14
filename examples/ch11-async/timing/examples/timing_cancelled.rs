use std::future::pending;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut visits = 0;
    let work = async {
        visits += 1;
        pending::<()>().await;
        visits += 100;
    };

    match timeout(Duration::from_millis(20), work).await {
        Ok(()) => panic!("pending不會完成"),
        Err(_) => println!("工作已逾時並結束"),
    }
    assert_eq!(visits, 1);
    println!("取消後保留的開始次數={visits}");
}
