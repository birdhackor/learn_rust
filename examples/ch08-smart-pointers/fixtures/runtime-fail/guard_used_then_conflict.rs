use std::cell::RefCell;

fn main() {
    let value = RefCell::new(80);
    let view = value.borrow();
    println!("shared guard最後讀取={}", *view);
    let mut writable = value.borrow_mut();
    *writable += 20;
}
