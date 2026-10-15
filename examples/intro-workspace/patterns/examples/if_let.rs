enum Quote {
    Missing,
    Amount(i32),
}

fn show_with_match(quote: &Quote) {
    match quote {
        Quote::Amount(amount) => println!("match 讀到金額={}", *amount),
        Quote::Missing => (),
    }
}

fn show_with_if_let(quote: &Quote) {
    if let Quote::Amount(amount) = quote {
        println!("if let 讀到金額={}", *amount);
    }
}

fn show_or_missing(quote: &Quote) {
    if let Quote::Amount(amount) = quote {
        println!("金額={}", *amount);
    } else {
        println!("這筆沒有金額");
    }
}

fn main() {
    let quote = Quote::Amount(120);
    show_with_match(&quote);
    show_with_if_let(&quote);
    show_with_if_let(&Quote::Missing);
    show_or_missing(&Quote::Missing);
}
