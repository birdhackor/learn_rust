use ch14_macros::read_value;
use ch14_macros::reading_type;

reading_type!(Meters);
reading_type!(Seconds);

macro_rules! read_pair_reverse {
    ($left:expr, $right:expr $(,)?) => {{
        let left = $left;
        let right = $right;
        (read_value(right), read_value(left))
    }};
}

fn main() {
    let distance = Meters(80);
    let waiting = Seconds(30);
    let mut order = Vec::new();
    let values = read_pair_reverse!(
        {
            order.push(1);
            &distance
        },
        {
            order.push(2);
            &waiting
        },
    );
    assert_eq!(values, (30, 80));
    assert_eq!(order, vec![1, 2]);
    assert_eq!(distance, Meters(80));
    assert_eq!(waiting, Seconds(30));
    println!("先取得左再取得右：{order:?}；結果反過來：{values:?}");
}
