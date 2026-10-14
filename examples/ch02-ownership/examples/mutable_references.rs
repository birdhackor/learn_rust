struct Order {
    unit_price: i32,
    count: i32,
}

fn add_one(order: &mut Order) {
    order.count += 1;
}

fn main() {
    let mut order = Order {
        unit_price: 120,
        count: 3,
    };
    add_one(&mut order);
    println!("原訂單改成數量={}，單價={}", order.count, order.unit_price);

    let mut count: i32 = 3;
    let reference: &mut i32 = &mut count;
    *reference += 1;
    println!("透過 mutable reference 得到的數字={}", *reference);
}
