fn shipping(amount: i32) -> i32 {
    match amount {
        0..=299 => 20,
        300..=499 => 10,
        _ => 0,
    }
}

fn main() {
    for amount in [0, 299, 300, 499, 500] {
        println!("小計{amount}的運費={}", shipping(amount));
    }
    assert_eq!(shipping(299), 20);
    assert_eq!(shipping(300), 10);
    assert_eq!(shipping(499), 10);
    assert_eq!(shipping(500), 0);
}
