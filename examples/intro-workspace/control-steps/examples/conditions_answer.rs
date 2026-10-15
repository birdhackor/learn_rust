// --8<-- [start:tiers]
fn shipping(subtotal: i32) -> i32 {
    if subtotal >= 500 {
        0
    } else if subtotal >= 300 {
        10
    } else {
        20
    }
}
// --8<-- [end:tiers]

fn main() {
    let low = shipping(299);
    let middle = shipping(300);
    let below_high = shipping(499);
    let high = shipping(500);
    println!("299的運費={low}，300的運費={middle}");
    println!("499的運費={below_high}，500的運費={high}");
}
