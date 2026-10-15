use std::num::ParseIntError;

#[derive(Debug)]
enum CountError {
    Negative(i32),
}

fn nonnegative(text: &str) -> Result<i32, CountError> {
    let value = text.parse::<i32>()?;
    if value < 0 {
        Err(CountError::Negative(value))
    } else {
        Ok(value)
    }
}

fn main() {
    let parsed: Result<i32, ParseIntError> = "3".parse();
    println!("標準解析結果：{parsed:?}");
    match nonnegative("-1") {
        Ok(value) => println!("{value}"),
        Err(CountError::Negative(value)) => println!("負數：{value}"),
    }
}
