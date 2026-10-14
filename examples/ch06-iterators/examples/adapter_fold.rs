use ch06_iterators::adapters::scaled_sum;

fn main() {
    let values = vec![-2, 0, 3, 3];
    let total = scaled_sum(values.as_slice(), 0, 10);
    println!("篩選、轉換後的合計={total}");
    assert_eq!(total, 60);
}
