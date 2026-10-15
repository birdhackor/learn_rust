enum QuoteText {
    Missing,
    Label(String),
}

fn consume(quote: QuoteText) {
    match quote {
        QuoteText::Missing => println!("沒有文字標籤"),
        QuoteText::Label(text) => println!("接手文字={}", text.as_str()),
    }
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    consume(QuoteText::Label(label));
    consume(QuoteText::Missing);
}
