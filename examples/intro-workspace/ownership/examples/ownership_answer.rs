struct Order {
    unit_price: i32,
    count: i32,
}

fn add_one(mut order: Order) -> Order {
    order.count += 1;
    order
}

fn main() {
    let order = Order {
        unit_price: 80,
        count: 2,
    };
    let updated = add_one(order);
    assert_eq!(updated.count, 3);
    assert_eq!(updated.unit_price * updated.count, 240);
    println!("交回的數量={}，小計=240", updated.count);
}
