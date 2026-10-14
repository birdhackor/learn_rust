fn shipping(subtotal: i32) -> i32 {
    if subtotal >= 500 {
        0
    } else if subtotal >= 300 {
        10
    } else {
        20
    }
}

fn main() {
    let low = shipping(299);
    let middle = shipping(300);
    let high = shipping(500);
    println!("299的運費={low}，300的運費={middle}，500的運費={high}");
}
