// --8<-- [start:model]
use std::ptr;

pub struct AddressNote {
    pub value: i32,
    saved: *const i32,
}

impl AddressNote {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            saved: ptr::null(),
        }
    }

    pub fn remember(&mut self) {
        self.saved = &raw const self.value;
    }

    pub fn address_matches(&self) -> bool {
        ptr::eq(self.saved, &raw const self.value)
    }
}
// --8<-- [end:model]
