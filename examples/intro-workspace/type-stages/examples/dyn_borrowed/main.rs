mod dyn_charges;

use crate::dyn_charges::Bundle;
use crate::dyn_charges::Order;
use crate::dyn_charges::dyn_amount;
use crate::dyn_charges::generic_amount;

fn main() {
    let order = Order {
        unit_price: 80,
        count: 1,
    };
    let bundle = Bundle { price: 120 };
    println!("泛型讀訂單={}", generic_amount(&order));
    println!("dyn借用訂單={}", dyn_amount(&order));
    println!("dyn借用組合價={}", dyn_amount(&bundle));
    println!("原訂單仍有數量={}", order.count);
}
