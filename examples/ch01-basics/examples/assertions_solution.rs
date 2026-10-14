use ch01_basics::pricing::subtotal;
use ch01_basics::pricing::total_with_shipping;

fn main() {
    let goods = subtotal(140, 2);
    let total = total_with_shipping(140, 2);
    // --8<-- [start:answer-checks]
    assert_eq!(goods, 280);
    assert_eq!(total, 300);
    // --8<-- [end:answer-checks]
    println!("商品小計={goods}，含運費={total}");
}
