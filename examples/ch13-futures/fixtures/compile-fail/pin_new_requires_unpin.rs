use std::marker::PhantomPinned;
use std::pin::Pin;

fn main() {
    let mut marker = PhantomPinned;
    let _borrowed = Pin::new(&mut marker);
}
