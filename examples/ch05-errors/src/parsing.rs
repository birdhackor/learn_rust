use crate::conversions::Count;
use crate::policy::CountError;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

// --8<-- [start:parse-count-error]
#[derive(Debug, PartialEq)]
pub enum ParseCountError {
    Integer(ParseIntError),
    Range(CountError),
}

impl fmt::Display for ParseCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(error) => write!(f, "無法解析整數：{error}"),
            Self::Range(error) => write!(f, "件數不符規則：{error}"),
        }
    }
}
// --8<-- [end:parse-count-error]

// --8<-- [start:parse-error-from]
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
// --8<-- [end:parse-error-from]

// --8<-- [start:count-from-str]
impl FromStr for Count {
    type Err = ParseCountError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let value = match i32::from_str(text) {
            Ok(value) => value,
            Err(error) => return Err(ParseCountError::from(error)),
        };

        match Self::try_from(value) {
            Ok(count) => Ok(count),
            Err(error) => Err(ParseCountError::from(error)),
        }
    }
}
// --8<-- [end:count-from-str]

// --8<-- [start:parsing-tests]
#[cfg(test)]
mod tests {
    use crate::conversions::Count;
    use crate::parsing::ParseCountError;
    use crate::policy::CountError;
    use std::str::FromStr;

    #[test]
    fn integer_syntax_and_valid_limits_are_accepted() {
        for (text, expected) in [("0", 0), ("+3", 3), ("100", 100)] {
            let count = Count::from_str(text).expect("固定文字表示有效件數");
            assert_eq!(count.value(), expected);
        }
    }

    #[test]
    fn malformed_and_unrepresentable_integers_keep_integer_errors() {
        for text in ["", "abc", "3 ", "2147483648"] {
            match text.parse::<Count>() {
                Err(ParseCountError::Integer(_)) => {}
                other => panic!("預期整數解析失敗，得到 {other:?}"),
            }
        }
    }

    #[test]
    fn valid_integer_syntax_can_still_fail_the_count_rule() {
        assert_eq!(
            "-1".parse::<Count>(),
            Err(ParseCountError::Range(CountError::Negative(-1)))
        );
        assert_eq!(
            "101".parse::<Count>(),
            Err(ParseCountError::Range(CountError::TooLarge(101)))
        );
    }

    #[test]
    fn parsed_count_does_not_borrow_the_input_string() {
        let count;
        {
            let mut text = String::new();
            text.push_str("+3");
            count = text
                .as_str()
                .parse::<Count>()
                .expect("固定文字表示有效件數");
        }
        assert_eq!(count.value(), 3);
    }
}
// --8<-- [end:parsing-tests]
