use std::any::Any;
use std::any::TypeId;

pub fn concrete_id<T: Any>(value: &T) -> TypeId {
    value.type_id()
}
