use std::fmt;

struct LimitError {
    value: i32,
    maximum: i32,
}

impl fmt::Display for LimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "件數 {} 超過上限 {}", self.value, self.maximum)
    }
}

fn main() {
    let error = LimitError {
        value: 101,
        maximum: 100,
    };
    let message = format!("{error}");
    assert_eq!(message.as_str(), "件數 101 超過上限 100");
    println!("{message}");
}
