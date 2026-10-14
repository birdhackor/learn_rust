use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut rust = String::new();
    rust.push_str("Rust");
    counts.insert(rust, 2);
    let found = counts.get("Rust");
    let mut python = String::new();
    python.push_str("Python");
    counts.insert(python, 1);
    if let Some(count) = found {
        println!("{}", *count);
    }
}
