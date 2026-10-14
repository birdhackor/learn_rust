enum CountError {
    Rejected,
}

fn main() {
    let result: Result<i32, CountError> = Err(CountError::Rejected);
    result.unwrap();
}
