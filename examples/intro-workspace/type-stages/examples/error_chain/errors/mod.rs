use std::borrow::ToOwned;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
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
// --8<-- [start:parse-error]
#[derive(Debug, PartialEq)]
pub enum ParseCountError {
    Integer(ParseIntError),
    Range(CountError),
}

impl fmt::Display for ParseCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(_) => write!(f, "解析件數時無法取得整數"),
            Self::Range(_) => write!(f, "解析後的件數不合法"),
        }
    }
}

impl Error for ParseCountError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Integer(error) => Some(error),
            Self::Range(error) => Some(error),
        }
    }
}

impl From<ParseIntError> for ParseCountError {
    fn from(error: ParseIntError) -> Self {
        Self::Integer(error)
    }
}

impl From<CountError> for ParseCountError {
    fn from(error: CountError) -> Self {
        Self::Range(error)
    }
}

pub fn parse_count(text: &str) -> Result<i32, ParseCountError> {
    let value = text.parse::<i32>()?;
    let count = checked_count(value)?;
    Ok(count)
}
// --8<-- [end:parse-error]
// --8<-- [start:read-error]
#[derive(Debug)]
pub struct ReadCountError {
    pub input: String,
    pub cause: ParseCountError,
}

impl fmt::Display for ReadCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "讀取件數文字「{}」失敗", self.input.as_str())
    }
}

impl Error for ReadCountError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

pub fn read_count(text: &str) -> Result<i32, ReadCountError> {
    match parse_count(text) {
        Ok(count) => Ok(count),
        Err(cause) => Err(ReadCountError {
            input: text.to_owned(),
            cause,
        }),
    }
}
// --8<-- [end:read-error]
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
