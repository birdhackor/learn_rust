// --8<-- [start:consumer-models]
use ch14_reading_derive::ReadingValue;

#[derive(Debug, PartialEq, ReadingValue)]
pub struct Meters(pub i32);

#[derive(Debug, PartialEq, ReadingValue)]
pub struct Seconds(pub i32);
// --8<-- [end:consumer-models]

// --8<-- [start:consumer-tests]
#[cfg(test)]
mod tests {
    use ch14_macros::read_value;
    use ch14_reading_derive::ReadingValue;

    use crate::Meters;
    use crate::Seconds;

    // type alias 保留 i32 的型別身分；欄位不需要寫 pub。
    type RawReading = i32;

    #[derive(ReadingValue)]
    struct PrivateReading(RawReading);

    #[test]
    fn derived_reading_borrows_original_wrappers() {
        let distance = Meters(0);
        let waiting = Seconds(-5);
        assert_eq!(read_value(&distance), 0);
        assert_eq!(read_value(&waiting), -5);
        assert_eq!(distance, Meters(0));
        assert_eq!(waiting, Seconds(-5));
    }

    #[test]
    fn field_alias_and_visibility_remain_unchanged() {
        let value = PrivateReading(80);
        assert_eq!(read_value(&value), 80);
        assert_eq!(value.0, 80);
    }
}
// --8<-- [end:consumer-tests]
