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
// --8<-- [start:boxed-count]
pub fn count_for_report(text: &str) -> Result<i32, Box<dyn Error>> {
    let value = text.parse::<i32>()?;
    let count = checked_count(value)?;
    Ok(count)
}
// --8<-- [end:boxed-count]
// --8<-- [start:report]
pub fn report(error: &dyn Error) -> String {
    let mut message = format!("處理失敗：{error}");
    let mut cause = error.source();
    loop {
        match cause {
            Some(current) => {
                message.push_str("\n原因：");
                let text = format!("{current}");
                message.push_str(text.as_str());
                cause = current.source();
            }
            None => return message,
        }
    }
}
// --8<-- [end:report]
