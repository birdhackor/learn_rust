mod views;

use crate::views::Wrapper;
use std::ops::Deref;
use std::ops::DerefMut;

fn main() {
    let mut wrapped = Wrapper { value: 80 };
    let view = wrapped.deref();
    println!("deref借到的數字={}", *view);
    let view = &*wrapped;
    println!("*語法借到的數字={}", *view);
    let writable = wrapped.deref_mut();
    *writable += 20;
    println!("修改後的原欄位={}", wrapped.value);
}
