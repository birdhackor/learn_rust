use ch04_traits::counts::owned_text;
use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let rust = owned_text("Rust");
    match counts.insert(rust, 1) {
        Some(old) => println!("取代舊次數={old}"),
        None => println!("第一次加入，沒有舊次數"),
    }
    match counts.insert(owned_text("Rust"), 2) {
        Some(old) => println!("取代舊次數={old}"),
        None => println!("第一次加入，沒有舊次數"),
    }
    counts.insert(owned_text("Python"), 1);

    let mut query = owned_text("Rust");
    let found = counts.get(query.as_str());
    query.push_str(" 課程");
    match found {
        Some(count) => println!("查到 Rust 次數={}", *count),
        None => println!("Rust 尚未收錄"),
    }
    println!("查詢文字改成={}", query.as_str());
    match counts.get("JavaScript") {
        Some(count) => println!("JavaScript 次數={}", *count),
        None => println!("JavaScript 尚未收錄"),
    }
    println!("不同名稱數={}", counts.len());
    assert_eq!(counts.len(), 2);
}
