use std::sync::Arc;
use std::sync::Mutex;
use std::task::Wake;

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
