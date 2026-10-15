pub mod errors;

use crate::errors::CountError;
use std::error::Error;

fn main() {
    let error = CountError::Negative(-1);
    println!("Debug檢查={error:?}");
    println!("Display訊息={error}");
    match error.source() {
        Some(cause) => println!("下層原因={cause}"),
        None => println!("這個錯誤已是最後原因"),
    }
}
