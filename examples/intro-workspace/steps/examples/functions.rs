// --8<-- [start:subtotal]
fn subtotal(price: i32, count: i32) -> i32 {
    price * count
}
// --8<-- [end:subtotal]

// --8<-- [start:show]
fn show_total(total: i32) {
    println!("總額={total}");
}
// --8<-- [end:show]

// --8<-- [start:calls]
fn main() {
    let first = subtotal(120, 3);
    let second = subtotal(80, 2);
    show_total(first);
    show_total(second);
}
// --8<-- [end:calls]
