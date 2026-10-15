pub mod higher_ranked;

use crate::higher_ranked::with_local;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let saved = with_local(|number: &i32| {
        assert_eq!(*number, 80);
        text.as_str()
    });

    assert_eq!(saved, "Rust");
    println!("函式結束後，仍借自外部文字={saved}");
    println!("外部owner仍可讀取，bytes={}", text.len());
}
