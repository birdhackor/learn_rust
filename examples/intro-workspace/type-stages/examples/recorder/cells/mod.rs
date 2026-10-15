use std::cell::Ref;
use std::cell::RefCell;

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
