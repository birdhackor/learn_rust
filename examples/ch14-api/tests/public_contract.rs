use ch14_api::BatchLimit;
use ch14_api::Hit;
use ch14_api::LimitError;
use ch14_api::Options;
use ch14_api::SearchError;
use ch14_api::read_matches;
use ch14_api::search;
use std::error::Error;
use std::io;
use std::path::Path;

#[test]
fn every_constructible_limit_and_the_default_respect_the_boundary() {
    assert_eq!(BatchLimit::new(0), Err(LimitError::Zero));
    assert_eq!(BatchLimit::new(1001), Err(LimitError::TooLarge(1001)));
    assert_eq!(BatchLimit::new(1).expect("最小有效值").get(), 1);
    assert_eq!(BatchLimit::new(1000).expect("最大有效值").get(), 1000);
    assert_eq!(BatchLimit::default().get(), 100);
}

#[test]
fn selection_keeps_original_text_and_line_numbers_with_a_valid_limit() {
    let options = Options::default()
        .with_ignore_case(true)
        .with_limit(BatchLimit::new(2).expect("2在有效範圍內"));
    let hits =
        search("RUST", "Python\r\n\r\nRust\r\nrust\r\nRUST\r\n", &options).expect("query非空");
    assert_eq!(
        hits,
        vec![
            Hit {
                line_number: 3,
                text: "Rust".to_owned(),
            },
            Hit {
                line_number: 4,
                text: "rust".to_owned(),
            },
        ]
    );
    assert_eq!(options.limit().get(), 2);
}

#[test]
fn zero_matches_is_success_and_non_ascii_remains_case_sensitive() {
    let options = Options::default().with_ignore_case(true);
    assert_eq!(
        search("Rust", "Python", &options).expect("query非空").len(),
        0
    );
    let hits = search("Ä", "Ä\nä\n", &options).expect("query非空");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits.as_slice()[0].text.as_str(), "Ä");
}

#[test]
fn invalid_query_has_priority_over_file_access() {
    match read_matches("", Path::new("this-file-is-not-used"), &Options::default()) {
        Err(SearchError::EmptyQuery) => {}
        other => panic!("空query須先拒絕：{other:?}"),
    }
}

#[test]
fn a_public_read_error_keeps_its_typed_cause() {
    let error = SearchError::Read(io::Error::from(io::ErrorKind::PermissionDenied));
    match error.source() {
        Some(_) => {}
        None => panic!("Read必須保留原原因"),
    }
    match error {
        SearchError::Read(cause) => assert_eq!(cause.kind(), io::ErrorKind::PermissionDenied),
        other => panic!("來源應是Read：{other}"),
    }
}

#[test]
fn a_file_result_owns_the_original_matching_lines() {
    let hits = read_matches("Rust", Path::new("fixtures/note.txt"), &Options::default())
        .expect("repo提供有效UTF-8檔案與非空query");
    assert_eq!(hits.len(), 2);
    assert_eq!(hits.as_slice()[0].line_number, 1);
    assert_eq!(hits.as_slice()[1].line_number, 3);
    assert_eq!(hits.as_slice()[1].text.as_str(), "Rust第三行");
}

#[cfg(feature = "text")]
#[test]
fn optional_text_representation_borrows_the_same_data() {
    let hits = search("Rust", "Python\nRust\n", &Options::default()).expect("query非空");
    let text = ch14_api::render_text(hits.as_slice());
    assert_eq!(text.as_str(), "2:Rust\n");
    assert_eq!(hits.as_slice()[0].text.as_str(), "Rust");
}
