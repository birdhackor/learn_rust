fn total_now(subtotal: i32) -> i32 {
    subtotal + 20
}

async fn total_later(subtotal: i32) -> i32 {
    println!("async 本體：處理小計 {subtotal}");
    total_now(subtotal)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let now = total_now(80);
    println!("普通函式已交回：{now}");

    let work = total_later(80);
    println!("future 已建立，async 本體尚未執行");
    let total = work.await;
    println!("await 已取得結果：{total}");

    let unused_work = total_later(120);
    drop(unused_work);
    println!("另一份 future 未執行就已丟棄");
}
