fn main() {
    let mut first = String::new();
    first.push_str("Rust");
    let second = first;
    println!("{}", second.as_str());
    println!("{}", first.as_str());
}
