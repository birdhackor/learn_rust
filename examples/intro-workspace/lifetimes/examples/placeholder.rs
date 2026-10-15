// --8<-- [start:placeholder]
fn same_placeholder(text: &'_ str) -> &'_ str {
    text
}
// --8<-- [end:placeholder]

fn main() {
    let mut source = String::new();
    source.push_str("Rust");
    let saved = same_placeholder(source.as_str());
    println!("推導後仍借原文字={saved}");
    assert!(saved.len() == 4);
}
