enum Quote {
    Missing,
    Amount(i32),
}

fn show(quote: Quote) {
    let Quote::Amount(amount) = quote;
    println!("報價={amount}");
}

fn main() {
    show(Quote::Amount(120));
    show(Quote::Missing);
}
