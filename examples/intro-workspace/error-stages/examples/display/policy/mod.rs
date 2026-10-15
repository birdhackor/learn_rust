// --8<-- [start:count-error]
#[derive(Debug, PartialEq)]
pub enum CountError {
    Negative(i32),
    TooLarge(i32),
}

pub fn checked_count(value: i32) -> Result<i32, CountError> {
    if value < 0 {
        Err(CountError::Negative(value))
    } else if value > 100 {
        Err(CountError::TooLarge(value))
    } else {
        Ok(value)
    }
}
// --8<-- [end:count-error]

// --8<-- [start:policy-tests]
#[cfg(test)]
mod tests {
    use crate::policy::CountError;
    use crate::policy::checked_count;

    #[test]
    fn zero_and_hundred_are_valid_boundary_values() {
        assert_eq!(checked_count(0), Ok(0));
        assert_eq!(checked_count(100), Ok(100));
    }

    #[test]
    fn invalid_values_keep_distinct_reasons_and_input() {
        assert_eq!(checked_count(-1), Err(CountError::Negative(-1)));
        assert_eq!(checked_count(101), Err(CountError::TooLarge(101)));
    }
}
// --8<-- [end:policy-tests]
