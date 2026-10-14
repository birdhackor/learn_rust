fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let byte_count = || text.as_str().len();

    println!("呼叫前，原文字={}", text.as_str());
    println!("closure 讀到 bytes={}", byte_count());
    println!("再呼叫仍是 bytes={}", byte_count());
    assert_eq!(text.as_str().len(), 4);
}
