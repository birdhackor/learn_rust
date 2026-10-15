// --8<-- [start:fold]
#[allow(clippy::unnecessary_fold)]
fn scaled_sum(values: &[i32], minimum: i32, factor: i32) -> i32 {
    values
        .iter()
        .filter(|item: &&i32| **item >= minimum)
        .map(|item: &i32| *item * factor)
        .fold(0, |total: i32, value: i32| total + value)
}
// --8<-- [end:fold]

fn main() {
    let values = vec![-2, 0, 3, 3];
    let total = scaled_sum(values.as_slice(), 0, 10);
    println!("篩選、轉換後的合計={total}");
    assert_eq!(total, 60);
    assert_eq!(scaled_sum(&[], 0, 10), 0);
    assert_eq!(scaled_sum(values.as_slice(), 4, 10), 0);
}
