// --8<-- [start:local-input]
pub fn with_local<F, T>(callback: F) -> T
where
    F: for<'a> FnOnce(&'a i32) -> T,
{
    let value = 80;
    callback(&value)
}
// --8<-- [end:local-input]
