use blake3::Hash;
use blake3::Hasher;
use ch09_files::scan::FileRecord;
use ch09_files::scan::ScanCause;
use ch09_files::scan::observe;
use lesson_checked_files::FileCause;
use lesson_checked_files::equal_files;
use lesson_checked_files::hash_file;
use lesson_checked_files::verify_unchanged;
use lesson_content_groups::MetricError;
use lesson_content_groups::add_duplicate_bytes;
use lesson_content_groups::duplicate_bytes;
use lesson_content_groups::refine_bucket;
use lesson_content_streams::hash_reader;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use tempfile::tempdir;

fn record(path: &Path) -> FileRecord {
    FileRecord {
        path: path.to_path_buf(),
        snapshot: observe(path).expect("固定材料可取得 metadata 與修改時間"),
    }
}

#[test]
fn group_order_is_determined_by_paths_instead_of_candidate_arrival_order() {
    let directory = tempdir().expect("可建立測試目錄");
    let mut records = Vec::new();
    for (name, contents) in [("z", "ZZZ"), ("a", "AAA"), ("b", "AAA")] {
        let path = directory.path().join(name);
        fs::write(path.as_path(), contents).expect("可寫材料");
        records.push(record(path.as_path()));
    }
    let groups = refine_bucket(records.as_slice()).expect("固定穩定材料");
    assert_eq!(groups.len(), 2);
    assert_eq!(groups.as_slice()[0].members.len(), 2);
    assert_eq!(
        groups.as_slice()[0].members.as_slice()[0].path,
        directory.path().join("a")
    );
    assert_eq!(
        groups.as_slice()[1].members.as_slice()[0].path,
        directory.path().join("z")
    );
}

#[test]
fn empty_bucket_returns_zero_groups() {
    let records = Vec::new();
    assert_eq!(
        refine_bucket(records.as_slice())
            .expect("空候選正常完成")
            .len(),
        0
    );
}

#[test]
fn real_files_have_equal_hashes_and_are_confirmed_by_bytes() {
    let directory = tempdir().expect("可建立測試目錄");
    let first = directory.path().join("a");
    let second = directory.path().join("b");
    let different = directory.path().join("c");
    fs::write(first.as_path(), "AAA").expect("可寫材料");
    fs::write(second.as_path(), "AAA").expect("可寫材料");
    fs::write(different.as_path(), "BBB").expect("可寫材料");
    let first = record(first.as_path());
    let second = record(second.as_path());
    let different = record(different.as_path());
    assert_eq!(
        hash_file(&first).expect("穩定可讀"),
        hash_file(&second).expect("穩定可讀")
    );
    assert!(equal_files(&first, &second).expect("穩定可讀"));
    assert!(!equal_files(&first, &different).expect("同長不同 bytes"));
}

#[test]
fn observable_change_before_hash_or_compare_names_the_exact_offender() {
    let directory = tempdir().expect("可建立測試目錄");
    let first = directory.path().join("a");
    let second = directory.path().join("b");
    fs::write(first.as_path(), "AAA").expect("可寫材料");
    fs::write(second.as_path(), "AAA").expect("可寫材料");
    let first_record = record(first.as_path());
    let second_record = record(second.as_path());
    fs::write(second.as_path(), "changed length").expect("可控制地改變長度");
    let hash_failure = match hash_file(&second_record) {
        Err(failure) => failure,
        Ok(_) => panic!("第二份記錄已變動"),
    };
    let compare_failure = match equal_files(&first_record, &second_record) {
        Err(failure) => failure,
        Ok(_) => panic!("第二份記錄已變動"),
    };
    for failure in [hash_failure, compare_failure] {
        assert_eq!(failure.path, second);
        match failure.cause {
            FileCause::Changed => (),
            _ => panic!("應保留可觀察變動"),
        }
    }
}

#[test]
fn post_read_observation_detects_controlled_change_while_file_is_open() {
    let directory = tempdir().expect("可建立測試目錄");
    let path = directory.path().join("a");
    fs::write(path.as_path(), "AAA").expect("可寫材料");
    let before = record(path.as_path());
    let mut file = File::open(path.as_path()).expect("可打開材料");
    hash_reader(&mut file).expect("可讀完整 bytes");
    fs::write(path.as_path(), "B").expect("打開 File 沒有禁止另一項寫入工作");
    let failure = match verify_unchanged(&before, &file, "受控讀取後觀察") {
        Err(failure) => failure,
        Ok(()) => panic!("長度已改變"),
    };
    assert_eq!(failure.path, path);
    assert_eq!(failure.action, "受控讀取後觀察");
    match failure.cause {
        FileCause::Changed => (),
        _ => panic!("應保留可觀察變動"),
    }
}

