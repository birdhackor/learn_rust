use ch04_lifetimes::source::copy_text;

fn main() {
    let saved = {
        let mut inside = String::new();
        inside.push_str("Rust");
        copy_text(inside.as_str())
    };
    println!("來源結束後，owned 結果={}", saved.as_str());
    assert!(saved.as_str().len() == 4);
}
