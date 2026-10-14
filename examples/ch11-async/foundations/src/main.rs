use ch11_foundations::total_later;
use ch11_foundations::total_now;

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
