fn new_text<'a>() -> &'a str {
    let mut inside = String::new();
    inside.push_str("Rust");
    inside.as_str()
}

fn main() {
    println!("借回={}", new_text());
}
