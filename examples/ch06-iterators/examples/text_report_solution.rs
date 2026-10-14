use ch06_iterators::text_ops::count_names;
use std::cmp::Ordering;

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
