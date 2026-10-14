enum Quote {
    Missing,
    Amount(i32),
}

fn amount(quote: Quote) -> i32 {
    match quote {
        Quote::Amount(value) => value,
    }
}

fn main() {
    println!("{}", amount(Quote::Amount(120)));
}
