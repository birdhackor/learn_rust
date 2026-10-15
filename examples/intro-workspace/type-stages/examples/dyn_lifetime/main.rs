mod dyn_charges;

use crate::dyn_charges::borrow_charge;
use crate::dyn_charges::dyn_amount;

fn main() {
    let prices = vec![80, 120];
    let charge = borrow_charge(prices.as_slice());
    println!("owned Box中的模型借原資料={}", charge.amount());
    println!("借成dyn reference仍讀同一總額={}", dyn_amount(&*charge));
    println!("原資料仍有項目={}", prices.len());
}
