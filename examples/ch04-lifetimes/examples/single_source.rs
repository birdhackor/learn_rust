use ch04_lifetimes::source::same;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let view = same(text.as_str());
    println!("借回的文字={view}，長度={}", view.len());
    assert!(view.len() == 4);
}
