// --8<-- [start:imports]
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;
// --8<-- [end:imports]

// --8<-- [start:state]
struct State {
    value: Option<i32>,
    waiting: Option<Waker>,
}

pub struct EventAmount {
    shared: Arc<Mutex<State>>,
    completed: bool,
    polls: usize,
}

pub struct AmountEvent {
    shared: Arc<Mutex<State>>,
}
// --8<-- [end:state]

// --8<-- [start:pair]
pub fn amount_pair() -> (EventAmount, AmountEvent) {
    let shared = Arc::new(Mutex::new(State {
        value: None,
        waiting: None,
    }));
    let event = AmountEvent {
        shared: Arc::clone(&shared),
    };
    let work = EventAmount {
        shared,
        completed: false,
        polls: 0,
    };
    (work, event)
}

impl EventAmount {
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn poll_count(&self) -> usize {
        self.polls
    }
}
// --8<-- [end:pair]

// --8<-- [start:event]
impl AmountEvent {
    pub fn send(self, value: i32) {
        let waiting = {
            let mut state = self.shared.lock().expect("有限事件沒有持鎖panic");
            state.value = Some(value);
            state.waiting.take()
        };
        if let Some(waker) = waiting {
            waker.wake();
        }
    }
}
// --8<-- [end:event]

// --8<-- [start:future]
impl Future for EventAmount {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = self.get_mut();
        if inner.completed {
            panic!("EventAmount 已完成，不應再次 poll");
        }
        inner.polls += 1;
        let next_waiting = cx.waker().clone();
        let (answer, previous_waiting) = {
            let mut state = inner.shared.lock().expect("有限事件沒有持鎖panic");
            let answer = state.value.take();
            let previous_waiting = state.waiting.take();
            if answer.is_none() {
                state.waiting = Some(next_waiting);
            }
            (answer, previous_waiting)
        };
        drop(previous_waiting);
        match answer {
            Some(value) => {
                inner.completed = true;
                Poll::Ready(value)
            }
            None => Poll::Pending,
        }
    }
}
// --8<-- [end:future]

#[cfg(test)]
mod tests {
    use crate::polling::WakeCounter;
    use crate::readiness::State;
    use crate::readiness::amount_pair;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::task::Context;
    use std::task::Poll;
    use std::task::Wake;
    use std::task::Waker;
    use std::thread;

    #[test]
    fn pending_registration_receives_the_external_event_notification() {
        let counter = Arc::new(WakeCounter::new(0));
        let waker = Waker::from(Arc::clone(&counter));
        let mut cx = Context::from_waker(&waker);
        let (mut work, event) = amount_pair();
        match Pin::new(&mut work).poll(&mut cx) {
            Poll::Pending => {}
            Poll::Ready(_) => panic!("事件尚未供值"),
        }
        let worker = thread::spawn(move || event.send(80));
        worker.join().expect("事件worker正常返回");
        assert_eq!(counter.notifications(), 1);
        assert_eq!(work.poll_count(), 1);
        assert!(!work.is_completed());
        match Pin::new(&mut work).poll(&mut cx) {
            Poll::Ready(value) => assert_eq!(value, 80),
            Poll::Pending => panic!("事件已供值"),
        }
        assert!(work.is_completed());
        assert_eq!(work.poll_count(), 2);
    }

    #[test]
    fn an_external_event_before_poll_is_ready_without_a_notification() {
        let counter = Arc::new(WakeCounter::new(0));
        let waker = Waker::from(Arc::clone(&counter));
        let mut cx = Context::from_waker(&waker);
        let (mut work, event) = amount_pair();
        let worker = thread::spawn(move || event.send(120));
        worker.join().expect("事件worker正常返回");
        assert_eq!(counter.notifications(), 0);
        assert_eq!(work.poll_count(), 0);
        match Pin::new(&mut work).poll(&mut cx) {
            Poll::Ready(value) => assert_eq!(value, 120),
            Poll::Pending => panic!("首次檢查前事件已供值"),
        }
        assert_eq!(work.poll_count(), 1);
        assert!(work.is_completed());
    }

    #[test]
    fn the_external_event_uses_only_the_latest_registered_waker() {
        let first = Arc::new(WakeCounter::new(0));
        let second = Arc::new(WakeCounter::new(0));
        let first_waker = Waker::from(Arc::clone(&first));
        let second_waker = Waker::from(Arc::clone(&second));
        let mut first_cx = Context::from_waker(&first_waker);
        let mut second_cx = Context::from_waker(&second_waker);
        let (mut work, event) = amount_pair();
        match Pin::new(&mut work).poll(&mut first_cx) {
            Poll::Pending => {}
            Poll::Ready(_) => panic!("事件尚未供值"),
        }
        match Pin::new(&mut work).poll(&mut second_cx) {
            Poll::Pending => {}
            Poll::Ready(_) => panic!("更換通知位置不供值"),
        }
        let worker = thread::spawn(move || event.send(-1));
        worker.join().expect("事件worker正常返回");
        assert_eq!(first.notifications(), 0);
        assert_eq!(second.notifications(), 1);
        match Pin::new(&mut work).poll(&mut second_cx) {
            Poll::Ready(value) => assert_eq!(value, -1),
            Poll::Pending => panic!("事件已供值"),
        }
        assert_eq!(work.poll_count(), 3);
        assert!(work.is_completed());
    }

    // --8<-- [start:callback-probe]
    struct ReadinessObserver {
        shared: Arc<Mutex<State>>,
        observed: Mutex<Option<i32>>,
    }

    impl Wake for ReadinessObserver {
        fn wake(self: Arc<Self>) {
            let value = {
                let state = self.shared.lock().expect("事件已解鎖");
                state.value
            };
            let mut observed = self.observed.lock().expect("觀察值沒有持鎖panic");
            *observed = value;
        }
    }

    #[test]
    fn the_callback_can_lock_the_state_and_observe_the_supplied_value() {
        let (mut work, event) = amount_pair();
        let observer = Arc::new(ReadinessObserver {
            shared: Arc::clone(&work.shared),
            observed: Mutex::new(None),
        });
        let waker = Waker::from(Arc::clone(&observer));
        let mut cx = Context::from_waker(&waker);
        match Pin::new(&mut work).poll(&mut cx) {
            Poll::Pending => {}
            Poll::Ready(_) => panic!("事件尚未供值"),
        }
        let worker = thread::spawn(move || event.send(0));
        worker.join().expect("事件worker與回呼正常返回");
        {
            let observed = observer.observed.lock().expect("回呼已正常返回");
            match *observed {
                Some(value) => assert_eq!(value, 0),
                None => panic!("回呼應觀察到事件供值"),
            }
        }
        match Pin::new(&mut work).poll(&mut cx) {
            Poll::Ready(value) => assert_eq!(value, 0),
            Poll::Pending => panic!("事件已供值"),
        }
        assert_eq!(work.poll_count(), 2);
    }
    // --8<-- [end:callback-probe]
}
