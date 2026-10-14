use ch06_iterators::callbacks::run_once;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let take = move || text;
    let saved = run_once(take);
    println!("共用函式交回文字={}", saved.as_str());
    assert_eq!(run_once(|| 7), 7);
}
