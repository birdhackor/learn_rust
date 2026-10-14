use std::cell::Cell;

fn require_sync<T: Sync + ?Sized>(value: &T) -> &T {
    value
}

fn main() {
    let counter = Cell::new(3);
    let view = require_sync(&counter);
    println!("{}", view.get());
}
