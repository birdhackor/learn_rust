use ch04_traits::charges::Charge;
use ch04_traits::charges::meets_threshold;

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
    assert_eq!(bundle.amount(), 280);
    assert!(!meets_threshold(&bundle, 300));
    assert!(meets_threshold(&bundle, 280));
    println!(
        "折扣後={}，達300={}",
        bundle.amount(),
        meets_threshold(&bundle, 300)
    );
}
