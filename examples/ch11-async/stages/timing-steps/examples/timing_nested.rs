use ch11_foundations::checked_total;
use std::time::Duration;
use tokio::time::error::Elapsed;
use tokio::time::timeout;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    for subtotal in [-10, 80] {
        let work = checked_total(subtotal);
        let result: Result<Result<i32, String>, Elapsed> =
            timeout(Duration::from_millis(20), work).await;
        match result {
            Ok(Ok(total)) => println!("小計 {subtotal}：工作已完成，總額={total}"),
            Ok(Err(message)) => {
                println!("小計 {subtotal}：工作已完成，資料失敗：{message}");
            }
            Err(error) => println!("小計 {subtotal}：工作尚未完成，等待逾時：{error}"),
        }
    }
}
