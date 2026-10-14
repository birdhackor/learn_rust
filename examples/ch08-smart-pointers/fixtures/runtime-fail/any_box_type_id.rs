use std::any::Any;
use std::any::TypeId;

fn main() {
    let amount: i32 = 80;
    let boxed: Box<dyn Any> = Box::new(amount);
    assert_eq!(boxed.type_id(), TypeId::of::<i32>());
}
