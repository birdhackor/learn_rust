use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
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
