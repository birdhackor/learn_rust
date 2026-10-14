fn require_send_static<T: Send + 'static>(value: T) -> T {
    value
}

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let view = require_send_static(text.as_str());
    println!("{view}");
}
