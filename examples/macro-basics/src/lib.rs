// --8<-- [start:reading]
pub trait Reading {
    fn reading(&self) -> i32;
}

pub fn read_value<T: Reading>(value: &T) -> i32 {
    value.reading()
}
// --8<-- [end:reading]
