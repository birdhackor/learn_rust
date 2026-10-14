use ch04_lifetimes::source::first_nonempty;

fn main() {
    let empty = String::new();
    let mut rust = String::new();
    rust.push_str("Rust");
    let mut python = String::new();
    python.push_str("Python");
    println!("第一段空={}", first_nonempty(empty.as_str(), rust.as_str()));
    println!(
        "第一段非空={}",
        first_nonempty(rust.as_str(), python.as_str())
    );
    assert!(first_nonempty(empty.as_str(), rust.as_str()).len() == 4);
    assert!(first_nonempty(rust.as_str(), python.as_str()).len() == 4);
    assert_eq!(first_nonempty(empty.as_str(), empty.as_str()).len(), 0);
}
