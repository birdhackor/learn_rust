use std::mem;
use std::pin::Pin;

fn main() {
    let mut left = 80;
    let mut right = 120;
    let left_pin: Pin<&mut i32> = Pin::new(&mut left);
    let right_pin: Pin<&mut i32> = Pin::new(&mut right);
    mem::swap(left_pin.get_mut(), right_pin.get_mut());
    assert_eq!(left, 120);
    assert_eq!(right, 80);
    println!("i32 是 Unpin，取得普通 mutable 借用後可交換：{left}、{right}");
}
