struct TextView<'a> {
    text: &'a str,
}

impl<'a> TextView<'a> {
    fn text(&self) -> &str {
        self.text
    }
}

fn main() {
    let mut text = String::new();
    text.push_str("Rust tutorial");
    let saved = {
        let view = TextView {
            text: text.as_str(),
        };
        view.text()
    };
    println!("view 結束後={saved}");
}
