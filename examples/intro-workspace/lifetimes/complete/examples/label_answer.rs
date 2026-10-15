struct Label<'a> {
    text: &'a str,
}

impl<'a> Label<'a> {
    fn original(&self) -> &'a str {
        self.text
    }
}

fn main() {
    let mut source = String::new();
    source.push_str("Rust");
    let saved = {
        let label = Label {
            text: source.as_str(),
        };
        label.original()
    };
    assert_eq!(saved, "Rust");
    println!("Label 結束後，仍借讀原來源={saved}");
}
