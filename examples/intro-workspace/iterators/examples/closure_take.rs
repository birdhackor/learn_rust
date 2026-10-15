fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let take = move || text;
    let mut saved = take();
    saved.push_str(" 教學");
    println!("呼叫後交回的 owned 文字={}", saved.as_str());
    assert_eq!(saved.as_str().len(), 11);
}
