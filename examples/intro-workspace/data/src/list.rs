// --8<-- [start:list-rule]
pub fn keep_nonnegative(values: &[i32]) -> Vec<i32> {
    let mut kept = Vec::new();
    let mut position = 0;
    while position < values.len() {
        let value = values[position];
        if value >= 0 {
            kept.push(value);
        }
        position += 1;
    }
    kept
}
// --8<-- [end:list-rule]

// --8<-- [start:list-tests]
#[cfg(test)]
mod tests {
    use crate::list::keep_nonnegative;

    #[test]
    fn keeps_order_duplicates_and_zero() {
        let input = [80, -5, 80, 0];
        let result = keep_nonnegative(&input[..]);
        assert!(result.len() == 3);
        assert_eq!(result.as_slice()[0], 80);
        assert_eq!(result.as_slice()[1], 80);
        assert_eq!(result.as_slice()[2], 0);
        assert_eq!(input[1], -5);
    }

    #[test]
    fn empty_and_all_negative_inputs_produce_no_items() {
        let empty: [i32; 0] = [];
        let negative = [-5, -1];
        let empty_length = keep_nonnegative(&empty[..]).len();
        assert!(empty_length == 0);
        let negative_length = keep_nonnegative(&negative[..]).len();
        assert!(negative_length == 0);
    }
}
// --8<-- [end:list-tests]
