use crate::streams::NumberList;
use std::iter::FromIterator;
use std::num::ParseIntError;

// --8<-- [start:organize]
pub fn keep_and_scale(values: &[i32], minimum: i32, factor: i32) -> Vec<i32> {
    values
        .iter()
        .filter(|item: &&i32| **item >= minimum)
        .map(|item: &i32| *item * factor)
        .collect()
}
// --8<-- [end:organize]

// --8<-- [start:from-iterator]
impl FromIterator<i32> for NumberList {
    fn from_iter<S: IntoIterator<Item = i32>>(source: S) -> Self {
        let mut values = Vec::new();
        for value in source {
            values.push(value);
        }
        Self { values }
    }
}
// --8<-- [end:from-iterator]

// --8<-- [start:fold]
pub fn scaled_sum(values: &[i32], minimum: i32, factor: i32) -> i32 {
    values
        .iter()
        .filter(|item: &&i32| **item >= minimum)
        .map(|item: &i32| *item * factor)
        .fold(0, |total: i32, value: i32| total + value)
}
// --8<-- [end:fold]

// --8<-- [start:parse-all]
pub fn parse_all(texts: &[String]) -> Result<Vec<i32>, ParseIntError> {
    texts
        .iter()
        .map(|text: &String| text.as_str().parse::<i32>())
        .collect()
}
// --8<-- [end:parse-all]

#[cfg(test)]
mod tests {
    use crate::adapters::keep_and_scale;
    use crate::adapters::parse_all;
    use crate::adapters::scaled_sum;
    use crate::streams::NumberList;
    use std::num::ParseIntError;

    #[test]
    fn organization_preserves_zero_order_duplicates_and_input() {
        let values = vec![-2, 0, 3, 3];
        assert_eq!(keep_and_scale(values.as_slice(), 0, 10), vec![0, 30, 30]);
        assert_eq!(values, vec![-2, 0, 3, 3]);
        assert_eq!(scaled_sum(values.as_slice(), 0, 10), 60);
        assert_eq!(scaled_sum(values.as_slice(), 4, 10), 0);
    }

    #[test]
    fn empty_inputs_preserve_empty_collection_and_initial_accumulator() {
        let values: Vec<i32> = Vec::new();
        assert_eq!(keep_and_scale(values.as_slice(), 0, 10).len(), 0);
        assert_eq!(scaled_sum(values.as_slice(), 0, 10), 0);
        let texts: Vec<String> = Vec::new();
        assert_eq!(parse_all(texts.as_slice()), Ok(vec![]));
    }

    #[test]
    fn a_custom_destination_collects_the_same_numeric_items() {
        let saved: NumberList = (1..4).collect();
        assert_eq!(saved.values, vec![1, 2, 3]);
    }

    #[test]
    fn result_collection_stops_at_the_first_error() {
        let texts = vec!["10".to_owned(), "bad".to_owned(), "30".to_owned()];
        let mut visits = 0;
        let result: Result<Vec<i32>, ParseIntError> = texts
            .as_slice()
            .iter()
            .map(|text: &String| {
                visits += 1;
                text.as_str().parse::<i32>()
            })
            .collect();
        let first_error = match "bad".parse::<i32>() {
            Err(error) => error,
            Ok(value) => panic!("固定文字不應解析成 {value}"),
        };
        assert_eq!(result, Err(first_error));
        assert_eq!(visits, 2);
        assert_eq!(texts.len(), 3);
    }

    #[test]
    fn valid_texts_collect_owned_numbers_and_keep_the_source() {
        let texts = vec!["0".to_owned(), "+3".to_owned(), "-2".to_owned()];
        assert_eq!(parse_all(texts.as_slice()), Ok(vec![0, 3, -2]));
        assert_eq!(texts.as_slice()[1].as_str(), "+3");
    }
}
