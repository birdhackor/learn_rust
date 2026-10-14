use std::error::Error;
use std::fmt;

struct Failure {
    value: i32,
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "數值{}失敗", self.value)
    }
}

impl Error for Failure {}

fn main() {}
