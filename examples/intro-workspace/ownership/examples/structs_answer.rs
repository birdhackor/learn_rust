struct Order {
    unit_price: i32,
    count: i32,
}

fn main() {
    let mut order = Order {
        unit_price: 80,
        count: 2,
    };
    let before = order.unit_price * order.count;
    order.count += 1;
    let after = order.unit_price * order.count;
    assert_eq!(before, 160);
    assert_eq!(after, 240);
    println!("原小計={before}，加一件後={after}");
}
