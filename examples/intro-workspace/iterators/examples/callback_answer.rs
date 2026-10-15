fn sum_with<F: FnMut(i32) -> i32>(values: &[i32], mut task: F) -> i32 {
    let mut total = 0;
    let mut index = 0;
    while index < values.len() {
        total += task(values[index]);
        index += 1;
    }
    total
}

fn main() {
    let factor = 10;
    let mut visits = 0;
    let values = [1, 2, 3];
    let total = sum_with(&values[..], |value| {
        visits += 1;
        value * factor
    });
    println!("轉換後合計={total}，處理次數={visits}");
    assert_eq!(total, 60);
    assert_eq!(visits, 3);
    assert_eq!(sum_with(&values[0..0], |value| value * 10), 0);
}
