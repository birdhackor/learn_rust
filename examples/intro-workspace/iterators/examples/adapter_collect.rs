use std::iter::FromIterator;

struct NumberList {
    values: Vec<i32>,
}

impl IntoIterator for NumberList {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

// --8<-- [start:organize]
fn keep_and_scale(values: &[i32], minimum: i32, factor: i32) -> Vec<i32> {
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

fn main() {
    let values = vec![-2, 0, 3, 3];
    let result = keep_and_scale(values.as_slice(), 0, 10);
    println!("整理結果={result:?}");
    assert_eq!(result, vec![0, 30, 30]);

    let by_type = (1..4).collect::<Vec<i32>>();
    let by_method = Vec::<i32>::from_iter(1..4);
    assert_eq!(by_type, by_method);
    let custom: NumberList = (1..4).collect();
    println!("相同 Item，自訂目的型別={:?}", custom.values);
    assert_eq!(keep_and_scale(&[], 0, 10), vec![]);
    assert_eq!(values, vec![-2, 0, 3, 3]);
}
