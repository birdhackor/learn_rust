use std::cmp::Ordering;
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

fn rare_first(text: &str) -> Vec<(String, usize)> {
    let counts = count_names(text);
    let mut rows: Vec<(String, usize)> = counts.into_iter().collect();
    rows.as_mut_slice()
        .sort_by(
            |left: &(String, usize), right: &(String, usize)| match left.1.cmp(&right.1) {
                Ordering::Equal => left.0.as_str().cmp(right.0.as_str()),
                order => order,
            },
        );
    rows
}

fn main() {
    let rows = rare_first("Rust\nPython\nRust\n教學\n");
    assert_eq!(rows.as_slice()[0], ("Python".to_owned(), 1));
    assert_eq!(rows.as_slice()[1], ("教學".to_owned(), 1));
    assert_eq!(rows.as_slice()[2], ("Rust".to_owned(), 2));
    assert_eq!(rare_first("").len(), 0);
    for (name, count) in rows {
        println!("{}={count}", name.as_str());
    }
}
