use ch04_traits::charges::Bundle;
use ch04_traits::charges::Charge;
use ch04_traits::charges::Order;
use ch04_traits::charges::meets_threshold;

fn main() {
    let order = Order {
        unit_price: 120,
        count: 3,
    };
    let bundle = Bundle { price: 160 };
    println!("訂單小計={}，組合價={}", order.amount(), bundle.amount());
    println!("訂單達300={}", meets_threshold(&order, 300));
    println!("組合達300={}", meets_threshold(&bundle, 300));
    println!("訂單仍可讀，數量={}", order.count);
}
