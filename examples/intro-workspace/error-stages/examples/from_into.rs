use std::convert::From;
use std::convert::Into;

fn main() {
    let count: i32 = 3;
    let by_from = i64::from(count);
    let by_into: i64 = count.into();

    assert_eq!(by_from, 3);
    assert_eq!(by_into, 3);
    println!("From：{by_from}");
    println!("Into：{by_into}");
    println!("原本的 i32：{count}");
}
