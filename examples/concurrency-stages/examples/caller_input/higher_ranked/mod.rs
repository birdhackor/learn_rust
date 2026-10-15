// --8<-- [start:caller-input]
pub fn on_caller<'a, F>(value: &'a i32, callback: F) -> i32
where
    F: FnOnce(&'a i32) -> i32,
{
    callback(value)
}
// --8<-- [end:caller-input]
