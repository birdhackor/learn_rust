use ch12_unsafe::gat::GuardedNumbers;
use ch12_unsafe::gat::Numbers;
use ch12_unsafe::gat::borrow_view;
use std::cell::Ref;
use std::cell::RefCell;
use std::mem::drop;

fn main() {
    let mut plain = Numbers {
        values: vec![80, 120],
    };
    let first: &[i32] = borrow_view(&plain);
    println!("普通視圖總額={}", first[0] + first[1]);
    plain.values.push(40);
    let second: &[i32] = borrow_view(&plain);
    println!("下一次借用有{}項", second.len());

    let guarded = GuardedNumbers {
        values: RefCell::new(vec![80, 120]),
    };
    let view: Ref<'_, Vec<i32>> = borrow_view(&guarded);
    println!("guard視圖總額={}", view.as_slice()[0] + view.as_slice()[1]);
    drop(view);
    {
        let mut values = guarded.values.borrow_mut();
        values.push(40);
    }
    let next = borrow_view(&guarded);
    println!("guard清理後，下次借用有{}項", next.len());
}
