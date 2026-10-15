pub type MeterAlias = i32;
pub type SecondAlias = i32;

pub fn alias_distance(value: MeterAlias) -> MeterAlias {
    value
}

pub type PriceFn = Box<dyn Fn(i32) -> i32>;

pub fn price_fn(discount: bool) -> PriceFn {
    if discount {
        Box::new(|value| value - 10)
    } else {
        Box::new(|value| value + 10)
    }
}
