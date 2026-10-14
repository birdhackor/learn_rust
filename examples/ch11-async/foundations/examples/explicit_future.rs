use ch11_foundations::total_now;
use std::future::Future;

fn prepared_total(subtotal: i32) -> impl Future<Output = i32> {
    println!("普通工廠：先保存小計 {subtotal}");
    async move { total_now(subtotal) }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let work = prepared_total(80);
    let total = work.await;
    assert_eq!(total, 100);
    println!("Future 的 Output 是 i32：{total}");
}
