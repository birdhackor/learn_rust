// --8<-- [start:copy-text]
fn copy_text(text: &str) -> String {
    let mut copied = String::new();
    copied.push_str(text);
    copied
}
// --8<-- [end:copy-text]

fn main() {
    // --8<-- [start:source-ends]
    let saved = {
        let mut inside = String::new();
        inside.push_str("Rust");
        copy_text(inside.as_str())
    };
    println!("來源結束後，owned 結果={}", saved.as_str());
    assert!(saved.as_str().len() == 4);
    // --8<-- [end:source-ends]

    let mut original = String::new();
    original.push_str("Rust");
    let copied = copy_text(original.as_str());
    original.push_str(" tutorial");
    println!("原資料追加後={}", original.as_str());
    println!("獨立結果仍是={}", copied.as_str());
    assert!(original.as_str().len() == 13);
    assert!(copied.as_str().len() == 4);
}
