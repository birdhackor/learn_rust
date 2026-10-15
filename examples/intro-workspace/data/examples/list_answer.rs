fn capped_nonnegative(values: &[i32], ceiling: i32) -> Vec<i32> {
    let mut kept = Vec::new();
    let mut position = 0;
    while position < values.len() {
        let mut value = values[position];
        if value >= 0 {
            if value > ceiling {
                value = ceiling;
            }
            kept.push(value);
        }
        position += 1;
    }
    kept
}

fn main() {
    let values = vec![120, -5, 0, 80];
    let result = capped_nonnegative(values.as_slice(), 100);
    assert!(result.len() == 3);
    assert_eq!(result.as_slice()[0], 100);
    assert_eq!(result.as_slice()[1], 0);
    assert_eq!(result.as_slice()[2], 80);
    assert_eq!(values.as_slice()[0], 120);
    for position in 0..result.len() {
        println!("索引{position}={}", result.as_slice()[position]);
    }
}
