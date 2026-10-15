enum Quote {
    Amount(i32),
    Missing,
}

fn contribution(quote: Quote) -> i32 {
    match quote {
        Quote::Amount(amount) if amount >= 0 => amount,
        Quote::Missing => 0,
    }
}

fn main() {
    println!("{}", contribution(Quote::Amount(120)));
    println!("{}", contribution(Quote::Missing));
}
