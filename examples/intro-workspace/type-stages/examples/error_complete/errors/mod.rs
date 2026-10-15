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

// --8<-- [start:boxed-count]
pub fn count_for_report(text: &str) -> Result<i32, Box<dyn Error>> {
    let value = text.parse::<i32>()?;
    let count = checked_count(value)?;
    Ok(count)
}
// --8<-- [end:boxed-count]

// --8<-- [start:borrowed-error]
#[derive(Debug)]
pub struct BorrowedParseError<'a> {
    pub label: &'a str,
    pub cause: ParseIntError,
}

impl fmt::Display for BorrowedParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}：無法解析件數", self.label)
    }
}

impl Error for BorrowedParseError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

pub fn with_context<'a>(label: &'a str, text: &str) -> Result<i32, BorrowedParseError<'a>> {
    match text.parse::<i32>() {
        Ok(value) => Ok(value),
        Err(cause) => Err(BorrowedParseError { label, cause }),
    }
}
// --8<-- [end:borrowed-error]

#[cfg(test)]
mod tests {
    use crate::errors::CountError;
    use crate::errors::ParseCountError;
    use crate::errors::count_for_report;
    use crate::errors::parse_count;
    use crate::errors::read_count;
    use crate::errors::report;
    use crate::errors::with_context;
    use std::error::Error;

    #[test]
    fn concrete_errors_still_preserve_the_range_reason() {
        assert_eq!(parse_count("0"), Ok(0));
        assert_eq!(parse_count("100"), Ok(100));
        assert_eq!(
            parse_count("101"),
            Err(ParseCountError::Range(CountError::TooLarge(101)))
        );
    }

    #[test]
    fn a_report_includes_each_context_and_leaf_once() {
        let error = match read_count("-1") {
            Err(error) => error,
            Ok(count) => panic!("固定負數不應取得合法件數={count}"),
        };
        let text = report(&error);
        assert_eq!(
            text.as_str(),
            "處理失敗：讀取件數文字「-1」失敗\n原因：解析後的件數不合法\n原因：件數 -1 不能為負數"
        );
    }

    #[test]
    fn a_borrowed_context_can_expose_an_owned_static_cause() {
        let mut label = String::new();
        label.push_str("資料欄位A");
        let error = match with_context(label.as_str(), "abc") {
            Err(error) => error,
            Ok(count) => panic!("固定無效文字不應取得整數={count}"),
        };
        let cause = error.source().expect("此型別保存解析原因");
        assert_eq!(format!("{cause}").as_str(), "invalid digit found in string");
        assert_eq!(error.label, "資料欄位A");
    }

    #[test]
    fn the_reporting_boundary_accepts_both_error_types() {
        match count_for_report("3") {
            Ok(count) => assert_eq!(count, 3),
            Err(error) => panic!("固定合法文字不應失敗：{}", report(&*error).as_str()),
        }
        let integer = match count_for_report("abc") {
            Err(error) => error,
            Ok(count) => panic!("固定無效文字不應取得整數={count}"),
        };
        let range = match count_for_report("101") {
            Err(error) => error,
            Ok(count) => panic!("固定超上限文字不應取得合法件數={count}"),
        };
        assert_eq!(
            report(&*integer).as_str(),
            "處理失敗：invalid digit found in string"
        );
        assert_eq!(
            report(&*range).as_str(),
            "處理失敗：件數 101 超過批次上限100"
        );
    }
}
