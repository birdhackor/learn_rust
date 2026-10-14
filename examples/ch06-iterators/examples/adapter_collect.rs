use ch06_iterators::adapters::keep_and_scale;
use ch06_iterators::streams::NumberList;
use std::iter::FromIterator;

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
}
