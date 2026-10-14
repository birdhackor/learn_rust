fn byte_reader(text: &str) -> impl Fn() -> usize {
    move || text.len()
}

fn main() {
    let read = {
        let mut inside = String::new();
        inside.push_str("Rust");
        byte_reader(inside.as_str())
    };
    println!("來源結束後={}", read());
}
