#[derive(Debug)]
struct OrderSummary {
    amount: i32,
    count: i32,
}

fn main() {
    let summary = OrderSummary {
        amount: 360,
        count: 3,
    };
    println!("整筆資料={summary:?}");
    assert_eq!(summary.amount, 360);
    assert_eq!(summary.count, 3);
}
