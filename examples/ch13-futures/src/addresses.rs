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

#[cfg(test)]
mod tests {
    use crate::addresses::AddressNote;
    use std::mem;

    #[test]
    fn whole_value_swap_keeps_the_old_address_records_until_explicit_refresh() {
        let mut left = AddressNote::new(80);
        let mut right = AddressNote::new(120);
        assert!(!left.address_matches());
        left.remember();
        right.remember();
        assert!(left.address_matches());
        assert!(right.address_matches());
        mem::swap(&mut left, &mut right);
        assert_eq!(left.value, 120);
        assert_eq!(right.value, 80);
        assert!(!left.address_matches());
        assert!(!right.address_matches());
        left.remember();
        assert!(left.address_matches());
        assert!(!right.address_matches());
        right.remember();
        assert!(right.address_matches());
    }

    #[test]
    fn equal_values_do_not_make_two_distinct_locations_match() {
        let mut left = AddressNote::new(80);
        let mut right = AddressNote::new(80);
        left.remember();
        right.remember();
        mem::swap(&mut left, &mut right);
        assert_eq!(left.value, right.value);
        assert!(!left.address_matches());
        assert!(!right.address_matches());
    }

    #[test]
    fn swapping_box_owners_preserves_the_heap_field_addresses() {
        let mut left = Box::new(AddressNote::new(80));
        let mut right = Box::new(AddressNote::new(120));
        (*left).remember();
        (*right).remember();
        mem::swap(&mut left, &mut right);
        let left_note = &*left;
        let right_note = &*right;
        assert_eq!(left_note.value, 120);
        assert_eq!(right_note.value, 80);
        assert!((*left).address_matches());
        assert!((*right).address_matches());
    }

    #[test]
    fn swapping_only_field_values_keeps_the_address_records_in_place() {
        let mut left = AddressNote::new(80);
        let mut right = AddressNote::new(120);
        left.remember();
        right.remember();
        mem::swap(&mut left.value, &mut right.value);
        assert_eq!(left.value, 120);
        assert_eq!(right.value, 80);
        assert!(left.address_matches());
        assert!(right.address_matches());
    }
}
