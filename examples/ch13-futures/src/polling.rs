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

#[cfg(test)]
mod tests {
    use crate::polling::ManualAmount;
    use crate::polling::WakeCounter;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::task::Context;
    use std::task::Poll;
    use std::task::Waker;

    #[test]
    fn a_notification_does_not_poll_or_complete_the_future() {
        let counter = Arc::new(WakeCounter::new(0));
        let waker = Waker::from(Arc::clone(&counter));
        let mut cx = Context::from_waker(&waker);
        let mut work = ManualAmount::new(80);
        match Pin::new(&mut work).poll(&mut cx) {
            Poll::Pending => {}
            Poll::Ready(_) => panic!("未授權不應完成"),
        }
        assert_eq!(work.poll_count(), 1);
        assert!(!work.is_released());
        work.release();
        assert_eq!(counter.notifications(), 1);
        assert!(work.is_released());
        assert!(!work.is_completed());
        assert_eq!(work.poll_count(), 1);
        // 重複同一授權不重新通知已取走的 waiting handle。
        work.release();
        assert_eq!(counter.notifications(), 1);
        match Pin::new(&mut work).poll(&mut cx) {
            Poll::Ready(value) => assert_eq!(value, 80),
            Poll::Pending => panic!("已授權應交出結果"),
        }
        assert!(work.is_completed());
        assert_eq!(work.poll_count(), 2);
    }

    #[test]
    fn an_event_before_the_first_poll_needs_no_notification() {
        let counter = Arc::new(WakeCounter::new(0));
        let waker = Waker::from(Arc::clone(&counter));
        let mut cx = Context::from_waker(&waker);
        let mut work = ManualAmount::new(120);
        work.release();
        assert_eq!(counter.notifications(), 0);
        match Pin::new(&mut work).poll(&mut cx) {
            Poll::Ready(value) => assert_eq!(value, 120),
            Poll::Pending => panic!("檢查前已授權，不需要Pending"),
        }
        assert_eq!(work.poll_count(), 1);
        assert!(work.is_completed());
    }

    #[test]
    fn only_the_latest_pending_context_receives_the_notification() {
        let first_counter = Arc::new(WakeCounter::new(0));
        let second_counter = Arc::new(WakeCounter::new(0));
        let first_waker = Waker::from(Arc::clone(&first_counter));
        let second_waker = Waker::from(Arc::clone(&second_counter));
        let mut first_cx = Context::from_waker(&first_waker);
        let mut second_cx = Context::from_waker(&second_waker);
        let mut work = ManualAmount::new(80);
        match Pin::new(&mut work).poll(&mut first_cx) {
            Poll::Pending => {}
            Poll::Ready(_) => panic!("未授權不應完成"),
        }
        match Pin::new(&mut work).poll(&mut second_cx) {
            Poll::Pending => {}
            Poll::Ready(_) => panic!("更換Context沒有授權"),
        }
        work.release();
        assert_eq!(first_counter.notifications(), 0);
        assert_eq!(second_counter.notifications(), 1);
        assert_eq!(work.poll_count(), 2);
        match Pin::new(&mut work).poll(&mut second_cx) {
            Poll::Ready(value) => assert_eq!(value, 80),
            Poll::Pending => panic!("已授權應交出結果"),
        }
        assert_eq!(work.poll_count(), 3);
    }

    #[test]
    fn the_owned_waker_clone_survives_its_borrowed_context() {
        let counter = Arc::new(WakeCounter::new(0));
        let mut work = ManualAmount::new(80);
        {
            let local_waker = Waker::from(Arc::clone(&counter));
            let mut local_cx = Context::from_waker(&local_waker);
            match Pin::new(&mut work).poll(&mut local_cx) {
                Poll::Pending => {}
                Poll::Ready(_) => panic!("未授權不應完成"),
            }
        }
        work.release();
        assert_eq!(counter.notifications(), 1);
        let next_waker = Waker::from(Arc::clone(&counter));
        let mut next_cx = Context::from_waker(&next_waker);
        match Pin::new(&mut work).poll(&mut next_cx) {
            Poll::Ready(value) => assert_eq!(value, 80),
            Poll::Pending => panic!("已授權應交出結果"),
        }
        assert!(work.is_completed());
    }
}
