// --8<-- [start:charge-model]
pub struct Order {
    pub unit_price: i32,
    pub count: i32,
}

pub struct Bundle {
    pub price: i32,
}
// --8<-- [end:charge-model]

// --8<-- [start:charge-trait]
pub trait Charge {
    fn amount(&self) -> i32;
}
// --8<-- [end:charge-trait]

// --8<-- [start:charge-implementations]
impl Charge for Order {
    fn amount(&self) -> i32 {
        self.unit_price * self.count
    }
}

impl Charge for Bundle {
    fn amount(&self) -> i32 {
        self.price
    }
}
// --8<-- [end:charge-implementations]

// --8<-- [start:charge-bound]
pub fn meets_threshold<T: Charge>(value: &T, threshold: i32) -> bool {
    value.amount() >= threshold
}
// --8<-- [end:charge-bound]

// --8<-- [start:charge-tests]
#[cfg(test)]
mod tests {
    use crate::charge_support::Bundle;
    use crate::charge_support::Charge;
    use crate::charge_support::Order;
    use crate::charge_support::meets_threshold;

    #[test]
    fn both_models_supply_the_same_amount_contract() {
        let order = Order {
            unit_price: 120,
            count: 3,
        };
        let bundle = Bundle { price: 160 };
        assert_eq!(order.amount(), 360);
        assert_eq!(bundle.amount(), 160);
        assert!(meets_threshold(&order, 300));
        assert!(!meets_threshold(&bundle, 300));
    }

    #[test]
    fn threshold_includes_the_boundary_for_both_models() {
        let order = Order {
            unit_price: 100,
            count: 3,
        };
        let bundle = Bundle { price: 300 };
        assert!(meets_threshold(&order, 300));
        assert!(meets_threshold(&bundle, 300));
        assert!(!meets_threshold(&order, 301));
        assert!(!meets_threshold(&bundle, 301));
    }
}
// --8<-- [end:charge-tests]
