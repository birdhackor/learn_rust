fn longer(first: &str, second: &str) -> &str {
    if first.len() >= second.len() { first } else { second }
}

fn main() {
    println!("選到={}", longer("Rust", "Python"));
}
