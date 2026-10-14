fn main() {
    let saved: &'static str = {
        let mut inside = String::new();
        inside.push_str("Rust");
        inside.as_str()
    };
    println!("文字={saved}");
}
