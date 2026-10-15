pub mod conversions;
pub mod formatting;
pub mod parsing;
pub mod policy;

use crate::conversions::Count;
use std::str::FromStr;

fn main() {
    let direct = Count::from_str("3").expect("固定文字表示有效件數");
    let through_parse = "3".parse::<Count>().expect("固定文字表示有效件數");
    assert_eq!(direct, through_parse);
    println!("兩種寫法都取得件數 {}", direct.value());

    for text in ["0", "100", "abc", "-1", "101"] {
        match text.parse::<Count>() {
            Ok(count) => println!("{text}：有效件數 {}", count.value()),
            Err(error) => println!("{text}：{error}"),
        }
    }

    let mut owned_text = String::new();
    owned_text.push_str("+3");
    let parsed = owned_text.as_str().parse::<Count>();
    assert_eq!(parsed, Ok(direct));
    println!("原文字仍可使用：{}", owned_text.as_str());
}
