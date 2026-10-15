fn longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

// --8<-- [start:first-text]
fn first_text<'a>(first: &'a str, _: &str) -> &'a str {
    first
}
// --8<-- [end:first-text]

fn main() {
    let mut outside = String::new();
    outside.push_str("Rust tutorial");
    {
        let mut inside = String::new();
        inside.push_str("hi");
        let selected = longer(outside.as_str(), inside.as_str());
        println!("兩來源契約，內層使用={selected}");
    }

    // --8<-- [start:save-first]
    let saved = {
        let mut inside = String::new();
        inside.push_str("hi");
        first_text(outside.as_str(), inside.as_str())
    };
    println!("只接第一來源，外層使用={saved}");
    // --8<-- [end:save-first]
    assert!(saved.len() == 13);
}
