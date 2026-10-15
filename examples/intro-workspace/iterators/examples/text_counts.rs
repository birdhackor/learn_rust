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
        let text = "Rust\nPython\nRust\n\n教學\n".to_owned();
        count_names(text.as_str())
    };
    for name in ["Rust", "Python", "教學"] {
        println!("{name}={:?}", counts.get(name));
    }
    assert_eq!(counts.get("Rust"), Some(&2));
    assert_eq!(counts.get("Python"), Some(&1));
    assert_eq!(counts.get("教學"), Some(&1));
    assert_eq!(counts.get("JavaScript"), None);
    assert_eq!(count_names("").len(), 0);
}
