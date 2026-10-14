use std::convert::AsRef;

fn text_bytes<T: AsRef<str>>(text: &T) -> usize {
    text.as_ref().len()
}

#[derive(PartialEq, Eq, Hash)]
struct Label {
    id: i32,
    text: String,
}

impl AsRef<str> for Label {
    fn as_ref(&self) -> &str {
        self.text.as_str()
    }
}

fn main() {
    let borrowed = "Rust";
    let mut owned = String::new();
    owned.push_str("Rust");
    assert_eq!(text_bytes(&borrowed), 4);
    assert_eq!(text_bytes(&owned), 4);
    println!("&str 與 String 都有 4 bytes");
    println!("String 仍在：{}", owned.as_str());

    let first = Label { id: 1, text: owned };
    let mut second_text = String::new();
    second_text.push_str("Rust");
    let second = Label {
        id: 2,
        text: second_text,
    };
    assert!(first != second);
    assert_eq!(first.as_ref(), second.as_ref());
    assert_eq!(text_bytes(&first), 4);
    println!("Label 身分不同，借出的標籤文字相同");
}
