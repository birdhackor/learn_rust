pub mod readiness;
pub mod runner;

use crate::readiness::amount_pair;
use crate::runner::StartAmount;
use crate::runner::run_one;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (work, event) = amount_pair();
    let (start_sender, start_receiver) = mpsc::channel::<i32>();
    let worker = thread::spawn(move || {
        let value = start_receiver.recv().expect("首次Pending應供控制值");
        event.send(value);
    });
    let value = run_one(StartAmount::new(work, start_sender, 120));
    assert_eq!(value, 120);
    println!("練習答案={value}");
    worker.join().expect("事件worker正常返回");
    println!("練習worker已收尾");
}
