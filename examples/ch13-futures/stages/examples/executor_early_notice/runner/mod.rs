use std::future::Future;
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

pub struct Recheck {
    value: i32,
    requested: bool,
}

impl Recheck {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            requested: false,
        }
    }
}

impl Future for Recheck {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = self.get_mut();
        if inner.requested {
            Poll::Ready(inner.value)
        } else {
            inner.requested = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
