use std::fmt;

type CountList = Vec<i32>;

impl fmt::Display for CountList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}項", self.len())
    }
}

fn main() {}
