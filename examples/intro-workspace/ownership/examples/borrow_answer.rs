struct Order {
    unit_price: i32,
    count: i32,
}

fn set_count(order: &mut Order, count: i32) {
    order.count = count;
}

fn subtotal(order: &Order) -> i32 {
    order.unit_price * order.count
}

fn main() {
    let mut order = Order {
        unit_price: 80,
        count: 2,
    };
    let before = subtotal(&order);
    set_count(&mut order, 3);
    let after = subtotal(&order);
    assert_eq!(before, 160);
    assert_eq!(after, 240);
    println!("修改前={before}，修改後={after}");
}
