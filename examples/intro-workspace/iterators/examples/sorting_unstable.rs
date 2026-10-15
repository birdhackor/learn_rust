fn main() {
    let mut numbers = vec![3, 1, 2, 1];
    numbers.as_mut_slice().sort_unstable();
    println!("不要求等值原序，整數結果={numbers:?}");
    assert_eq!(numbers, vec![1, 1, 2, 3]);
}
