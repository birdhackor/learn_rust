fn subtotal(price: i32, count: i32) -> i32 {
    price * count
}

fn show_total(total: i32) {
    println!("總額={total}");
}

fn main() {
    let first = subtotal(120, 3);
    let second = subtotal(80, 2);
    show_total(first);
    show_total(second);
}
