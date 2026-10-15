pub mod polling;

use crate::polling::ManualAmount;
use crate::polling::WakeCounter;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

fn main() {
    let counter = Arc::new(WakeCounter::new(0));
    let waker = Waker::from(Arc::clone(&counter));
    let mut cx = Context::from_waker(&waker);
    let mut work = ManualAmount::new(120);
    work.release();
    assert_eq!(counter.notifications(), 0);
    match Pin::new(&mut work).poll(&mut cx) {
        Poll::Ready(value) => {
            assert_eq!(value, 120);
            println!("第1次poll前已授權：Ready({value})");
        }
        Poll::Pending => panic!("檢查前已授權，不需要Pending"),
    }
    assert_eq!(work.poll_count(), 1);
    assert!(work.is_completed());
    println!(
        "poll次數={}，通知={}，停止poll",
        work.poll_count(),
        counter.notifications()
    );
}
