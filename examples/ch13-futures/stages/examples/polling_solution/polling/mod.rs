// --8<-- [start:imports]
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::Context;
use std::task::Poll;
use std::task::Wake;
use std::task::Waker;
// --8<-- [end:imports]

// --8<-- [start:observer]
pub struct WakeCounter {
    notifications: Mutex<usize>,
}

impl WakeCounter {
    pub fn new(initial_count: usize) -> Self {
        Self {
            notifications: Mutex::new(initial_count),
        }
    }

    pub fn notifications(&self) -> usize {
        let guard = self
            .notifications
            .lock()
            .expect("有限通知觀察器沒有持鎖panic");
        *guard
    }
}

impl Wake for WakeCounter {
    fn wake(self: Arc<Self>) {
        let mut guard = self
            .notifications
            .lock()
            .expect("有限通知觀察器沒有持鎖panic");
        *guard += 1;
    }
}
// --8<-- [end:observer]

// --8<-- [start:amount]
pub struct ManualAmount {
    value: i32,
    released: bool,
    completed: bool,
    waiting: Option<Waker>,
    polls: usize,
}

impl ManualAmount {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            released: false,
            completed: false,
            waiting: None,
            polls: 0,
        }
    }

    pub fn release(&mut self) {
        self.released = true;
        if let Some(waker) = self.waiting.take() {
            waker.wake();
        }
    }

    pub fn is_released(&self) -> bool {
        self.released
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn poll_count(&self) -> usize {
        self.polls
    }
}

impl Future for ManualAmount {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = self.get_mut();
        if inner.completed {
            panic!("ManualAmount 已完成，不應再次 poll");
        }
        inner.polls += 1;
        if inner.released {
            inner.completed = true;
            Poll::Ready(inner.value)
        } else {
            inner.waiting = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
// --8<-- [end:amount]
