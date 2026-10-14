use ch10_concurrency::contracts::require_send;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let work = move || text.as_str().len();
    let work = require_send(work);
    println!("擁有String的工作通過Send，bytes={}", work());
    assert_eq!(work(), 4);
}
