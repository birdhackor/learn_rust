use std::marker::PhantomPinned;
use std::pin::Pin;
use std::pin::pin;

struct LocalNote {
    value: i32,
    _pin: PhantomPinned,
}

impl LocalNote {
    fn value(self: Pin<&Self>) -> i32 {
        self.get_ref().value
    }
}

fn main() {
    let first = pin!(LocalNote {
        value: 80,
        _pin: PhantomPinned,
    });
    let next = first;
    assert_eq!(next.as_ref().value(), 80);
    println!("局部固定的數字={}；Pin 借用可以交接", next.as_ref().value());
}
