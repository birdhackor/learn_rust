use ch13_futures::pinning::PinnedNote;
use std::mem;

fn main() {
    let first = PinnedNote::new(80);
    let mut next = first;
    next.as_mut().set_value(100);
    assert_eq!(next.as_ref().value(), 100);
    assert!(next.as_ref().address_matches());
    println!("next 接手並改值：{}，地址仍一致", next.as_ref().value());
    mem::drop(next);
    println!("next 的資料已正常收尾");
}
