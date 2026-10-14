fn twice<F: FnMut() -> String>(mut task: F) -> (String, String) {
    (task(), task())
}

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let take = move || text;
    let (first, second) = twice(take);
    println!("{}、{}", first.as_str(), second.as_str());
}
