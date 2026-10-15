use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let count = counts.entry("Rust".to_owned()).or_insert(1);
    counts.entry("Python".to_owned()).or_insert(1);
    println!("第一個 Entry 的值={}", *count);
}
