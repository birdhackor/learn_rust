use std::mem;

struct BorrowedNote<'a> {
    value: i32,
    saved: Option<&'a i32>,
}

fn main() {
    let mut left = BorrowedNote {
        value: 80,
        saved: None,
    };
    let mut right = BorrowedNote {
        value: 120,
        saved: None,
    };
    left.saved = Some(&left.value);
    right.saved = Some(&right.value);
    mem::swap(&mut left, &mut right);
    println!("交換後仍想使用借用：{:?}、{:?}", left.saved, right.saved);
}
