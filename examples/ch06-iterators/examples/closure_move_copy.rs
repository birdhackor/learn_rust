fn main() {
    let limit = 300;
    let same_limit = move || limit;
    println!("外面的數字={limit}，closure 的數字={}", same_limit());

    let mut text = String::new();
    text.push_str("Rust");
    let view = text.as_str();
    let byte_count = move || view.len();
    println!("外面的 reference={view}，closure 讀到={}", byte_count());
    assert_eq!(text.as_str().len(), 4);
}
