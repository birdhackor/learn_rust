pub fn subtotal(price: i32, count: i32) -> i32 {
    price * count
}

fn shipping(subtotal: i32) -> i32 {
    if subtotal >= 300 { 0 } else { 20 }
}

pub fn total_with_shipping(price: i32, count: i32) -> i32 {
    if count == 0 {
        return 0;
    }
    let subtotal = subtotal(price, count);
    subtotal + shipping(subtotal)
}

pub fn sum_three(values: [i32; 3]) -> i32 {
    let mut total = 0;
    for value in values {
        total += value;
    }
    total
}
