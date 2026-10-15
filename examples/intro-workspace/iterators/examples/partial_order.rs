use std::cmp::Ordering;

fn main() {
    let first: f64 = 3.0;
    let invalid = f64::NAN;
    println!("一般浮點比較={:?}", first.partial_cmp(&4.0));
    println!("NaN 比較={:?}", invalid.partial_cmp(&first));
    assert_eq!(first.partial_cmp(&4.0), Some(Ordering::Less));
    assert_eq!(invalid.partial_cmp(&first), None);
    let second: i32 = 2;
    let third: i32 = 3;
    assert_eq!(second.cmp(&third), Ordering::Less);
}
