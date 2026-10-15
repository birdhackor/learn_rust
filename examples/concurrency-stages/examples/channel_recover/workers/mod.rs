// --8<-- [start:sum]
pub fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for value in values {
        total += *value;
    }
    total
}
// --8<-- [end:sum]
