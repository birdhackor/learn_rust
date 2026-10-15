fn selected_count(count: i32, ready: bool) -> i32 {
    match count {
        actual @ 1..=3 if ready => actual,
        4 | 5 if ready => 3,
        _ => 0,
    }
}

fn main() {
    for (count, ready) in [(2, true), (4, true), (4, false), (0, true)] {
        println!(
            "數量{count}，備妥{ready}，選出{}",
            selected_count(count, ready)
        );
    }
    assert_eq!(selected_count(2, true), 2);
    assert_eq!(selected_count(4, true), 3);
    assert_eq!(selected_count(4, false), 0);
    assert_eq!(selected_count(0, true), 0);
}
