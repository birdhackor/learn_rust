use std::any::Any;
use std::any::TypeId;

// --8<-- [start:event]
pub struct Event {
    pub id: i32,
    pub payload: Box<dyn Any>,
}

pub fn event(id: i32, payload: impl Any) -> Event {
    Event {
        id,
        payload: Box::new(payload),
    }
}
// --8<-- [end:event]

// --8<-- [start:number-view]
pub fn payload_number(event: &Event) -> Option<&i32> {
    let view: &dyn Any = &*event.payload;
    view.downcast_ref::<i32>()
}
// --8<-- [end:number-view]

// --8<-- [start:number-update]
pub fn add_to_number(event: &mut Event, extra: i32) -> bool {
    let view: &mut dyn Any = &mut *event.payload;
    match view.downcast_mut::<i32>() {
        Some(number) => {
            *number += extra;
            true
        }
        None => false,
    }
}
// --8<-- [end:number-update]

// --8<-- [start:type-id]
pub fn concrete_id<T: Any>(value: &T) -> TypeId {
    value.type_id()
}
// --8<-- [end:type-id]

#[cfg(test)]
mod tests {
    use crate::payloads::add_to_number;
    use crate::payloads::event;
    use crate::payloads::payload_number;
    use std::any::Any;

    #[test]
    fn matching_numeric_payloads_change_but_same_value_other_types_do_not() {
        let amount: i32 = 80;
        let count: usize = 80;
        let mut numeric = event(1, amount);
        let mut other = event(2, count);
        assert!(add_to_number(&mut numeric, 20));
        assert!(!add_to_number(&mut other, 20));
        let updated = match payload_number(&numeric) {
            Some(number) => *number,
            None => -1,
        };
        assert_eq!(updated, 100);
        let as_i32 = match payload_number(&other) {
            Some(number) => *number,
            None => -1,
        };
        assert_eq!(as_i32, -1);
        let retained = match other.payload.downcast_ref::<usize>() {
            Some(original) => *original,
            None => 0,
        };
        assert_eq!(retained, 80);
    }

    #[test]
    fn a_failed_owned_downcast_returns_the_original_string() {
        let mut text = String::new();
        text.push_str("Rust");
        let boxed: Box<dyn Any> = Box::new(text);
        let observed = match boxed.downcast::<i32>() {
            Ok(_) => 0,
            Err(original) => match original.downcast::<String>() {
                Ok(mut text) => {
                    text.push_str(" 教學");
                    assert_eq!(text.as_str(), "Rust 教學");
                    text.as_str().len()
                }
                Err(_) => 0,
            },
        };
        assert_eq!(observed, 11);
    }
}
