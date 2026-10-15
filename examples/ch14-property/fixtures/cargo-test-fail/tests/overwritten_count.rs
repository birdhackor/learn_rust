use ch14_property::total_count;
use proptest::collection::vec;
use proptest::prop_assert_eq;
use proptest::proptest;
use proptest::strategy::Just;
use proptest::test_runner::Config;
use proptest::test_runner::RngSeed;
use std::collections::HashMap;

// --8<-- [start:broken-counter]
fn overwritten_count(values: &[i32]) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    for value in values {
        counts.insert(*value, 1);
    }
    counts
}
// --8<-- [end:broken-counter]

// --8<-- [start:replay-config]
fn replay_config() -> Config {
    let mut config = Config::with_cases(64);
    config.rng_seed = RngSeed::Fixed(20261014);
    config.failure_persistence = None;
    config
}
// --8<-- [end:replay-config]

// --8<-- [start:failing-property]
proptest! {
    #![proptest_config(replay_config())]

    #[test]
    fn overwritten_values_lose_occurrences(values in vec(Just(0i32), 0usize..32)) {
        let counts = overwritten_count(values.as_slice());
        prop_assert_eq!(total_count(&counts), values.len());
    }
}
// --8<-- [end:failing-property]
