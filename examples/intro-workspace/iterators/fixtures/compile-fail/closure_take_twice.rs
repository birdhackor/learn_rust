fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let take = move || text;
    let first = take();
    let second = take();
    println!("{}、{}", first.as_str(), second.as_str());
}
