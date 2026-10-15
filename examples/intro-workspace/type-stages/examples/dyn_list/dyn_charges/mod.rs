// --8<-- [start:models]
pub struct Order {
    pub unit_price: i32,
    pub count: i32,
}

pub struct Bundle {
    pub price: i32,
}

pub trait Charge {
    fn amount(&self) -> i32;
}

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
// --8<-- [end:models]
// --8<-- [start:known-charges]
pub enum KnownCharge {
    Order(Order),
    Bundle(Bundle),
}

impl Charge for KnownCharge {
    fn amount(&self) -> i32 {
        match self {
            KnownCharge::Order(order) => order.amount(),
            KnownCharge::Bundle(bundle) => bundle.amount(),
        }
    }
}
// --8<-- [end:known-charges]
// --8<-- [start:dyn-total]
pub fn total(values: &[Box<dyn Charge>]) -> i32 {
    let mut sum = 0;
    let mut index = 0;
    while index < values.len() {
        sum += values[index].amount();
        index += 1;
    }
    sum
}
// --8<-- [end:dyn-total]
