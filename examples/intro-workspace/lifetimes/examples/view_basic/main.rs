mod views;

use crate::views::TextView;
use crate::views::make_view;

fn main() {
    let mut source = String::new();
    source.push_str("Rust tutorial");

    // --8<-- [start:build]
    let view = TextView {
        text: source.as_str(),
    };
    let built = make_view(source.as_str());
    println!("直接建立，欄位文字={}", view.text);
    println!("函式建立，欄位文字={}", built.text);
    println!("方法查到的 bytes={}", built.byte_len());
    // --8<-- [end:build]
    assert!(view.byte_len() == 13);
    assert!(built.byte_len() == 13);
    println!("owner 仍持有文字={}", source.as_str());
}
