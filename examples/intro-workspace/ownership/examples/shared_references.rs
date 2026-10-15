struct Order {
    unit_price: i32,
    count: i32,
}

// --8<-- [start:borrow-function]
fn subtotal(order: &Order) -> i32 {
    order.unit_price * order.count
}
// --8<-- [end:borrow-function]

fn main() {
    let order = Order {
        unit_price: 120,
        count: 3,
    };
    // --8<-- [start:borrow-call]
    let reference: &Order = &order;
    let total = subtotal(reference);
    println!("借用算出的小計={total}，原數量={}", order.count);
    // --8<-- [end:borrow-call]

    // --8<-- [start:dereference]
    let count: i32 = 3;
    let count_reference: &i32 = &count;
    let copied_count = *count_reference;
    println!("讀出的數字={copied_count}，原數字={count}");
    // --8<-- [end:dereference]
}
