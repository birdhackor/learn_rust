// --8<-- [start:same]
fn same(text: &str) -> &str {
    text
}
// --8<-- [end:same]

fn main() {
    let mut source = String::new();
    source.push_str("Rust");
    let saved = same(source.as_str());
    println!("借回原文字={saved}");
    assert!(saved.len() == 4);
}
