pub mod polling;
pub mod readiness;

use crate::polling::WakeCounter;
use crate::readiness::amount_pair;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;
use std::thread;

fn main() {
    let counter = Arc::new(WakeCounter::new(0));
    let waker = Waker::from(Arc::clone(&counter));
    let mut cx = Context::from_waker(&waker);
    let (mut work, event) = amount_pair();
    let worker = thread::spawn(move || event.send(120));
    worker.join().expect("事件worker正常返回");
    assert_eq!(counter.notifications(), 0);
    assert_eq!(work.poll_count(), 0);
    println!("先事件：已供120，worker已收尾，通知=0，poll次數=0");

    match Pin::new(&mut work).poll(&mut cx) {
        Poll::Ready(value) => {
            assert_eq!(value, 120);
            println!("driver首次poll：Ready({value})，停止poll");
        }
        Poll::Pending => panic!("事件已供值"),
    }
    assert_eq!(work.poll_count(), 1);
    assert!(work.is_completed());
}
