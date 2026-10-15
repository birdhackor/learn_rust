use std::cell::RefCell;
use std::mem::drop;

fn main() {
    let value = RefCell::new(80);
    let view = value.borrow();
    println!("shared guard最後讀取={}", *view);
    match value.try_borrow_mut() {
        Ok(mut writable) => {
            *writable += 20;
            println!("意外取得mutable guard={}", *writable);
        }
        Err(_) => println!("讀完還沒drop，mutable借用被拒絕"),
    }
    drop(view);
    match value.try_borrow_mut() {
        Ok(mut writable) => {
            *writable += 20;
            println!("drop shared guard後更新={}", *writable);
        }
        Err(_) => println!("仍有活躍借用"),
    }

    let mut writable = value.borrow_mut();
    *writable += 20;
    match value.try_borrow() {
        Ok(view) => println!("意外取得shared guard={}", *view),
        Err(_) => println!("mutable guard仍在，shared借用被拒絕"),
    }
    drop(writable);
    let final_view = value.borrow();
    println!("guard釋放後讀回={}", *final_view);
}
