use ch11_foundations::total_later;

async fn receipt(subtotal: i32, label: String) -> String {
    let total = total_later(subtotal).await;
    format!("{label}：總額 {total}")
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let label = "訂單 A".to_owned();
    let work = receipt(80, label);
    println!("收據 future 已保存小計與標籤");
    let text = work.await;
    assert_eq!(text.as_str(), "訂單 A：總額 100");
    println!("{}", text.as_str());
}
