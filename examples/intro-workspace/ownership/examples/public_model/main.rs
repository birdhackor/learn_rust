mod model;

use crate::model::Order;

fn main() {
    let mut order = Order {
        unit_price: 80,
        count: 2,
    };
    order.count += 1;
    println!(
        "跨 module 的數量={}，小計={}",
        order.count,
        order.unit_price * order.count
    );
}
