use std::cell::RefCell;
use std::mem::drop;

fn main() {
    let values = RefCell::new(vec![80]);
    let first = values.borrow();
    let second = values.borrow();
    println!(
        "兩份shared guard讀同一值={}、{}",
        first.as_slice()[0],
        second.as_slice()[0]
    );
    drop(first);
    drop(second);

    let mut writable = values.borrow_mut();
    writable.push(120);
    drop(writable);

    let final_view = values.borrow();
    println!(
        "修改後有{}項，總額={}",
        final_view.len(),
        final_view.as_slice()[0] + final_view.as_slice()[1]
    );
}
