enum QuoteText {
    Missing,
    Label(String),
}

fn show(quote: &QuoteText) {
    match quote {
        QuoteText::Missing => println!("沒有文字標籤"),
        QuoteText::Label(text) => {
            // text 的型別是 &String。
            println!("借讀文字={}", text.as_str());
        }
    }
}

fn append(quote: &mut QuoteText, part: &str) {
    match quote {
        QuoteText::Missing => println!("沒有可追加的標籤"),
        QuoteText::Label(text) => {
            // text 的型別是 &mut String。
            text.push_str(part);
        }
    }
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    let mut quote = QuoteText::Label(label);
    show(&quote);
    show(&quote);
    append(&mut quote, "：已備妥");
    show(&quote);
    show(&QuoteText::Missing);
}
