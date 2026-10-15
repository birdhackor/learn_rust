fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let view = text.as_str();
    text.push_str("!");
    println!("{view}");
}
