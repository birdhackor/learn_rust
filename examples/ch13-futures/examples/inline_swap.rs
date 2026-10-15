use ch13_futures::addresses::AddressNote;
use std::mem;

fn main() {
    let mut left = AddressNote::new(80);
    let mut right = AddressNote::new(120);
    left.remember();
    right.remember();
    assert!(left.address_matches());
    assert!(right.address_matches());
    println!(
        "交換前：left 記錄地址與目前欄位相同={}，right={}",
        left.address_matches(),
        right.address_matches()
    );

    mem::swap(&mut left, &mut right);
    assert_eq!(left.value, 120);
    assert_eq!(right.value, 80);
    assert!(!left.address_matches());
    assert!(!right.address_matches());
    println!("交換後：left 值={}，right 值={}", left.value, right.value);
    println!(
        "交換後：left 記錄地址與目前欄位相同={}，right={}",
        left.address_matches(),
        right.address_matches()
    );
}
