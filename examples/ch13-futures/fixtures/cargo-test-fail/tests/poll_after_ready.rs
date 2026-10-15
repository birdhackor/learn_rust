use ch13_futures::polling::ManualAmount;
use ch13_futures::polling::WakeCounter;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

#[test]
fn polling_again_after_ready_panics() {
    let counter = Arc::new(WakeCounter::new(0));
    let waker = Waker::from(Arc::clone(&counter));
    let mut cx = Context::from_waker(&waker);
    let mut work = ManualAmount::new(80);
    work.release();
    match Pin::new(&mut work).poll(&mut cx) {
        Poll::Ready(value) => assert_eq!(value, 80),
        Poll::Pending => panic!("已授權應交出結果"),
    }
    let _again = Pin::new(&mut work).poll(&mut cx);
}
