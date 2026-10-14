fn show(text: &str) {
    println!("借來的文字={text}");
}

fn main() {
    let mut greeting = String::new();
    greeting.push_str("Hello, ");
    greeting.push_str("Rust");
    show(greeting.as_str());

    let ending: &str = "!";
    greeting.push_str(ending);
    show(greeting.as_str());

    let moved = greeting;
    show(moved.as_str());
}
