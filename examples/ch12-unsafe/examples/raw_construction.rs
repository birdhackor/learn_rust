use std::mem::align_of;
use std::ptr::null;

fn main() {
    let mut amount = 80;
    let readable: *const i32 = &raw const amount;
    let writable: *mut i32 = &raw mut amount;
    let empty: *const i32 = null::<i32>();

    println!("局部值pointer是null={}", readable.is_null());
    println!("mutable pointer是null={}", writable.is_null());
    println!("明確的null pointer是null={}", empty.is_null());
    println!("i32要求的alignment={} bytes", align_of::<i32>());
    assert!(!readable.is_null());
    assert!(empty.is_null());
}
