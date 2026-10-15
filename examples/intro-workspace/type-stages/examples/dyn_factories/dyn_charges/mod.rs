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
// --8<-- [start:charge-factories]
pub fn make_bundle(price: i32) -> Bundle {
    Bundle { price }
}

pub fn make_opaque_bundle(price: i32) -> impl Charge {
    Bundle { price }
}

pub fn make_charge(as_order: bool) -> Box<dyn Charge> {
    if as_order {
        Box::new(Order {
            unit_price: 80,
            count: 1,
        })
    } else {
        Box::new(Bundle { price: 120 })
    }
}
// --8<-- [end:charge-factories]
// --8<-- [start:closure-factory]
pub fn make_adjuster(subtract: bool, base: i32) -> Box<dyn Fn(i32) -> i32> {
    if subtract {
        Box::new(move |value| value - base)
    } else {
        Box::new(move |value| value + base)
    }
}
// --8<-- [end:closure-factory]
