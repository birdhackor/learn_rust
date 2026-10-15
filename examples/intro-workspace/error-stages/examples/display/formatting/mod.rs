use crate::policy::CountError;
use std::fmt;

// --8<-- [start:count-display]
impl fmt::Display for CountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CountError::Negative(value) => write!(f, "件數 {} 不能為負數", *value),
            CountError::TooLarge(value) => write!(f, "件數 {} 超過批次上限100", *value),
        }
    }
}
// --8<-- [end:count-display]

#[cfg(test)]
mod tests {
    use crate::policy::CountError;

    #[test]
    fn messages_keep_the_rejected_value_and_reason() {
        let negative = format!("{}", CountError::Negative(-3));
        let too_large = format!("{}", CountError::TooLarge(101));
        assert_eq!(negative.as_str(), "件數 -3 不能為負數");
        assert_eq!(too_large.as_str(), "件數 101 超過批次上限100");
    }
}
