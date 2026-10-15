pub mod policy;

use crate::policy::checked_count;

fn main() {
    for input in [-1, 0, 3, 100, 101] {
        match checked_count(input) {
            Ok(count) => println!("接受件數={count}"),
            Err(error) => println!("拒絕這筆，原因={error:?}"),
        }
    }
    println!("所有資料都已檢查");
}
