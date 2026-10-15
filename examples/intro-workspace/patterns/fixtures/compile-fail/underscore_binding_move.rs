enum QuoteText {
    Missing,
    Label(String),
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    let quote = QuoteText::Label(label);
    match quote {
        QuoteText::Label(_text) => println!("已接手文字"),
        QuoteText::Missing => (),
    }
    match &quote {
        QuoteText::Label(text) => println!("{}", text.as_str()),
        QuoteText::Missing => (),
    }
    let _missing = QuoteText::Missing;
}
