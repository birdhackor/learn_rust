use std::future::pending;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let limit = Duration::from_millis(20);
    let ready = async { 100 };
    match timeout(limit, ready).await {
        Ok(value) => println!("立即完成：{value}"),
        Err(_) => panic!("本版timeout先讓即時工作取得執行機會"),
    }

    let unavailable = pending::<i32>();
    match timeout(limit, unavailable).await {
        Ok(_) => panic!("pending不會交出結果"),
        Err(_) => println!("未完成工作已逾時"),
    }
}
