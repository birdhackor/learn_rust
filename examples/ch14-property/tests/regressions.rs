use ch14_property::count_of;
use ch14_property::frequencies;
use ch14_property::total_count;

// --8<-- [start:zero-regression]
#[test]
fn repeated_zero_is_counted_twice_in_one_group() {
    let values = [0, 0];
    let counts = frequencies(&values[..]);
    assert_eq!(counts.len(), 1);
    assert_eq!(count_of(&counts, 0), 2);
    assert_eq!(total_count(&counts), 2);
}
// --8<-- [end:zero-regression]

#[test]
fn fixed_large_keys_are_valid_outside_the_generated_domain() {
    let values = [-1000, 1000, -1000];
    let counts = frequencies(&values[..]);
    assert_eq!(count_of(&counts, -1000), 2);
    assert_eq!(count_of(&counts, 1000), 1);
    assert_eq!(total_count(&counts), 3);
}

#[test]
fn empty_input_has_no_groups() {
    let values = Vec::<i32>::new();
    let counts = frequencies(values.as_slice());
    assert_eq!(counts.len(), 0);
    assert_eq!(total_count(&counts), 0);
}
