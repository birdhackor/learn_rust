mod views;

use crate::views::TextView;

fn main() {
    let mut source = String::new();
    source.push_str("Rust tutorial");
    // --8<-- [start:receiver-and-source]
    let saved = {
        let view = TextView {
            text: source.as_str(),
        };
        println!("借讀模型時的文字={}", view.text());
        assert!(view.byte_len() == 13);
        view.original()
    };
    println!("模型結束後，原來源仍有效={saved}");
    // --8<-- [end:receiver-and-source]
    assert_eq!(saved, "Rust tutorial");
}
