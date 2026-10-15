// --8<-- [start:property-imports]
use ch14_property::count_of;
use ch14_property::frequencies;
use ch14_property::total_count;
use proptest::collection::vec;
use proptest::prop_assert;
use proptest::prop_assert_eq;
use proptest::proptest;
use proptest::test_runner::Config;
// --8<-- [end:property-imports]

// --8<-- [start:properties]
proptest! {
    #![proptest_config(Config::with_cases(64))]

    #[test]
    fn grouping_preserves_every_occurrence(values in vec(-3i32..4, 0usize..32)) {
        let counts = frequencies(values.as_slice());
        for count in counts.values() {
            prop_assert!(*count > 0);
        }
        prop_assert_eq!(total_count(&counts), values.len());
    }

    #[test]
    fn each_key_agrees_with_direct_counting(values in vec(-3i32..4, 0usize..32)) {
        let counts = frequencies(values.as_slice());
        for key in -3..4 {
            let mut expected = 0;
            for value in values.as_slice() {
                if *value == key {
                    expected += 1;
                }
            }
            prop_assert_eq!(count_of(&counts, key), expected, "key={}", key);
        }
    }
}
// --8<-- [end:properties]
