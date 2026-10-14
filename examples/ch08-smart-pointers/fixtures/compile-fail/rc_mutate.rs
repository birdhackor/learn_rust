use std::rc::Rc;

fn main() {
    let mut owner = Rc::new(80);
    *owner += 20;
    println!("數字={}", *owner);
}
