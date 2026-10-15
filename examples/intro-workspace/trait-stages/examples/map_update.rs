mod map_support;

use crate::map_support::owned_text;
use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<String, usize> = HashMap::new();
    counts.insert(owned_text("Rust"), 2);
    match counts.get_mut("Rust") {
        Some(count) => *count += 1,
        None => println!("Rust 尚未收錄"),
    }
    assert_eq!(counts.get("Rust"), Some(&3));
    println!("更新後={:?}", counts.get("Rust"));
    match counts.get_mut("JavaScript") {
        Some(count) => *count += 1,
        None => println!("JavaScript 尚未收錄，沒有新增"),
    }
    assert_eq!(counts.len(), 1);
}
