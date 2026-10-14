use ch04_lifetimes::views::TextPair;

fn main() {
    let mut first = String::new();
    first.push_str("Rust tutorial");
    let saved = {
        let mut second = String::new();
        second.push_str("note");
        let pair = TextPair {
            first: first.as_str(),
            second: second.as_str(),
        };
        println!("兩段合計 bytes={}", pair.byte_len());
        println!("第二來源={}", pair.second_source());
        pair.first_source()
    };
    println!("第二來源結束後，第一來源={saved}");
    assert!(saved.len() == 13);
}
