use ch04_lifetimes::views::TextView;

fn main() {
    let mut text = String::new();
    text.push_str("Rust tutorial");
    let saved = {
        let view = TextView {
            text: text.as_str(),
        };
        println!("借用 view 時讀取={}", view.text());
        view.original()
    };
    println!("view 結束後，來源仍存在={saved}");
    assert!(saved.len() == 13);
}
