// --8<-- [start:library]
pub fn order_subtotal(price: i32, count: i32) -> i32 {
    ch01_basics::pricing::subtotal(price, count)
}

#[cfg(feature = "batch")]
pub fn two_orders(first: [i32; 2], second: [i32; 2]) -> i32 {
    order_subtotal(first[0], first[1]) + order_subtotal(second[0], second[1])
}
// --8<-- [end:library]
