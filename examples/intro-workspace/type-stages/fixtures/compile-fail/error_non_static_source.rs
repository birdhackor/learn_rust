use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct BorrowedLeaf<'a> {
    message: &'a str,
}

impl fmt::Display for BorrowedLeaf<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for BorrowedLeaf<'_> {}

#[derive(Debug)]
struct Outer<'a> {
    cause: BorrowedLeaf<'a>,
}

impl fmt::Display for Outer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "外層錯誤")
    }
}

impl Error for Outer<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

fn main() {}
