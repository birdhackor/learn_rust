enum QuoteText {
    Missing,
    Label(String),
}

fn show(quote: &QuoteText) {
    match quote {
        QuoteText::Missing => println!("沒有文字標籤"),
        QuoteText::Label(text) => println!("原值仍可借讀={}", text.as_str()),
    }
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    let quote = QuoteText::Label(label);
    match quote {
        QuoteText::Label(_) => println!("只確認形態，不取出文字"),
        QuoteText::Missing => println!("缺少資料"),
    }
    show(&quote);
    show(&QuoteText::Missing);
}
