use ch13_futures::polling::ManualAmount;
use ch13_futures::polling::WakeCounter;
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
    let mut work = ManualAmount::new(80);
    println!(
        "建立工作：poll次數={}，通知={}",
        work.poll_count(),
        counter.notifications()
    );

    match Pin::new(&mut work).poll(&mut cx) {
        Poll::Pending => println!("第1次poll：Pending，已保存目前通知handle"),
        Poll::Ready(_) => panic!("未授權不應完成"),
    }
    assert_eq!(work.poll_count(), 1);
    assert_eq!(counter.notifications(), 0);
    work.release();
    assert!(work.is_released());
    assert!(!work.is_completed());
    assert_eq!(work.poll_count(), 1);
    assert_eq!(counter.notifications(), 1);
    println!(
        "release後：已授權={}，通知={}，poll次數仍={}，已完成={}",
        work.is_released(),
        counter.notifications(),
        work.poll_count(),
        work.is_completed()
    );

    match Pin::new(&mut work).poll(&mut cx) {
        Poll::Ready(value) => {
            assert_eq!(value, 80);
            println!("driver第2次poll：Ready({value})");
        }
        Poll::Pending => panic!("已授權應交出結果"),
    }
    assert!(work.is_completed());
    assert_eq!(work.poll_count(), 2);
    println!("完成後：poll次數={}，停止poll", work.poll_count());
}
