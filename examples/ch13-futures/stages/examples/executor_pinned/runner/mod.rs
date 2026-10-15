use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::task::Context;
use std::task::Poll;
use std::task::Wake;
use std::task::Waker;

struct ChannelWake {
    sender: Sender<()>,
}

impl Wake for ChannelWake {
    fn wake(self: Arc<Self>) {
        let _delivery = self.sender.send(());
    }
}

pub fn run_one<F: Future>(work: F) -> F::Output {
    let mut pinned = Box::pin(work);
    let (sender, receiver) = mpsc::channel::<()>();
    let waker = Waker::from(Arc::new(ChannelWake { sender }));
    let mut cx = Context::from_waker(&waker);
    loop {
        println!("executor：poll");
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(value) => {
                println!("executor：Ready，停止poll");
                return value;
            }
            Poll::Pending => {
                println!("executor：Pending，等待通知");
                receiver.recv().expect("run_one仍保有通知傳送端");
                println!("executor：收到通知，安排再poll");
            }
        }
    }
}

pub struct PinnedAnswer {
    value: i32,
    _pin: PhantomPinned,
}

impl PinnedAnswer {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            _pin: PhantomPinned,
        }
    }
}

impl Future for PinnedAnswer {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let view = self.as_ref().get_ref();
        Poll::Ready(view.value)
    }
}
