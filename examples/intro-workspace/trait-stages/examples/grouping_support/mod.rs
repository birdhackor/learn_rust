use std::collections::HashMap;

// --8<-- [start:count-labels]
pub fn count_labels(labels: &[String]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for label in labels {
        match counts.get_mut(label.as_str()) {
            Some(count) => *count += 1,
            None => {
                counts.insert((*label).clone(), 1);
            }
        }
    }
    counts
}
// --8<-- [end:count-labels]

// --8<-- [start:counts-tests]
#[cfg(test)]
mod tests {
    use crate::grouping_support::count_labels;
    use crate::map_support::owned_text;

    #[test]
    fn repeated_labels_share_a_count() {
        let labels = vec![owned_text("Rust"), owned_text("Python"), owned_text("Rust")];
        let counts = count_labels(labels.as_slice());
        assert_eq!(counts.len(), 2);
        assert_eq!(counts.get("Rust"), Some(&2));
        assert_eq!(counts.get("Python"), Some(&1));
        assert_eq!(counts.get("JavaScript"), None);
        assert_eq!(labels.len(), 3);
    }

    #[test]
    fn empty_input_has_no_groups() {
        let labels: Vec<String> = Vec::new();
        let counts = count_labels(labels.as_slice());
        assert_eq!(counts.len(), 0);
        assert_eq!(counts.get("Rust"), None);
    }

    #[test]
    fn result_owns_keys_after_input_ends() {
        let counts = {
            let labels = vec![owned_text("Rust"), owned_text("Rust")];
            count_labels(labels.as_slice())
        };
        assert_eq!(counts.get("Rust"), Some(&2));
    }
}
// --8<-- [end:counts-tests]
