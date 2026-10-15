struct Order {
    unit_price: i32,
    count: i32,
}

// --8<-- [start:consume]
fn subtotal(order: Order) -> i32 {
    order.unit_price * order.count
}
// --8<-- [end:consume]

fn main() {
    let first = Order {
        unit_price: 120,
        count: 3,
    };
    // --8<-- [start:move]
    let second = first;
    println!("second 的數量={}", second.count);
    let total = subtotal(second);
    println!("接手訂單後算出的值={total}");
    // --8<-- [end:move]

    // --8<-- [start:copy]
    let count: i32 = 3;
    let mut another_count = count;
    another_count += 1;
    println!("count={count}，another_count={another_count}");
    // --8<-- [end:copy]
}
