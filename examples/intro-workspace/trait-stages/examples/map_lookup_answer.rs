mod map_support;

use crate::map_support::owned_text;
use std::collections::HashMap;

fn main() {
    let mut quantities: HashMap<String, usize> = HashMap::new();
    assert_eq!(quantities.insert(owned_text("Rust"), 5), None);
    assert_eq!(quantities.insert(owned_text("Python"), 3), None);
    assert_eq!(quantities.insert(owned_text("Rust"), 7), Some(5));
    match quantities.get_mut("Python") {
        Some(quantity) => *quantity += 2,
        None => println!("Python 尚未收錄"),
    }
    assert_eq!(quantities.get("Rust"), Some(&7));
    assert_eq!(quantities.get("Python"), Some(&5));
    assert_eq!(quantities.get("JavaScript"), None);
    assert_eq!(quantities.len(), 2);
    println!(
        "Rust={:?}，Python={:?}",
        quantities.get("Rust"),
        quantities.get("Python")
    );
}
