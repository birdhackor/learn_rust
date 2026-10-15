struct TextView<'a> {
    text: &'a str,
}

impl<'a> TextView<'a> {
    fn text(&self) -> &str {
        self.text
    }
}

fn main() {
    let mut source = String::new();
    source.push_str("Rust tutorial");
    let saved = {
        let view = TextView {
            text: source.as_str(),
        };
        view.text()
    };
    println!("{saved}");
}
