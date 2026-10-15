pub mod errors;

use crate::errors::CountError;
use crate::errors::report;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct OrderError {
    id: i32,
    cause: CountError,
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "訂單{}的件數檢查失敗", self.id)
    }
}

impl Error for OrderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

fn main() {
    let error = OrderError {
        id: 7,
        cause: CountError::TooLarge(101),
    };
    let text = report(&error);
    assert_eq!(
        text.as_str(),
        "處理失敗：訂單7的件數檢查失敗\n原因：件數 101 超過批次上限100"
    );
    println!("{}", text.as_str());
}
