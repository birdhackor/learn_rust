use std::rc::Rc;

fn require_send<T: Send>(value: T) -> T {
    value
}

fn main() {
    let owner = Rc::new(80);
    let saved = require_send(owner);
    println!("{}", *saved);
}
