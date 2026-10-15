struct Order {
    unit_price: i32,
    count: i32,
}

fn total_with_shipping(order: &Order, shipping: i32) -> i32 {
    order.unit_price * order.count + shipping
}

fn main() {
    let order = Order {
        unit_price: 80,
        count: 2,
    };
    let total = total_with_shipping(&order, 20);
    assert_eq!(total, 180);
    assert_eq!(order.count, 2);
    println!("含運總額={total}，原數量={}", order.count);
}
