use ch05_errors::conversions::Count;
use ch05_errors::policy::CountError;

fn count_value(value: i32) -> Result<i32, CountError> {
    match Count::try_from(value) {
        Ok(count) => Ok(i32::from(count)),
        Err(error) => Err(error),
    }
}

fn main() {
    assert_eq!(count_value(0), Ok(0));
    assert_eq!(count_value(100), Ok(100));
    assert_eq!(count_value(-1), Err(CountError::Negative(-1)));
    assert_eq!(count_value(101), Err(CountError::TooLarge(101)));
    println!("轉換往返保留有效件數，也保留失敗原因");
}
