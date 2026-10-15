mod charge_support;

use crate::charge_support::Bundle;
use crate::charge_support::Charge;
use crate::charge_support::Order;
use crate::charge_support::meets_threshold;

struct DiscountedBundle {
    original_price: i32,
    discount: i32,
}

impl Charge for DiscountedBundle {
    fn amount(&self) -> i32 {
        self.original_price - self.discount
    }
}

fn main() {
    let bundle = DiscountedBundle {
        original_price: 360,
        discount: 80,
    };
    let order = Order {
        unit_price: 140,
        count: 2,
    };
    let standard = Bundle { price: 280 };
    assert_eq!(order.amount(), bundle.amount());
    assert_eq!(standard.amount(), bundle.amount());
    assert_eq!(bundle.amount(), 280);
    assert!(!meets_threshold(&bundle, 300));
    assert!(meets_threshold(&bundle, 280));
    println!(
        "折扣後={}，達300={}",
        bundle.amount(),
        meets_threshold(&bundle, 300)
    );
}
