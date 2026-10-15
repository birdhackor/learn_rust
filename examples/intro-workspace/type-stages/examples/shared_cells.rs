use std::cell::RefCell;
use std::mem::drop;
use std::rc::Rc;

fn main() {
    let first = Rc::new(RefCell::new(vec![80]));
    let second = Rc::clone(&first);
    {
        let mut writable = second.borrow_mut();
        writable.push(120);
    }
    let view = first.borrow();
    println!(
        "另一個Rc看到同一清單={}、{}",
        view.as_slice()[0],
        view.as_slice()[1]
    );
    println!("Rc strong數量={}", Rc::strong_count(&first));
    drop(view);
    drop(first);
    let view = second.borrow();
    println!("只剩second仍有{}項", view.len());
}
