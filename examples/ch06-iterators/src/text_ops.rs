use std::cmp::Ordering;
use std::collections::HashMap;

// --8<-- [start:lines]
pub fn nonempty_names(text: &str) -> impl Iterator<Item = &str> {
    text.lines().filter(|line: &&str| !(*line).is_empty())
}
// --8<-- [end:lines]

// --8<-- [start:counts]
pub fn count_names(text: &str) -> HashMap<String, usize> {
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
pub fn sort_counts(counts: HashMap<String, usize>) -> Vec<(String, usize)> {
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
pub fn report(text: &str) -> Vec<(String, usize)> {
    sort_counts(count_names(text))
}
// --8<-- [end:report]

#[cfg(test)]
mod tests {
    use crate::text_ops::count_names;
    use crate::text_ops::nonempty_names;
    use crate::text_ops::report;
    use crate::text_ops::sort_counts;
    use std::collections::HashMap;

    #[test]
    fn line_endings_and_empty_lines_follow_the_named_contract() {
        let names: Vec<&str> = nonempty_names("Rust\r\n\nPython\n").collect();
        assert_eq!(names, vec!["Rust", "Python"]);
        let lone_cr: Vec<&str> = nonempty_names("Rust\r").collect();
        assert_eq!(lone_cr, vec!["Rust\r"]);
        assert_eq!(nonempty_names("").next(), None);
    }

    #[test]
    fn names_preserve_case_spaces_and_distinct_keys() {
        let counts = count_names("Rust\nRust\nrust\nRust \n\n");
        assert_eq!(counts.get("Rust"), Some(&2));
        assert_eq!(counts.get("rust"), Some(&1));
        assert_eq!(counts.get("Rust "), Some(&1));
        assert_eq!(counts.get(""), None);
        assert_eq!(counts.len(), 3);
    }

    #[test]
    fn count_and_name_comparison_overrides_map_insertion_order() {
        let mut first = HashMap::new();
        first.insert("Rust".to_owned(), 2);
        first.insert("教學".to_owned(), 1);
        first.insert("Python".to_owned(), 1);
        let mut second = HashMap::new();
        second.insert("Python".to_owned(), 1);
        second.insert("Rust".to_owned(), 2);
        second.insert("教學".to_owned(), 1);
        let expected = vec![
            ("Rust".to_owned(), 2),
            ("Python".to_owned(), 1),
            ("教學".to_owned(), 1),
        ];
        assert_eq!(sort_counts(first), expected);
        assert_eq!(sort_counts(second), expected);
    }

    #[test]
    fn report_owns_names_after_source_ends_and_handles_empty_input() {
        let rows = {
            let text = "Rust\nPython\nRust\n\n教學\n".to_owned();
            report(text.as_str())
        };
        assert_eq!(rows.as_slice()[0], ("Rust".to_owned(), 2));
        assert_eq!(rows.len(), 3);
        assert_eq!(report("").len(), 0);
    }
}
