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
    let first = Arc::new(WakeCounter::new(0));
    let second = Arc::new(WakeCounter::new(0));
    let first_waker = Waker::from(Arc::clone(&first));
    let second_waker = Waker::from(Arc::clone(&second));
    let mut first_cx = Context::from_waker(&first_waker);
    let mut second_cx = Context::from_waker(&second_waker);
    let (mut work, event) = amount_pair();
    let (start_sender, start_receiver) = mpsc::channel::<i32>();
    let worker = thread::spawn(move || {
        let value = start_receiver.recv().expect("main應提供一次事件數值");
        event.send(value);
    });
    match Pin::new(&mut work).poll(&mut first_cx) {
        Poll::Pending => println!("第1次poll：Pending，登記第一份通知位置"),
        Poll::Ready(_) => panic!("事件尚未供值"),
    }
    match Pin::new(&mut work).poll(&mut second_cx) {
        Poll::Pending => println!("第2次poll：Pending，改登記第二份通知位置"),
        Poll::Ready(_) => panic!("更換通知位置沒有供值"),
    }
    start_sender.send(80).expect("worker仍持有接收端");
    drop(start_sender);
    worker.join().expect("事件worker正常返回");
    assert_eq!(first.notifications(), 0);
    assert_eq!(second.notifications(), 1);
    assert_eq!(work.poll_count(), 2);
    println!(
        "worker已收尾：第一份通知={}，第二份={}，poll次數仍={}",
        first.notifications(),
        second.notifications(),
        work.poll_count()
    );
    match Pin::new(&mut work).poll(&mut second_cx) {
        Poll::Ready(value) => {
            assert_eq!(value, 80);
            println!("driver第3次poll：Ready({value})，停止poll");
        }
        Poll::Pending => panic!("事件已供值"),
    }
    assert!(work.is_completed());
    assert_eq!(work.poll_count(), 3);
}
