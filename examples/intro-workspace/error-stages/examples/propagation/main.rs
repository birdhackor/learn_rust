pub mod aliases;
pub mod conversions;
pub mod formatting;
pub mod parsing;
pub mod policy;
pub mod propagation;

use crate::propagation::parse_count;
use crate::propagation::parse_count_manual;

fn main() {
    for text in ["0", "3", "100", "abc", "-1", "101"] {
        let manual = parse_count_manual(text);
        let short = parse_count(text);
        assert_eq!(manual, short);

        match short {
            Ok(count) => println!("{text}：件數 {}", count.value()),
            Err(error) => println!("{text}：{error}"),
        }
    }
    println!("呼叫者已決定報告失敗並繼續下一筆");
}
