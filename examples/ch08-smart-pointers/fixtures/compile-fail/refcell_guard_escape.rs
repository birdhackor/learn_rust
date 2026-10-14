use std::cell::Ref;
use std::cell::RefCell;

fn new_view<'a>() -> Ref<'a, i32> {
    let value = RefCell::new(80);
    value.borrow()
}

fn main() {
    let view = new_view();
    println!("借來的數字={}", *view);
}
