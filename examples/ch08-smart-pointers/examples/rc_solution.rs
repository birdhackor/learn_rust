use std::mem::drop;
use std::rc::Rc;

fn main() {
    let first = Rc::new(120);
    let observer = Rc::downgrade(&first);
    let second = Rc::clone(&first);
    let third = Rc::clone(&first);
    assert_eq!(Rc::strong_count(&first), 3);
    drop(first);
    drop(second);
    println!("只剩第三個handle={}", *third);
    assert_eq!(Rc::strong_count(&third), 1);
    let found = match observer.upgrade() {
        Some(temporary) => {
            println!("Weak暫時取得資料={}", *temporary);
            true
        }
        None => false,
    };
    assert!(found);
    drop(third);
    let still_alive = match observer.upgrade() {
        Some(_) => true,
        None => {
            println!("所有strong結束後，Weak沒有資料");
            false
        }
    };
    assert!(!still_alive);
}