#[test]
fn forced_same_digest_a_b_b_is_refined_into_two_actual_content_groups() {
    let directory = tempdir().expect("可建立測試目錄");
    let forced = Hasher::new().finalize();
    let mut buckets: HashMap<Hash, Vec<FileRecord>> = HashMap::new();
    for (name, contents) in [("a", "AAA"), ("b2", "BBB"), ("b1", "BBB")] {
        let path = directory.path().join(name);
        fs::write(path.as_path(), contents).expect("可寫材料");
        let record = record(path.as_path());
        match buckets.get_mut(&forced) {
            Some(bucket) => bucket.push(record),
            None => {
                buckets.insert(forced, vec![record]);
            }
        }
    }
    let candidates = buckets.get(&forced).expect("固定 digest 的候選桶");
    let groups = refine_bucket(candidates.as_slice()).expect("固定材料可確認");
    assert_eq!(groups.len(), 2);
    assert_eq!(groups.as_slice()[0].members.len(), 1);
    assert_eq!(groups.as_slice()[1].members.len(), 2);
    assert_eq!(
        groups.as_slice()[1].members.as_slice()[0].path,
        directory.path().join("b1")
    );
    assert_eq!(
        groups.as_slice()[1].members.as_slice()[1].path,
        directory.path().join("b2")
    );
}

#[test]
fn repeated_same_native_path_is_not_counted_as_two_files() {
    let directory = tempdir().expect("可建立測試目錄");
    let path = directory.path().join("a");
    fs::write(path.as_path(), "AAA").expect("可寫材料");
    let candidates = vec![record(path.as_path()), record(path.as_path())];
    let groups = refine_bucket(candidates.as_slice()).expect("固定材料可確認");
    assert_eq!(groups.len(), 1);
    assert_eq!(groups.as_slice()[0].members.len(), 1);
}

#[test]
fn failed_offender_can_be_removed_without_losing_other_valid_groups() {
    let directory = tempdir().expect("可建立測試目錄");
    let first = directory.path().join("a");
    let second = directory.path().join("b");
    let third = directory.path().join("c");
    fs::write(first.as_path(), "AAA").expect("可寫材料");
    fs::write(second.as_path(), "BBB").expect("可寫材料");
    fs::write(third.as_path(), "BBB").expect("可寫材料");
    let valid_first = record(first.as_path());
    let missing = directory.path().join("missing");
    let missing_record = FileRecord {
        path: missing.as_path().to_path_buf(),
        snapshot: valid_first.snapshot,
    };
    let candidates = vec![
        valid_first,
        missing_record,
        record(second.as_path()),
        record(third.as_path()),
    ];
    let failure = match refine_bucket(candidates.as_slice()) {
        Err(failure) => failure,
        Ok(_) => panic!("不存在的候選必須有錯誤"),
    };
    assert_eq!(failure.path, missing);
    match &failure.cause {
        FileCause::Observation(ScanCause::Io(error)) => {
            assert_eq!(error.kind(), io::ErrorKind::NotFound);
        }
        _ => panic!("應保留原始不存在原因"),
    }
    let remaining: Vec<FileRecord> = candidates
        .into_iter()
        .filter(|candidate: &FileRecord| candidate.path != failure.path)
        .collect();
    let groups = refine_bucket(remaining.as_slice()).expect("移除確切失敗者後仍可重建有效群組");
    assert_eq!(groups.len(), 2);
    assert_eq!(groups.as_slice()[1].members.len(), 2);
}

#[test]
fn count_metrics_preserve_zero_cases_and_reject_overflow() {
    assert_eq!(duplicate_bytes(3, 2).expect("小數字"), 3);
    assert_eq!(duplicate_bytes(3, 0).expect("零路徑"), 0);
    assert_eq!(duplicate_bytes(3, 1).expect("單一路徑"), 0);
    assert_eq!(duplicate_bytes(0, 5).expect("空內容"), 0);
    assert_eq!(add_duplicate_bytes(3, 6).expect("小數字"), 9);
    match duplicate_bytes(u64::MAX, 3) {
        Err(MetricError::Overflow) => (),
        _ => panic!("乘法溢位不能回報正常量"),
    }
    match add_duplicate_bytes(u64::MAX, 1) {
        Err(MetricError::Overflow) => (),
        _ => panic!("加法溢位不能回報正常量"),
    }
}

#[cfg(unix)]
#[test]
fn hard_link_paths_are_counted_by_path_and_not_physical_reclaimable_space() {
    let directory = tempdir().expect("可建立測試目錄");
    let first = directory.path().join("a");
    let second = directory.path().join("b");
    fs::write(first.as_path(), "AAA").expect("可寫材料");
    fs::hard_link(first.as_path(), second.as_path()).expect("同檔案系統可建立 hard link");
    let candidates = vec![record(first.as_path()), record(second.as_path())];
    let groups = refine_bucket(candidates.as_slice()).expect("內容相同");
    assert_eq!(groups.len(), 1);
    let count = groups.as_slice()[0].members.len();
    assert_eq!(count, 2);
    assert_eq!(duplicate_bytes(3, count).expect("兩個path"), 3);
}
