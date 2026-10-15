use ch14_profiling::repeat_counts;

#[test]
fn both_modes_repeat_the_same_case_sensitive_line_answer() {
    let lines = vec![
        String::from("Rust Rust"),
        String::from("Python"),
        String::from("rust"),
        String::from("中有 Rust"),
    ];
    assert_eq!(repeat_counts(lines.as_slice(), true, 3), 6);
    assert_eq!(repeat_counts(lines.as_slice(), false, 3), 6);
}

#[test]
fn no_work_is_not_confused_with_one_successful_iteration() {
    let lines = vec![String::from("Rust")];
    assert_eq!(repeat_counts(lines.as_slice(), true, 0), 0);
    assert_eq!(repeat_counts(lines.as_slice(), false, 0), 0);
    let empty = Vec::<String>::new();
    assert_eq!(repeat_counts(empty.as_slice(), true, 3), 0);
    assert_eq!(repeat_counts(empty.as_slice(), false, 3), 0);
}
