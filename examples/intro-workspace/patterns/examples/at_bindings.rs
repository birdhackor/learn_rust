fn small_count(count: i32) -> i32 {
    match count {
        actual @ 1..=3 => actual,
        _ => 0,
    }
}

fn main() {
    for count in [0, 1, 3, 4] {
        println!("數量{count}選出={}", small_count(count));
    }
    assert_eq!(small_count(1), 1);
    assert_eq!(small_count(3), 3);
    assert_eq!(small_count(4), 0);
}
