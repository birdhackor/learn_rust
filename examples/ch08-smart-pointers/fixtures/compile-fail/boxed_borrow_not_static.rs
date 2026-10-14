fn require_static<T: 'static>(value: T) -> T {
    value
}

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let boxed = Box::new(text.as_str());
    let saved = require_static(boxed);
    println!("文字={}", *saved);
}
