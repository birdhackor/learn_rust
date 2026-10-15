use std::cell::Cell;

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
