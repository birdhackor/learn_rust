use ch13_futures::addresses::AddressNote;
use std::mem;

fn main() {
    let mut left = Box::new(AddressNote::new(80));
    let mut right = Box::new(AddressNote::new(120));
    (*left).remember();
    (*right).remember();
    assert!((*left).address_matches());
    assert!((*right).address_matches());

    mem::swap(&mut left, &mut right);
    let left_note = &*left;
    let right_note = &*right;
    assert_eq!(left_note.value, 120);
    assert_eq!(right_note.value, 80);
    assert!((*left).address_matches());
    assert!((*right).address_matches());
    println!(
        "交換 Box owner 後：left 值={}，right 值={}",
        left_note.value, right_note.value
    );
    println!(
        "記錄地址與目前欄位相同：left={}，right={}",
        (*left).address_matches(),
        (*right).address_matches()
    );
}
