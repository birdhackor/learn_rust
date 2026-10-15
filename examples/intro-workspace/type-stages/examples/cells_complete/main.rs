pub mod cells;

use cells::Record;
use cells::Recorder;
use std::mem::drop;
use std::rc::Rc;

fn main() {
    let first = Rc::new(Recorder::empty());
    let second = Rc::clone(&first);
    first.record(80);
    second.record(120);
    let view = first.view();
    assert_eq!(view.len(), 2);
    assert_eq!(view.as_slice()[0] + view.as_slice()[1], 200);
    drop(view);
    second.record(40);
    let view = first.view();
    assert_eq!(view.len(), 3);
    println!(
        "共享記錄器新增後總額={}",
        view.as_slice()[0] + view.as_slice()[1] + view.as_slice()[2]
    );
}
