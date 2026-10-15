pub mod variance;

use crate::variance::shorten_mutable;
use crate::variance::shorten_shared;

fn main() {
    let mut local = String::new();
    local.push_str("note");
    let selected: &str = shorten_shared("Rust");
    println!(
        "長資料可交給一般借用={selected}，局部文字={}",
        local.as_str()
    );

    let mut amount = 80;
    {
        let temporary = shorten_mutable(&mut amount);
        *temporary += 20;
        println!("短暫mutable借用更新={temporary}");
    }
    println!("借用結束後原值={amount}");
    assert_eq!(amount, 100);
}
