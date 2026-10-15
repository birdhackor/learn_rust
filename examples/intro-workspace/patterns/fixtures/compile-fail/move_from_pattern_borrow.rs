enum QuoteText {
    Missing,
    Label(String),
}

fn take(quote: &QuoteText) {
    match quote {
        QuoteText::Missing => (),
        QuoteText::Label(text) => {
            let owned: String = *text;
            println!("{}", owned.as_str());
        }
    }
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    take(&QuoteText::Label(label));
    take(&QuoteText::Missing);
}
