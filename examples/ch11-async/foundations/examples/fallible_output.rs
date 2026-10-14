use ch11_foundations::checked_total;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    for subtotal in [-10, 80] {
        let work = checked_total(subtotal);
        let result = work.await;
        match result {
            Ok(total) => println!("小計 {subtotal} 的工作完成，結果為 Ok({total})"),
            Err(message) => println!("小計 {subtotal} 的工作完成，結果為 Err：{message}"),
        }
    }
}
