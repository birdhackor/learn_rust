enum QuoteText {
    Missing,
    Label(String),
}

fn append(quote: &mut QuoteText, part: &str) {
    match quote {
        QuoteText::Label(text) => text.push_str(part),
        QuoteText::Missing => (),
    }
}

fn show(quote: &QuoteText) {
    match quote {
        QuoteText::Label(text) => println!("標籤={}", text.as_str()),
        QuoteText::Missing => println!("沒有標籤"),
    }
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單B");
    let mut quote = QuoteText::Label(label);
    append(&mut quote, "：完成");
    show(&quote);
    show(&quote);
    show(&QuoteText::Missing);
}
