mod chain_support;

use crate::chain_support::Chain;
use crate::chain_support::into_total;

fn main() {
    let chain = Chain::Link {
        value: 80,
        next: Box::new(Chain::Link {
            value: 120,
            next: Box::new(Chain::End),
        }),
    };
    let total = into_total(chain);
    let empty_total = into_total(Chain::End);
    assert_eq!(total, 200);
    assert_eq!(empty_total, 0);
    println!("沿連結加總={total}");
    println!("空鏈加總={empty_total}");
}
