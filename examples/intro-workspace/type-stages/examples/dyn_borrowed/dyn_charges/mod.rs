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
// --8<-- [start:amount-interfaces]
pub fn generic_amount<T: Charge>(value: &T) -> i32 {
    value.amount()
}

pub fn dyn_amount(value: &dyn Charge) -> i32 {
    value.amount()
}
// --8<-- [end:amount-interfaces]
