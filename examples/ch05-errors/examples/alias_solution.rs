type PriceAndCount = (i32, i32);

fn subtotal(order: PriceAndCount) -> i32 {
    order.0 * order.1
}

fn main() {
    let named: PriceAndCount = (80, 3);
    let ordinary: (i32, i32) = named;
    let total = subtotal(ordinary);
    assert_eq!(total, 240);
    println!("使用另一個名稱後，小計={total}");
}
