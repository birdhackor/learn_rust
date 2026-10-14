use ch04_lifetimes::source::same_placeholder;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let view = same_placeholder(text.as_str());
    println!("placeholder 推導的借用={view}");
}
