// --8<-- [start:caller-input]
pub fn on_caller<'a, F>(value: &'a i32, callback: F) -> i32
where
    F: FnOnce(&'a i32) -> i32,
{
    callback(value)
}
// --8<-- [end:caller-input]

// --8<-- [start:local-input]
pub fn with_local<F, T>(callback: F) -> T
where
    F: for<'a> FnOnce(&'a i32) -> T,
{
    let value = 80;
    callback(&value)
}
// --8<-- [end:local-input]

// --8<-- [start:projection]
pub fn sum_selected<F>(choose: &F) -> i32
where
    F: for<'a> Fn(&'a (i32, i32)) -> &'a i32,
{
    let first = (80, 120);
    let first_selected = *choose(&first);
    let second_selected = {
        let second = (3, 6);
        *choose(&second)
    };
    first_selected + second_selected
}
// --8<-- [end:projection]
