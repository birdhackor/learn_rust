fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let byte_count = move || text.as_str().len();

    let first = byte_count();
    let second = byte_count();
    println!("擁有文字的 closure，第一次讀到={first}");
    println!("第二次仍讀到={second}");
    assert_eq!(first, 4);
    assert_eq!(second, 4);
}
