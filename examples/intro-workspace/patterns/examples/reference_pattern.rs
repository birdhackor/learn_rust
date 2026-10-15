enum QuoteText {
    Missing,
    Label(String),
}

// 刻意並列 & pattern 與 ref binding，保留完整形式供本節比較。
#[allow(clippy::match_ref_pats)]
#[allow(clippy::needless_borrowed_reference)]
fn show(quote: &QuoteText) {
    match quote {
        &QuoteText::Missing => println!("沒有文字標籤"),
        &QuoteText::Label(ref text) => println!("明寫 reference pattern={}", text.as_str()),
    }
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    let quote = QuoteText::Label(label);
    show(&quote);
    show(&quote);
    show(&QuoteText::Missing);
}
