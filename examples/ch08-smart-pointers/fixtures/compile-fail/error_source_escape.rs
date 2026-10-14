use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct Cause {
    value: i32,
}

impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "原因數值={}", self.value)
    }
}

impl Error for Cause {}

#[derive(Debug)]
struct Outer {
    cause: Cause,
}

impl fmt::Display for Outer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "外層錯誤")
    }
}

impl Error for Outer {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

fn main() {
    let cause = {
        let error = Outer {
            cause: Cause { value: 80 },
        };
        error.source().expect("固定Outer一定保存Cause")
    };
    println!("原error已結束，仍想讀原因={cause}");
}
