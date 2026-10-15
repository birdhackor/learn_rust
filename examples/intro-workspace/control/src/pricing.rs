// --8<-- [start:subtotal]
pub fn subtotal(price: i32, count: i32) -> i32 {
    price * count
}
// --8<-- [end:subtotal]

// --8<-- [start:shipping]
fn shipping(subtotal: i32) -> i32 {
    if subtotal >= 300 { 0 } else { 20 }
}
// --8<-- [end:shipping]

// --8<-- [start:total]
pub fn total_with_shipping(price: i32, count: i32) -> i32 {
    if count == 0 {
        return 0;
    }
    let item_total = subtotal(price, count);
    item_total + shipping(item_total)
}
// --8<-- [end:total]

// --8<-- [start:sum]
pub fn sum_three(values: [i32; 3]) -> i32 {
    let mut total = 0;
    for value in values {
        total += value;
    }
    total
}
// --8<-- [end:sum]
