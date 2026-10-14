enum Quote {
    Missing,
    Amount(i32),
    Rejected { code: i32 },
}

fn show_quote(quote: Quote) {
    match quote {
        Quote::Missing => println!("目前沒有報價"),
        Quote::Amount(amount) => println!("報價={amount}"),
        Quote::Rejected { code } => println!("拒絕報價，代碼={code}"),
    }
}

fn main() {
    show_quote(Quote::Missing);
    show_quote(Quote::Amount(120));
    show_quote(Quote::Rejected { code: 37 });
}
