enum Quote {
    Missing,
    Amount(i32),
}

fn show(quote: &Quote) {
    let Quote::Amount(amount) = quote else {
        println!("這裡沒有金額，卻還想繼續");
    };
    println!("{}", *amount);
}

fn main() {
    show(&Quote::Amount(120));
    show(&Quote::Missing);
}
