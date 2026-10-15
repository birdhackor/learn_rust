mod chain_support;

use crate::chain_support::Chain;
use crate::chain_support::into_total;

fn main() {
    let chain = Chain::Link {
        value: 40,
        next: Box::new(Chain::Link {
            value: 80,
            next: Box::new(Chain::Link {
                value: 120,
                next: Box::new(Chain::End),
            }),
        }),
    };
    let total = into_total(chain);
    assert_eq!(total, 240);
    println!("三個連結的總和={total}");
}
