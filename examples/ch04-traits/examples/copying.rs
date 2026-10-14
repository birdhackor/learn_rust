#[derive(Debug, PartialEq, Clone, Copy)]
struct OrderSummary {
    amount: i32,
    count: i32,
}

fn main() {
    let first = OrderSummary {
        amount: 360,
        count: 3,
    };
    let mut second = first;
    second.count += 1;
    println!("原數量={}，另一份數量={}", first.count, second.count);
    assert_eq!(first.amount, 360);
    assert_eq!(first.count, 3);
    assert_eq!(second.amount, 360);
    assert_eq!(second.count, 4);
}
