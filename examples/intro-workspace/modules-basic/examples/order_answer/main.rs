mod display;

use intro_modules_basic::pricing::subtotal;
use intro_modules_basic::pricing::total_with_shipping;

fn main() {
    let goods = subtotal(140, 2);
    let total = total_with_shipping(140, 2);
    display::show_order(goods, total);
}
