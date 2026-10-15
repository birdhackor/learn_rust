enum Quote {
    Amount(i32),
    Estimate(i32),
    Missing,
}

fn amount_or_zero(quote: Quote) -> i32 {
    match quote {
        Quote::Amount(value) | Quote::Estimate(value) => value,
        Quote::Missing => 0,
    }
}

fn main() {
    let confirmed = amount_or_zero(Quote::Amount(120));
    let estimated = amount_or_zero(Quote::Estimate(80));
    let missing = amount_or_zero(Quote::Missing);
    println!("確認金額={confirmed}，預估金額={estimated}，缺值貢獻={missing}");
    assert_eq!(confirmed, 120);
    assert_eq!(estimated, 80);
    assert_eq!(missing, 0);
}
