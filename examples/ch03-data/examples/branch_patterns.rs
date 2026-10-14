enum Quote {
    Missing,
    Amount(i32),
}

fn show_amount(quote: &Quote) {
    let Quote::Amount(amount) = quote else {
        println!("略過沒有報價的資料");
        return;
    };
    println!("報價加運費={}", *amount + 20);
}

fn main() {
    let quote = Quote::Amount(120);
    if let Quote::Amount(amount) = &quote {
        println!("只關心報價={}", *amount);
    }
    show_amount(&quote);
    show_amount(&Quote::Missing);
}
