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
