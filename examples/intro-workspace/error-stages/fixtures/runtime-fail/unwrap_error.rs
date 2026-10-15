#[derive(Debug)]
enum CountError {
    TooLarge(i32),
}

fn checked_count(value: i32) -> Result<i32, CountError> {
    if value > 100 { Err(CountError::TooLarge(value)) } else { Ok(value) }
}

fn main() {
    let result = checked_count(101);
    match &result {
        Ok(count) => println!("待取的件數={}", *count),
        Err(CountError::TooLarge(value)) => println!("被拒絕的件數={}", *value),
    }
    result.unwrap();
}
