fn fixed_reader(_: &str) -> impl Fn() -> usize {
    || 4
}

fn main() {
    let read = {
        let mut inside = String::new();
        inside.push_str("Rust");
        fixed_reader(inside.as_str())
    };
    println!("來源結束後={}", read());
}
