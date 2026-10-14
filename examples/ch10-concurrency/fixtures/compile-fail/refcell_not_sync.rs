use std::cell::RefCell;

fn require_sync<T: Sync + ?Sized>(value: &T) -> &T {
    value
}

fn main() {
    let values = RefCell::new(vec![80]);
    let view = require_sync(&values);
    println!("{}", view.borrow().len());
}
