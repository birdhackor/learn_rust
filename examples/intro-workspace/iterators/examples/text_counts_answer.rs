use std::collections::HashMap;

// --8<-- [start:lines]
fn nonempty_names(text: &str) -> impl Iterator<Item = &str> {
    text.lines().filter(|line: &&str| !(*line).is_empty())
}
// --8<-- [end:lines]

// --8<-- [start:counts]
fn count_names(text: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for name in nonempty_names(text) {
        counts
            .entry(name.to_owned())
            .and_modify(|count: &mut usize| *count += 1)
            .or_insert(1);
    }
    counts
}
// --8<-- [end:counts]

fn main() {
    let counts = {
        let text = "Rust\nRust\nrust\nRust \n\n".to_owned();
        count_names(text.as_str())
    };
    assert_eq!(counts.get("Rust"), Some(&2));
    assert_eq!(counts.get("rust"), Some(&1));
    assert_eq!(counts.get("Rust "), Some(&1));
    assert_eq!(counts.get(""), None);
    assert_eq!(counts.len(), 3);
    println!("Rust={:?}", counts.get("Rust"));
    println!("rust={:?}", counts.get("rust"));
    println!("Rust 加尾端空白={:?}", counts.get("Rust "));

    let lone_cr = count_names("Rust\r");
    assert_eq!(lone_cr.get("Rust\r"), Some(&1));
    assert_eq!(lone_cr.get("Rust"), None);
    assert_eq!(count_names("").len(), 0);
}
