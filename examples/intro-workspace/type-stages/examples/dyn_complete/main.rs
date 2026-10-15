pub mod dyn_charges;

use crate::dyn_charges::Bundle;
use crate::dyn_charges::Charge;
use crate::dyn_charges::Order;
use crate::dyn_charges::total;

struct ExtraFee {
    amount: i32,
}

impl Charge for ExtraFee {
    fn amount(&self) -> i32 {
        self.amount
    }
}

fn main() {
    let values: Vec<Box<dyn Charge>> = vec![
        Box::new(Order {
            unit_price: 80,
            count: 1,
        }),
        Box::new(Bundle { price: 120 }),
        Box::new(ExtraFee { amount: 40 }),
    ];
    assert_eq!(total(values.as_slice()), 240);
    println!("新的費用種類一起加總={}", total(values.as_slice()));
}
