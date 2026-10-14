use ch03_data::list::keep_nonnegative;

fn main() {
    let input = [120, -5, 0, 80];
    let kept = keep_nonnegative(&input[..]);
    println!("保留的項目={}", kept.len());
    for position in 0..kept.len() {
        println!("索引{position}={}", kept.as_slice()[position]);
    }
    assert_eq!(input[1], -5);
    assert_eq!(kept.len(), 3);
    assert_eq!(kept.as_slice()[0], 120);
    assert_eq!(kept.as_slice()[1], 0);
    assert_eq!(kept.as_slice()[2], 80);
}
