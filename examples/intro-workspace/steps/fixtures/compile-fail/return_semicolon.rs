fn subtotal(price: i32, count: i32) -> i32 {
    price * count;
}

fn main() {
    let total = subtotal(120, 3);
    println!("總額={total}");
}
