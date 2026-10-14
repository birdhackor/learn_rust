struct Order {
    unit_price: i32,
    count: i32,
}

fn subtotal(order: &Order) -> i32 {
    order.unit_price * order.count
}

fn main() {
    let order = Order {
        unit_price: 120,
        count: 3,
    };
    let reference: &Order = &order;
    let total = subtotal(reference);
    println!("借用算出的小計={total}，原數量={}", order.count);

    let count: i32 = 3;
    let count_reference: &i32 = &count;
    let copied_count = *count_reference;
    println!("讀出的數字={copied_count}，原數字={count}");
}
