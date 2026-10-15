enum Quote {
    Amount(i32),
    Missing,
}

fn contribution(quote: Quote) -> i32 {
    match quote {
        Quote::Amount(value) | Quote::Missing => value,
    }
}

fn main() {
    println!("{}", contribution(Quote::Amount(120)));
    println!("{}", contribution(Quote::Missing));
}
