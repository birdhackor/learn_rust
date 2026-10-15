pub mod contracts;

use crate::contracts::split_i32_at_mut;

fn main() {
    let mut values = [80, 120, 40, 60];
    {
        let (left, right) = split_i32_at_mut(&mut values, 3).expect("3在四項數列的範圍內");
        assert_eq!(left.len(), 3);
        assert_eq!(right.len(), 1);
        left[2] += 20;
        right[0] += 20;
    }
    assert_eq!(values, [80, 120, 60, 80]);
    println!("切分後改到原數列={values:?}");

    let len = values.len();
    let (left, right) = split_i32_at_mut(&mut values, len).expect("len是合法端點");
    assert_eq!(left.len(), 4);
    assert_eq!(right.len(), 0);
    println!(
        "切在len時，left有{}項、right有{}項",
        left.len(),
        right.len()
    );
}
