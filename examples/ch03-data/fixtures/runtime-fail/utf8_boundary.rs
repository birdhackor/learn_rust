fn main() {
    let mut text = String::new();
    text.push_str("A中🦀");
    let start = 1;
    let end = 2;
    let partial = &text.as_str()[start..end];
    println!("{partial}");
}
