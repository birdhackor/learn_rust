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
    let first_counter = Arc::new(WakeCounter::new(0));
    let second_counter = Arc::new(WakeCounter::new(0));
    let first_waker = Waker::from(Arc::clone(&first_counter));
    let second_waker = Waker::from(Arc::clone(&second_counter));
    let mut first_cx = Context::from_waker(&first_waker);
    let mut second_cx = Context::from_waker(&second_waker);
    let mut work = ManualAmount::new(80);
    match Pin::new(&mut work).poll(&mut first_cx) {
        Poll::Pending => println!("第1次poll：Pending，保存第一份通知handle"),
        Poll::Ready(_) => panic!("未授權不應完成"),
    }
    match Pin::new(&mut work).poll(&mut second_cx) {
        Poll::Pending => println!("第2次poll：Pending，改存第二份通知handle"),
        Poll::Ready(_) => panic!("更換Context沒有授權"),
    }
    work.release();
    assert_eq!(first_counter.notifications(), 0);
    assert_eq!(second_counter.notifications(), 1);
    assert_eq!(work.poll_count(), 2);
    println!(
        "release後：第一份通知={}，第二份={}，poll次數仍={}",
        first_counter.notifications(),
        second_counter.notifications(),
        work.poll_count()
    );
    match Pin::new(&mut work).poll(&mut second_cx) {
        Poll::Ready(value) => {
            assert_eq!(value, 80);
            println!("driver第3次poll：Ready({value})，停止poll");
        }
        Poll::Pending => panic!("已授權應交出結果"),
    }
    assert!(work.is_completed());
    assert_eq!(work.poll_count(), 3);
}
