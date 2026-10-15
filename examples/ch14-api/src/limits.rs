use std::error::Error;
use std::fmt;

// --8<-- [start:limit-error]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum LimitError {
    Zero,
    TooLarge(usize),
}

impl fmt::Display for LimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "結果上限不能為零"),
            Self::TooLarge(value) => write!(f, "結果上限 {} 超過1000", *value),
        }
    }
}

impl Error for LimitError {}
// --8<-- [end:limit-error]

// --8<-- [start:limit]
/// 可接受的結果上限，範圍為1到1000。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatchLimit(usize);

impl BatchLimit {
    /// 驗證原數字；失敗保留Zero或TooLarge原因。
    ///
    /// # Examples
    ///
    /// ```
    /// use ch14_api::BatchLimit;
    /// let limit = BatchLimit::new(2).expect("2在有效範圍內");
    /// assert_eq!(limit.get(), 2);
    /// ```
    pub fn new(value: usize) -> Result<Self, LimitError> {
        if value == 0 {
            Err(LimitError::Zero)
        } else if value > 1000 {
            Err(LimitError::TooLarge(value))
        } else {
            Ok(Self(value))
        }
    }

    pub fn get(&self) -> usize {
        self.0
    }
}
// --8<-- [end:limit]

// --8<-- [start:limit-default]
impl Default for BatchLimit {
    fn default() -> Self {
        Self(100)
    }
}
// --8<-- [end:limit-default]
