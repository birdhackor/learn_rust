fn main() {
    let mut numbers = vec![3, 1, 2, 1];
    numbers.as_mut_slice().sort();
    println!("預設整數順序={numbers:?}");

    let mut rows = vec![(2, 20), (1, 30), (1, 10)];
    rows.as_mut_slice().sort_by_key(|row: &(i32, i32)| row.0);
    println!("只按第一欄排序={rows:?}");
    assert_eq!(rows.as_slice()[0].1, 30);
    assert_eq!(rows.as_slice()[1].1, 10);
}
