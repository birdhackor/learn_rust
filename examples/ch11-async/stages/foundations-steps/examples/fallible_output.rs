fn total_now(subtotal: i32) -> i32 {
    subtotal + 20
}

async fn checked_total(subtotal: i32) -> Result<i32, String> {
    if subtotal < 0 {
        Err("小計不能為負數".to_owned())
    } else {
        Ok(total_now(subtotal))
    }
}

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
