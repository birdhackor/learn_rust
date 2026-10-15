use std::rc::Rc;

fn require_send<T: Send>(value: T) -> T {
    value
}

fn main() {
    let owner = Rc::new(80);
    let another = Rc::clone(&owner);
    let work = move || *another;
    let work = require_send(work);
    println!("{}", work());
}
