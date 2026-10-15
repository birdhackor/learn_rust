enum Quote {
    Amount(i32),
    Missing,
}

fn show_binding(quote: Quote, limit: i32) {
    match quote {
        Quote::Amount(limit) => println!("分支內新 binding 的 limit={limit}"),
        Quote::Missing => println!("沒有報價"),
    }
    println!("分支外原本的 limit={limit}");
}

fn main() {
    show_binding(Quote::Amount(50), 100);
    show_binding(Quote::Missing, 100);
}
