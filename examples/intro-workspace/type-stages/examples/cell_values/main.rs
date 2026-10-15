mod cells;

use cells::Counter;
use std::cell::Cell;

fn main() {
    let counter = Counter::new(0);
    println!("shared method第一次計數={}", counter.tick());
    println!("shared method第二次計數={}", counter.tick());

    let mut first = String::new();
    first.push_str("Rust");
    let text = Cell::new(first);
    let mut replacement = String::new();
    replacement.push_str("Rust 教學");
    let old = text.replace(replacement);
    println!("replace取回owned舊文字={}", old.as_str());
    let current = text.into_inner();
    println!("接手Cell內的owned新文字={}", current.as_str());
}
