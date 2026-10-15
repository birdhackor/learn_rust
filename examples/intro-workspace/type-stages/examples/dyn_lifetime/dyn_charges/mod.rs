pub trait Charge {
    fn amount(&self) -> i32;
}

pub fn dyn_amount(value: &dyn Charge) -> i32 {
    value.amount()
}

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
