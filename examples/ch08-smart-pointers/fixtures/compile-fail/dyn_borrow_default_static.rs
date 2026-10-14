trait Charge {
    fn amount(&self) -> i32;
}

struct BorrowedCharge<'a> {
    prices: &'a [i32],
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

fn borrow_charge(prices: &[i32]) -> Box<dyn Charge> {
    Box::new(BorrowedCharge { prices })
}

fn main() {
    let prices = [80, 120];
    println!("金額={}", borrow_charge(&prices[..]).amount());
}
