use ch14_macros::hygiene::read_pair_values;
use ch14_macros::read_pair;
use ch14_macros::reading_type;

reading_type!(Meters);
reading_type!(Seconds);

// --8<-- [start:hygiene-observe]
fn main() {
    let distance = Meters(80);
    let waiting = Seconds(30);
    let value = &waiting;
    let ordinary = read_pair_values(&distance, value);
    let generated = read_pair!(&distance, value);
    println!("普通函式={ordinary:?}，macro={generated:?}");
    assert_eq!(generated, (80, 30));
    assert_eq!(ordinary, generated);
    assert_eq!(waiting, Seconds(30));

    let mut calls = 0;
    let observed = read_pair!(
        {
            calls += 1;
            &distance
        },
        {
            calls += 1;
            value
        },
    );
    println!("取得reference的工作執行次數={calls}，讀值={observed:?}");
    assert_eq!(calls, 2);
    assert_eq!(observed, (80, 30));
}
// --8<-- [end:hygiene-observe]

#[cfg(test)]
mod tests {
    use crate::Meters;
    use crate::Seconds;
    use ch14_macros::read_pair;

    #[test]
    fn caller_expression_keeps_its_name_context_after_a_template_local_is_bound() {
        let distance = Meters(80);
        let waiting = Seconds(30);
        let value = &waiting;
        assert_eq!(read_pair!(&distance, value), (80, 30));
        assert_eq!(waiting, Seconds(30));
    }

    #[test]
    fn each_reference_expression_is_evaluated_once_in_left_to_right_order() {
        let distance = Meters(80);
        let waiting = Seconds(30);
        let mut order = Vec::new();
        let values = read_pair!(
            {
                order.push(1);
                &distance
            },
            {
                order.push(2);
                &waiting
            },
        );
        assert_eq!(values, (80, 30));
        assert_eq!(order, vec![1, 2]);
    }
}
