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
