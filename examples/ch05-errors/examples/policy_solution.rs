use ch05_errors::policy::CountError;
use ch05_errors::policy::checked_count;

fn show_input(input: i32) {
    match checked_count(input) {
        Ok(count) => println!("接受件數={count}"),
        Err(CountError::Negative(value)) => println!("件數 {value} 不能為負數"),
        Err(CountError::TooLarge(value)) => println!("件數 {value} 超過批次上限100"),
    }
}

fn main() {
    for input in [-1, 0, 100, 101] {
        show_input(input);
    }
    assert_eq!(checked_count(0), Ok(0));
    assert_eq!(checked_count(100), Ok(100));
}
