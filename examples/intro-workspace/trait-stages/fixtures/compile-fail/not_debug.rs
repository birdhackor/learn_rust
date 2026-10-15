struct OrderSummary {
    amount: i32,
    count: i32,
}

fn main() {
    let summary = OrderSummary {
        amount: 360,
        count: 3,
    };
    println!("{summary:?}");
}
