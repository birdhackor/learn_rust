pub mod errors;

use crate::errors::read_count;
use crate::errors::report;

fn main() {
    for text in ["3", "abc", "-1"] {
        match read_count(text) {
            Ok(count) => println!("成功件數={count}"),
            Err(error) => println!("{}", report(&error).as_str()),
        }
    }
}
