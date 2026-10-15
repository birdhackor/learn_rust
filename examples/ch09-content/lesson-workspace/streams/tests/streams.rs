use blake3::Hasher;
use lesson_content_streams::CompareReadError;
use lesson_content_streams::equal_readers;
use lesson_content_streams::hash_reader;
use lesson_content_streams::readers::failing_reader;
use lesson_content_streams::readers::short_reader;
use std::io;

#[test]
fn hash_depends_on_bytes_in_order_and_not_positive_short_read_sizes() {
    let data: [u8; 9001] = [82; 9001];
    let mut once = Hasher::new();
    once.update(&data[..]);
    let expected = once.finalize();
    for maximum in [1, 3, 7, 9000] {
        let mut reader = short_reader(&data[..], maximum);
        assert_eq!(hash_reader(&mut reader).expect("合法短讀取"), expected);
    }
}

#[test]
fn empty_and_non_utf8_bytes_are_valid_hash_inputs() {
    let empty = Vec::<u8>::new();
    let mut reader = short_reader(empty.as_slice(), 1);
    assert_eq!(
        hash_reader(&mut reader).expect("空來源成功"),
        Hasher::new().finalize()
    );
    let data: [u8; 2] = [255, 0];
    let mut once = Hasher::new();
    once.update(&data[..]);
    let mut reader = short_reader(&data[..], 1);
    assert_eq!(
        hash_reader(&mut reader).expect("bytes 不需 UTF-8"),
        once.finalize()
    );
}

#[test]
fn equality_normalizes_unrelated_short_read_boundaries_across_buffers() {
    let data: [u8; 9001] = [82; 9001];
    for left_maximum in [1, 3, 5000] {
        for right_maximum in [2, 7, 6000] {
            let mut left = short_reader(&data[..], left_maximum);
            let mut right = short_reader(&data[..], right_maximum);
            assert!(equal_readers(&mut left, &mut right).expect("合法短讀取"));
        }
    }
}

#[test]
fn same_length_changes_and_a_missing_final_byte_are_different() {
    let original: [u8; 9001] = [82; 9001];
    let mut changed: [u8; 9001] = [82; 9001];
    changed[9000] = 83;
    let mut left = short_reader(&original[..], 3);
    let mut right = short_reader(&changed[..], 7);
    assert!(!equal_readers(&mut left, &mut right).expect("合法來源"));
    let mut left = short_reader(&original[..], 7);
    let mut shorter = short_reader(&original[..9000], 3);
    assert!(!equal_readers(&mut left, &mut shorter).expect("合法來源"));
}

#[test]
fn empty_equal_readers_and_one_empty_reader_have_different_answers() {
    let empty = Vec::<u8>::new();
    let mut left = short_reader(empty.as_slice(), 1);
    let mut right = short_reader(empty.as_slice(), 7);
    assert!(equal_readers(&mut left, &mut right).expect("空來源合法"));
    let mut left = short_reader(empty.as_slice(), 1);
    let mut right = short_reader("R".as_bytes(), 1);
    assert!(!equal_readers(&mut left, &mut right).expect("來源合法"));
}

#[test]
fn interrupted_is_retried_and_later_failure_is_not_partial_success() {
    let mut reader = failing_reader();
    match hash_reader(&mut reader) {
        Err(error) => assert_eq!(error.kind(), io::ErrorKind::PermissionDenied),
        Ok(_) => panic!("第一個 byte 不代表完整成功"),
    }
    let mut left = failing_reader();
    let mut right = short_reader("R".as_bytes(), 1);
    match equal_readers(&mut left, &mut right) {
        Err(CompareReadError::Left(error)) => {
            assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        }
        _ => panic!("應保留左來源後續失敗"),
    }
    let mut left = short_reader("R".as_bytes(), 1);
    let mut right = failing_reader();
    match equal_readers(&mut left, &mut right) {
        Err(CompareReadError::Right(error)) => {
            assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        }
        _ => panic!("應保留右來源後續失敗"),
    }
}
