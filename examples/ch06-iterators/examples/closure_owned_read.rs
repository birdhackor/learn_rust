fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let byte_count = move || text.as_str().len();

    println!("擁有文字的 closure，第一次讀到={}", byte_count());
    println!("第二次仍讀到={}", byte_count());
}
