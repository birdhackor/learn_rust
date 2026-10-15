use crate::policy::CountError;
use crate::policy::checked_count;
use std::convert::From;
use std::convert::TryFrom;

// --8<-- [start:count-model]
#[derive(Debug, PartialEq)]
pub struct Count {
    value: i32,
}

impl Count {
    pub fn value(&self) -> i32 {
        self.value
    }
}
// --8<-- [end:count-model]

// --8<-- [start:count-try-from]
impl TryFrom<i32> for Count {
    type Error = CountError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match checked_count(value) {
            Ok(value) => Ok(Self { value }),
            Err(error) => Err(error),
        }
    }
}
// --8<-- [end:count-try-from]

// --8<-- [start:count-from]
impl From<Count> for i32 {
    fn from(count: Count) -> Self {
        count.value
    }
}
// --8<-- [end:count-from]

// --8<-- [start:conversion-tests]
#[cfg(test)]
mod tests {
    use crate::conversions::Count;
    use crate::policy::CountError;

    #[test]
    fn valid_limits_create_counts() {
        for value in [0, 100] {
            let result = Count::try_from(value);
            match result {
                Ok(count) => assert_eq!(count.value(), value),
                Err(error) => panic!("有效邊界不應失敗：{error}"),
            }
        }
    }

    #[test]
    fn invalid_counts_preserve_the_reason() {
        assert_eq!(Count::try_from(-1), Err(CountError::Negative(-1)));
        assert_eq!(Count::try_from(101), Err(CountError::TooLarge(101)));
    }

    #[test]
    fn converting_back_preserves_the_count() {
        for value in [0, 3, 100] {
            let count = Count::try_from(value).expect("固定資料皆符合件數規則");
            let recovered = i32::from(count);
            assert_eq!(recovered, value);
        }
    }
}
// --8<-- [end:conversion-tests]
