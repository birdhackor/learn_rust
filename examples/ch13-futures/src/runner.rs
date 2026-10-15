// --8<-- [start:imports]
use crate::readiness::EventAmount;
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
// --8<-- [end:imports]

// --8<-- [start:notification]
struct ChannelWake {
    sender: Sender<()>,
}

impl Wake for ChannelWake {
    fn wake(self: Arc<Self>) {
        let _delivery = self.sender.send(());
    }
}
// --8<-- [end:notification]

// --8<-- [start:executor]
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
// --8<-- [end:executor]

// --8<-- [start:start-amount]
pub struct StartAmount {
    work: EventAmount,
    start: Option<Sender<i32>>,
    value: i32,
}

impl StartAmount {
    pub fn new(work: EventAmount, start: Sender<i32>, value: i32) -> Self {
        Self {
            work,
            start: Some(start),
            value,
        }
    }
}

impl Future for StartAmount {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = self.get_mut();
        match Pin::new(&mut inner.work).poll(cx) {
            Poll::Ready(value) => Poll::Ready(value),
            Poll::Pending => {
                if let Some(start) = inner.start.take() {
                    start.send(inner.value).expect("worker仍持有控制接收端");
                }
                Poll::Pending
            }
        }
    }
}
// --8<-- [end:start-amount]

// --8<-- [start:recheck]
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
// --8<-- [end:recheck]

// --8<-- [start:pinned-answer]
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
// --8<-- [end:pinned-answer]

#[cfg(test)]
mod tests {
    use crate::readiness::amount_pair;
    use crate::runner::PinnedAnswer;
    use crate::runner::Recheck;
    use crate::runner::StartAmount;
    use crate::runner::run_one;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn an_external_event_wakes_the_executor_after_pending_registration() {
        let (work, event) = amount_pair();
        let (start_sender, start_receiver) = mpsc::channel::<i32>();
        let worker = thread::spawn(move || {
            let value = start_receiver.recv().expect("首次Pending應供控制值");
            event.send(value);
        });
        let result = run_one(StartAmount::new(work, start_sender, 80));
        worker.join().expect("事件worker正常返回");
        assert_eq!(result, 80);
    }

    #[test]
    fn a_notice_sent_inside_poll_survives_until_the_driver_receives_it() {
        let result = run_one(Recheck::new(120));
        assert_eq!(result, 120);
    }

    #[test]
    fn the_executor_accepts_a_future_without_unpin() {
        let result = run_one(PinnedAnswer::new(0));
        assert_eq!(result, 0);
    }
}
