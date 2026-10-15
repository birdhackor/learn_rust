pub mod aliases;
pub mod conversions;
pub mod formatting;
pub mod parsing;
pub mod policy;
pub mod propagation;

use crate::conversions::Count;
use crate::parsing::ParseCountError;
use crate::policy::CountError;
use crate::propagation::parse_count;

fn count_pair(first: &str, second: &str) -> Result<(Count, Count), ParseCountError> {
    let first = parse_count(first)?;
    let second = parse_count(second)?;
    Ok((first, second))
}

fn main() {
    let (first, second) = count_pair("0", "100").expect("固定文字表示兩個有效件數");
    assert_eq!(first.value(), 0);
    assert_eq!(second.value(), 100);

    match count_pair("abc", "101") {
        Err(ParseCountError::Integer(_)) => println!("第一筆解析失敗，整個函式已回傳"),
        _ => panic!("預期第一筆整數解析失敗"),
    }

    match count_pair("3", "101") {
        Err(ParseCountError::Range(CountError::TooLarge(value))) => assert_eq!(value, 101),
        _ => panic!("預期第二筆超過件數上限"),
    }
    println!("兩筆都成功才回傳 tuple，失敗保留原原因");
}
