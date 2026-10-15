fn subtotal(price: i32, count: i32) -> i32 {
    price * count
}

fn with_shipping(price: i32, count: i32, shipping: i32) -> i32 {
    let item_total = subtotal(price, count);
    item_total + shipping
}

fn main() {
    let first = with_shipping(120, 3, 20);
    let second = with_shipping(80, 2, 20);
    println!("第一筆={first}");
    println!("第二筆={second}");
}
