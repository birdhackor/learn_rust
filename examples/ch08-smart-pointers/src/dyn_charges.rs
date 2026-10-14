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

// --8<-- [start:borrowed-charge]
pub struct BorrowedCharge<'a> {
    pub prices: &'a [i32],
}

impl Charge for BorrowedCharge<'_> {
    fn amount(&self) -> i32 {
        let mut sum = 0;
        let mut index = 0;
        while index < self.prices.len() {
            sum += self.prices[index];
            index += 1;
        }
        sum
    }
}

pub fn borrow_charge(prices: &[i32]) -> Box<dyn Charge + '_> {
    Box::new(BorrowedCharge { prices })
}
// --8<-- [end:borrowed-charge]

// --8<-- [start:rebuild]
pub trait RebuildCharge: Charge {
    fn rebuild(&self) -> Self
    where
        Self: Sized;
}

impl RebuildCharge for Bundle {
    fn rebuild(&self) -> Self {
        Bundle { price: self.price }
    }
}
// --8<-- [end:rebuild]

// --8<-- [start:measure]
pub trait Measure {
    type Output;

    fn measure(&self) -> Self::Output;
}

impl Measure for Bundle {
    type Output = i32;

    fn measure(&self) -> Self::Output {
        self.price
    }
}
// --8<-- [end:measure]

#[cfg(test)]
mod tests {
    use crate::dyn_charges::Bundle;
    use crate::dyn_charges::Charge;
    use crate::dyn_charges::Measure;
    use crate::dyn_charges::Order;
    use crate::dyn_charges::borrow_charge;
    use crate::dyn_charges::make_adjuster;
    use crate::dyn_charges::total;

    struct Shipping {
        fee: i32,
    }

    impl Charge for Shipping {
        fn amount(&self) -> i32 {
            self.fee
        }
    }

    #[test]
    fn an_external_model_joins_the_same_heterogeneous_list() {
        let empty: Vec<Box<dyn Charge>> = Vec::new();
        assert_eq!(total(empty.as_slice()), 0);
        let values: Vec<Box<dyn Charge>> = vec![
            Box::new(Order {
                unit_price: 80,
                count: 1,
            }),
            Box::new(Bundle { price: 120 }),
            Box::new(Shipping { fee: 20 }),
        ];
        assert_eq!(total(values.as_slice()), 220);
    }

    #[test]
    fn a_borrowed_object_reads_the_original_prices() {
        let prices = vec![80, 120];
        let charge = borrow_charge(prices.as_slice());
        assert_eq!(charge.amount(), 200);
        assert_eq!(prices.as_slice()[0], 80);
        assert_eq!(prices.as_slice()[1], 120);
    }

    #[test]
    fn different_closure_types_supply_the_same_dyn_call_interface() {
        let subtract = make_adjuster(true, 10);
        let increase = make_adjuster(false, 10);
        assert_eq!(subtract(30), 20);
        assert_eq!(increase(30), 40);
    }

    #[test]
    fn an_associated_output_is_fixed_by_the_object_type() {
        let value: Box<dyn Measure<Output = i32>> = Box::new(Bundle { price: 120 });
        assert_eq!(value.measure(), 120);
    }
}
