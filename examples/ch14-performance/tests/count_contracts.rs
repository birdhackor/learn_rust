use ch14_performance::count_borrowed;
use ch14_performance::count_cloned;
use ch14_performance::make_workload;

// --8<-- [start:contracts]
fn assert_counts(query: &str, lines: &[String], expected: usize) {
    assert_eq!(
        count_borrowed(query, lines).expect("已知搜尋文字非空"),
        expected
    );
    assert_eq!(
        count_cloned(query, lines).expect("已知搜尋文字非空"),
        expected
    );
}

#[test]
fn fragments_count_each_matching_line_once_and_keep_case() {
    let lines = vec![
        String::from("Rust Rust"),
        String::from("rust"),
        String::from("中有 Rust"),
        String::from("Trust"),
        String::from(""),
    ];
    assert_counts("Rust", lines.as_slice(), 2);
    assert_counts("rust", lines.as_slice(), 2);
    assert_counts("Python", lines.as_slice(), 0);
}

#[test]
fn empty_input_is_valid_but_empty_query_is_rejected() {
    let lines = Vec::<String>::new();
    assert_counts("Rust", lines.as_slice(), 0);
    match count_borrowed("", lines.as_slice()) {
        Err(message) => assert_eq!(message.as_str(), "搜尋文字不能是空字串"),
        Ok(_) => panic!("空query必須拒絕"),
    }
    match count_cloned("", lines.as_slice()) {
        Err(message) => assert_eq!(message.as_str(), "搜尋文字不能是空字串"),
        Ok(_) => panic!("空query必須拒絕"),
    }
}

#[test]
fn measured_workloads_have_known_sizes_and_hit_counts() {
    for rows in [64usize, 256] {
        let lines = make_workload(rows);
        assert_eq!(lines.len(), rows);
        assert_counts("Rust", lines.as_slice(), rows / 4);
    }
}
// --8<-- [end:contracts]
