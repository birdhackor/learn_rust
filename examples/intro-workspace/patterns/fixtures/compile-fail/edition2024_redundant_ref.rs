enum QuoteText {
    Missing,
    Label(String),
}

fn show(quote: &QuoteText) {
    match quote {
        QuoteText::Missing => (),
        QuoteText::Label(ref text) => println!("{}", text.as_str()),
    }
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    show(&QuoteText::Label(label));
    show(&QuoteText::Missing);
}
