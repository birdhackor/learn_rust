use std::error::Error;
use std::fmt;
// --8<-- [start:leaf-error]
#[derive(Debug, PartialEq)]
pub enum CountError {
    Negative(i32),
    TooLarge(i32),
}

impl fmt::Display for CountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative(value) => write!(f, "件數 {} 不能為負數", *value),
            Self::TooLarge(value) => write!(f, "件數 {} 超過批次上限100", *value),
        }
    }
}

impl Error for CountError {}

pub fn checked_count(value: i32) -> Result<i32, CountError> {
    if value < 0 {
        Err(CountError::Negative(value))
    } else if value > 100 {
        Err(CountError::TooLarge(value))
    } else {
        Ok(value)
    }
}
// --8<-- [end:leaf-error]
