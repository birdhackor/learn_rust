mod map_support;

use crate::map_support::owned_text;
use std::collections::HashMap;

fn main() {
    // --8<-- [start:new-insert]
    let mut counts: HashMap<String, usize> = HashMap::new();
    let rust = owned_text("Rust");
    match counts.insert(rust, 1) {
        Some(old) => println!("取代舊次數={old}"),
        None => println!("第一次加入，沒有舊次數"),
    }
    // --8<-- [end:new-insert]
    // --8<-- [start:replace]
    match counts.insert(owned_text("Rust"), 2) {
        Some(old) => println!("取代舊次數={old}"),
        None => println!("第一次加入，沒有舊次數"),
    }
    counts.insert(owned_text("Python"), 1);

    // --8<-- [end:replace]

    // --8<-- [start:query]
    let mut query = owned_text("Rust");
    let found = counts.get(query.as_str());
    query.push_str(" 課程");
    match found {
        Some(count) => println!("查到 Rust 次數={}", *count),
        None => println!("Rust 尚未收錄"),
    }
    println!("查詢文字改成={}", query.as_str());
    // --8<-- [end:query]
    // --8<-- [start:missing]
    match counts.get("JavaScript") {
        Some(count) => println!("JavaScript 次數={}", *count),
        None => println!("JavaScript 尚未收錄"),
    }
    println!("不同名稱數={}", counts.len());
    assert_eq!(counts.len(), 2);
    // --8<-- [end:missing]
}
