pub mod contracts;

use crate::contracts::require_send;
use std::cell::Cell;
use std::cell::RefCell;

fn main() {
    let counter = require_send(Cell::new(3));
    counter.set(counter.get() + 1);
    println!("owned Cell通過Send，計數={}", counter.get());
    assert_eq!(counter.get(), 4);

    let values = require_send(RefCell::new(vec![80]));
    {
        let mut writable = values.borrow_mut();
        writable.push(120);
    }
    let view = values.borrow();
    let total = view.as_slice()[0] + view.as_slice()[1];
    println!("owned RefCell通過Send，小計={total}");
    assert_eq!(view.len(), 2);
    assert_eq!(total, 200);
}
