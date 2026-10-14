use std::cell::Cell;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let value = Cell::new(text);
    let copied = value.get();
    println!("文字={}", copied.as_str());
}
