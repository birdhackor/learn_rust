enum Quote {
    Missing,
    Amount(i32),
    Rejected { code: i32 },
}

// --8<-- [start:show]
fn show_quote(quote: Quote) {
    match quote {
        Quote::Missing => println!("目前沒有報價"),
        Quote::Amount(amount) => println!("報價={amount}"),
        Quote::Rejected { code } => println!("拒絕報價，代碼={code}"),
    }
}
// --8<-- [end:show]

fn main() {
    show_quote(Quote::Missing);
    show_quote(Quote::Amount(0));
    show_quote(Quote::Amount(120));
    show_quote(Quote::Rejected { code: 37 });
}
