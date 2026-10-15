use ch14_property::count_of;
use ch14_property::frequencies;
use proptest::collection::vec;
use proptest::prop_assert_eq;
use proptest::proptest;
use proptest::test_runner::Config;

proptest! {
    #![proptest_config(Config::with_cases(64))]

    #[test]
    fn joining_inputs_adds_each_keys_occurrences(
        left in vec(-3i32..4, 0usize..32),
        right in vec(-3i32..4, 0usize..32),
    ) {
        let mut joined = Vec::new();
        for value in left.as_slice() {
            joined.push(*value);
        }
        for value in right.as_slice() {
            joined.push(*value);
        }
        let left_counts = frequencies(left.as_slice());
        let right_counts = frequencies(right.as_slice());
        let joined_counts = frequencies(joined.as_slice());
        for key in -3..4 {
            prop_assert_eq!(
                count_of(&joined_counts, key),
                count_of(&left_counts, key) + count_of(&right_counts, key),
            );
        }
    }
}
