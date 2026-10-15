// --8<-- [start:amount]
#[derive(Debug, PartialEq)]
pub enum WorkError {
    Negative(i32),
}

pub fn check_amount(amount: i32) -> Result<i32, WorkError> {
    if amount < 0 {
        Err(WorkError::Negative(amount))
    } else {
        Ok(amount)
    }
}
// --8<-- [end:amount]
