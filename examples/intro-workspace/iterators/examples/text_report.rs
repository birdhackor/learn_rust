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

// --8<-- [start:sort-counts]
fn sort_counts(counts: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut rows: Vec<(String, usize)> = counts.into_iter().collect();
    rows.as_mut_slice()
        .sort_by(
            |left: &(String, usize), right: &(String, usize)| match right.1.cmp(&left.1) {
                Ordering::Equal => left.0.as_str().cmp(right.0.as_str()),
                order => order,
            },
        );
    rows
}
// --8<-- [end:sort-counts]

// --8<-- [start:report]
fn report(text: &str) -> Vec<(String, usize)> {
    sort_counts(count_names(text))
}
// --8<-- [end:report]

fn main() {
    let mut text = String::new();
    text.push_str("Rust\nPython\nRust\n\n教學\n");
    let rows = report(text.as_str());
    for (name, count) in rows {
        println!("{}={count}", name.as_str());
    }
    assert_eq!(text.as_str().lines().collect::<Vec<&str>>().len(), 5);
    let mut first = HashMap::new();
    first.insert("Rust".to_owned(), 2);
    first.insert("教學".to_owned(), 1);
    first.insert("Python".to_owned(), 1);
    let mut second = HashMap::new();
    second.insert("Python".to_owned(), 1);
    second.insert("Rust".to_owned(), 2);
    second.insert("教學".to_owned(), 1);
    let first_rows = sort_counts(first);
    let second_rows = sort_counts(second);
    assert_eq!(first_rows, second_rows);
    assert_eq!(first_rows.as_slice()[0], ("Rust".to_owned(), 2));
    assert_eq!(first_rows.as_slice()[1], ("Python".to_owned(), 1));
    assert_eq!(first_rows.as_slice()[2], ("教學".to_owned(), 1));
    let saved = {
        let source = "Rust\nRust\n".to_owned();
        report(source.as_str())
    };
    assert_eq!(saved.as_slice()[0], ("Rust".to_owned(), 2));
    assert_eq!(report("").len(), 0);
}
