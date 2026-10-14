use ch01_basics::pricing::subtotal;
use ch01_basics::pricing::total_with_shipping;

fn main() {
    let before_shipping = subtotal(80, 2);
    let after_shipping = total_with_shipping(80, 2);
    println!("商品小計={before_shipping}，含運費={after_shipping}");
}
