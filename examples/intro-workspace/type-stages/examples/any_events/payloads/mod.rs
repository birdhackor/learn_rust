use std::any::Any;

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

pub fn payload_number(event: &Event) -> Option<&i32> {
    let view: &dyn Any = &*event.payload;
    view.downcast_ref::<i32>()
}

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
