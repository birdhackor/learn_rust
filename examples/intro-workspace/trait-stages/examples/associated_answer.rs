mod associated_support;

use crate::associated_support::Extract;
use crate::associated_support::Holder;
use crate::associated_support::SwitchState;
use crate::associated_support::double_number;
use crate::associated_support::extract_value;

struct FirstPair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Extract for FirstPair<T, U> {
    type Output = T;

    fn extract(self) -> Self::Output {
        self.first
    }
}

fn main() {
    assert_eq!(extract_value(Holder { value: 120 }), 120);
    assert!(extract_value(SwitchState { enabled: true }));

    let source = FirstPair {
        first: 120,
        second: true,
    };
    println!("取出前的第二欄={}", source.second);
    let amount: i32 = extract_value(source);
    assert_eq!(amount, 120);
    println!("取出第一欄={amount}");

    let other = FirstPair {
        first: 80,
        second: false,
    };
    assert_eq!(double_number(other), 160);
    println!("另一份來源的數字乘二=160");
}
