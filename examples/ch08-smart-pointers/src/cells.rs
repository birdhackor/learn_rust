use std::cell::Cell;
use std::cell::Ref;
use std::cell::RefCell;

// --8<-- [start:counter]
pub struct Counter {
    value: Cell<i32>,
}

impl Counter {
    pub fn new(start: i32) -> Self {
        Self {
            value: Cell::new(start),
        }
    }

    pub fn tick(&self) -> i32 {
        let next = self.value.get() + 1;
        self.value.set(next);
        next
    }
}
// --8<-- [end:counter]

// --8<-- [start:recorder]
pub trait Record {
    fn record(&self, amount: i32);
}

pub struct Recorder {
    values: RefCell<Vec<i32>>,
}

impl Recorder {
    pub fn empty() -> Self {
        Self {
            values: RefCell::new(Vec::new()),
        }
    }

    pub fn view(&self) -> Ref<'_, Vec<i32>> {
        self.values.borrow()
    }
}

impl Record for Recorder {
    fn record(&self, amount: i32) {
        let mut values = self.values.borrow_mut();
        values.push(amount);
    }
}
// --8<-- [end:recorder]

#[cfg(test)]
mod tests {
    use crate::cells::Counter;
    use crate::cells::Record;
    use crate::cells::Recorder;
    use std::cell::RefCell;
    use std::mem::drop;

    #[test]
    fn shared_callers_advance_the_same_cell_counter() {
        let counter = Counter::new(0);
        let first = &counter;
        let second = &counter;
        assert_eq!(first.tick(), 1);
        assert_eq!(second.tick(), 2);
        assert_eq!(first.tick(), 3);
    }

    #[test]
    fn every_shared_guard_must_end_before_a_mutable_guard_starts() {
        let cell = RefCell::new(80);
        let first = cell.borrow();
        let second = cell.borrow();
        assert_eq!(*first, 80);
        assert_eq!(*second, 80);
        drop(first);
        let before_release = match cell.try_borrow_mut() {
            Ok(writable) => *writable,
            Err(_) => -1,
        };
        assert_eq!(before_release, -1);
        drop(second);
        let after_release = match cell.try_borrow_mut() {
            Ok(mut writable) => {
                *writable += 20;
                *writable
            }
            Err(_) => -1,
        };
        assert_eq!(after_release, 100);
    }

    #[test]
    fn recording_preserves_values_and_order_across_a_released_view() {
        let recorder = Recorder::empty();
        recorder.record(0);
        recorder.record(120);
        let view = recorder.view();
        assert_eq!(view.len(), 2);
        assert_eq!(view.as_slice()[0], 0);
        assert_eq!(view.as_slice()[1], 120);
        drop(view);
        recorder.record(40);
        let view = recorder.view();
        assert_eq!(view.len(), 3);
        assert_eq!(view.as_slice()[2], 40);
    }
}
