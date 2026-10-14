struct Label<'a> {
    text: &'a str,
}

impl<'a> Label<'a> {
    fn original(&self) -> &'a str {
        self.text
    }
}

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let saved = {
        let label = Label {
            text: text.as_str(),
        };
        label.original()
    };
    println!("label 結束後仍借原文字={saved}");
    assert!(saved.len() == 4);
}
