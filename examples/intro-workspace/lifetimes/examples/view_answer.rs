struct Label<'a> {
    text: &'a str,
}

impl<'a> Label<'a> {
    fn byte_len(&self) -> usize {
        self.text.len()
    }
}

fn main() {
    let saved_length = {
        let mut source = String::new();
        source.push_str("Rust");
        let label = Label {
            text: source.as_str(),
        };
        println!("內層借讀標籤={}", label.text);
        label.byte_len()
    };
    assert!(saved_length == 4);
    println!("來源與模型結束後，留下 bytes={saved_length}");
}
