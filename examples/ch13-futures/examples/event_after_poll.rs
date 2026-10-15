use ch13_futures::polling::WakeCounter;
use ch13_futures::readiness::amount_pair;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::mpsc;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;
use std::thread;

fn main() {
    let counter = Arc::new(WakeCounter::new(0));
    let waker = Waker::from(Arc::clone(&counter));
    let mut cx = Context::from_waker(&waker);
    let (mut work, event) = amount_pair();
    let (start_sender, start_receiver) = mpsc::channel::<i32>();
    let worker = thread::spawn(move || {
        let value = start_receiver.recv().expect("main應提供一次事件數值");
        event.send(value);
    });

    match Pin::new(&mut work).poll(&mut cx) {
        Poll::Pending => println!("先poll：Pending，已登記通知位置"),
        Poll::Ready(_) => panic!("main尚未允許事件供值"),
    }
    assert_eq!(counter.notifications(), 0);
    start_sender.send(80).expect("worker仍持有接收端");
    drop(start_sender);
    worker.join().expect("事件worker正常返回");
    assert_eq!(counter.notifications(), 1);
    assert_eq!(work.poll_count(), 1);
    assert!(!work.is_completed());
    println!(
        "worker已供值並收尾：通知={}，poll次數仍={}，已完成={}",
        counter.notifications(),
        work.poll_count(),
        work.is_completed()
    );

    match Pin::new(&mut work).poll(&mut cx) {
        Poll::Ready(value) => {
            assert_eq!(value, 80);
            println!("driver第2次poll：Ready({value})，停止poll");
        }
        Poll::Pending => panic!("事件已供值"),
    }
    assert!(work.is_completed());
    assert_eq!(work.poll_count(), 2);
}
