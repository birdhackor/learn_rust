use crate::collection::FileRecord;
use crate::collection::ScanReport;
use crate::collection::collect_entries;
use crate::collection::observe;
use crate::content::ContentReport;
use crate::content::DuplicateGroup;
use crate::content::analyze;
use crate::content::confirm_bucket;
use crate::content::digest_candidates;
use crate::content::refine_bucket;
use crate::files::FileCause;
use crate::files::FileIssue;
use crate::files::equal_files;
use crate::metrics::MetricError;
use crate::report::Diagnostic;
use crate::report::ReportError;
use crate::report::build_report;
use crate::report::order_issues;
use crate::stream::CompareReadError;
use crate::stream::equal_readers;
use crate::stream::hash_reader;
use crate::write_report;
use blake3::Hasher;
use std::borrow::ToOwned;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;
use tempfile::tempdir;
use walkdir::WalkDir;

fn fixture(items: &[(&str, &str)]) -> (TempDir, Vec<FileRecord>) {
    let directory = tempdir().expect("可建立自有 fixture");
    let mut records = Vec::new();
    for entry in items {
        let path = directory.path().join(entry.0);
        fs::write(path.as_path(), entry.1).expect("可寫固定材料");
        records.push(FileRecord {
            path: path.as_path().to_path_buf(),
            snapshot: observe(path.as_path()).expect("可觀察材料"),
        });
    }
    (directory, records)
}

struct Source<'a> {
    data: &'a [u8],
    position: usize,
    width: usize,
    interrupted: bool,
    fail_at: Option<usize>,
}
fn reader(data: &[u8], width: usize) -> Source<'_> {
    Source {
        data,
        position: 0,
        width,
        interrupted: false,
        fail_at: None,
    }
}
impl Read for Source<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if self.interrupted {
            self.interrupted = false;
            return Err(io::Error::from(io::ErrorKind::Interrupted));
        }
        let failed = match &self.fail_at {
            Some(limit) => self.position >= *limit,
            None => false,
        };
        if failed {
            return Err(io::Error::from(io::ErrorKind::PermissionDenied));
        }
        let mut count = 0;
        while count < buffer.len() && count < self.width && self.position < self.data.len() {
            buffer[count] = self.data[self.position];
            count += 1;
            self.position += 1;
        }
        Ok(count)
    }
}
fn failing_reader() -> Source<'static> {
    let mut source = reader("RR".as_bytes(), 1);
    source.interrupted = true;
    source.fail_at = Some(1);
    source
}

#[test]
fn forced_digest_bucket_preserves_multiple_true_groups() {
    let (_owner, mut records) = fixture(
        &[
            ("a", "AAA"),
            ("b", "BBB"),
            ("c", "BBB"),
            ("d", "CCC"),
            ("e", "CCC"),
        ][..],
    );
    let repeated = FileRecord {
        path: records.as_slice()[0].path.as_path().to_path_buf(),
        snapshot: records.as_slice()[0].snapshot,
    };
    records.push(repeated);
    assert_eq!(
        refine_bucket(records.as_slice(), equal_files)
            .expect("可讀固定資料")
            .len(),
        3
    );
    let forced = Hasher::new().finalize();
    let mut issues = Vec::new();
    let buckets = digest_candidates(records, |_record| Ok(forced), &mut issues);
    assert_eq!(buckets.len(), 1);
    for (_key, candidates) in buckets {
        let duplicates = confirm_bucket(candidates, &mut issues, equal_files);
        assert_eq!(duplicates.len(), 2);
        for group in duplicates {
            assert_eq!(group.paths.len(), 2);
        }
    }
    assert_eq!(issues.len(), 0);
}

#[test]
fn failed_left_reader_is_removed_and_remaining_b_pair_is_rebuilt() {
    let (owner, records) = fixture(&[("a", "AAA"), ("b", "BBB"), ("c", "BBB")][..]);
    let failed = owner.path().join("a");
    let mut issues = Vec::new();
    let groups = confirm_bucket(records, &mut issues, |left, right| {
        if left.path == failed {
            let mut a = failing_reader();
            let mut b = reader("BBB".as_bytes(), 2);
            match equal_readers(&mut a, &mut b) {
                Err(CompareReadError::Left(error)) => Err(FileIssue {
                    path: left.path.as_path().to_path_buf(),
                    action: "受控比較",
                    cause: FileCause::Io(error),
                }),
                other => panic!("受控左來源應失敗：{other:?}"),
            }
        } else {
            equal_files(left, right)
        }
    });
    assert_eq!(groups.len(), 1);
    assert_eq!(groups.as_slice()[0].paths.len(), 2);
    assert_eq!(issues.len(), 1);
    assert_eq!(issues.as_slice()[0].path, failed);
    match &issues.as_slice()[0].cause {
        FileCause::Io(error) => assert_eq!(error.kind(), io::ErrorKind::PermissionDenied),
        other => panic!("保留原讀取錯誤：{other:?}"),
    }
}

#[test]
fn short_reads_interrupted_errors_binary_empty_and_tail_differences() {
    let mut bytes: [u8; 9001] = [7; 9001];
    bytes[0] = 255;
    assert!(
        equal_readers(&mut reader(&bytes[..], 3), &mut reader(&bytes[..], 7))
            .expect("不同正數短讀")
    );
    assert!(
        !equal_readers(&mut reader(&bytes[..], 3), &mut reader(&bytes[..9000], 7)).expect("缺尾端")
    );
    let mut changed = bytes[..].to_owned();
    changed.as_mut_slice()[9000] = 8;
    assert!(
        !equal_readers(
            &mut reader(&bytes[..], 3),
            &mut reader(changed.as_slice(), 7)
        )
        .expect("改尾端")
    );
    for data in ["".as_bytes(), &bytes[..]] {
        let mut expected = Hasher::new();
        expected.update(data);
        assert_eq!(
            hash_reader(&mut reader(data, 3)).expect("可讀 bytes"),
            expected.finalize()
        );
    }
    match equal_readers(&mut reader("ABC".as_bytes(), 3), &mut failing_reader()) {
        Err(CompareReadError::Right(error)) => {
            assert_eq!(error.kind(), io::ErrorKind::PermissionDenied)
        }
        other => panic!("右邊錯誤不能當作不同：{other:?}"),
    }
}

#[test]
fn observed_change_and_scan_failure_preserve_partial_success() {
    let (owner, records) = fixture(&[("a", "AAA"), ("b", "BBB"), ("c", "BBB")][..]);
    fs::write(owner.path().join("a").as_path(), "changed length").expect("可控制觀察差異");
    let changed = analyze(ScanReport {
        files: records,
        issues: Vec::new(),
    });
    assert_eq!(changed.groups.len(), 1);
    assert_eq!(changed.file_issues.len(), 1);
    match changed.file_issues.as_slice()[0].cause {
        FileCause::Changed => (),
        _ => panic!("保留觀察變動"),
    }
    let mut items = Vec::new();
    for name in ["b", "missing", "c"] {
        let path = owner.path().join(name);
        match WalkDir::new(path.as_path()).into_iter().next() {
            Some(item) => items.push(item),
            None => panic!("單一位置應有一筆 walker item"),
        }
    }
    let partial = build_report(analyze(collect_entries(items))).expect("問題不阻止報告");
    assert!(!partial.complete);
    assert_eq!(partial.groups.len(), 1);
    assert_eq!(partial.scan_issues.len(), 1);
}

#[test]
fn after_read_observation_retains_the_native_offender() {
    let (owner, records) = fixture(&[("a", "AAA")][..]);
    let original = &records.as_slice()[0];
    let mut file = File::open(original.path.as_path()).expect("可開固定資料");
    hash_reader(&mut file).expect("讀到原內容");
    let path = owner.path().join("a");
    fs::write(path.as_path(), "B").expect("在讀後控制長度改變");
    match crate::files::verify_unchanged(original, &file, "讀後觀察") {
        Err(issue) => {
            assert_eq!(issue.path, path);
            match issue.cause {
                FileCause::Changed => (),
                _ => panic!("應保存變動"),
            }
        }
        Ok(()) => panic!("已觀察到變動"),
    }
}

#[test]
fn size_singleton_skips_hash_and_hash_failure_does_not_stop_others() {
    let (owner, records) = fixture(
        &[
            ("single", "longer"),
            ("a", "AAA"),
            ("b", "BBB"),
            ("c", "BBB"),
        ][..],
    );
    let failed = owner.path().join("a");
    let mut calls = 0;
    let mut issues = Vec::new();
    let buckets = digest_candidates(
        records,
        |record| {
            calls += 1;
            if record.path == failed {
                Err(FileIssue {
                    path: record.path.as_path().to_path_buf(),
                    action: "受控摘要",
                    cause: FileCause::Io(io::Error::from(io::ErrorKind::PermissionDenied)),
                })
            } else {
                crate::files::hash_file(record)
            }
        },
        &mut issues,
    );
    assert_eq!(calls, 3);
    assert_eq!(issues.len(), 1);
    let mut groups = Vec::new();
    for (_key, candidates) in buckets {
        for group in confirm_bucket(candidates, &mut issues, equal_files) {
            groups.push(group);
        }
    }
    let report = build_report(ContentReport {
        groups,
        scan_issues: Vec::new(),
        file_issues: issues,
    })
    .expect("可交部分成果");
    assert!(!report.complete);
    assert_eq!(report.groups.len(), 1);
    assert_eq!(report.file_issues.len(), 1);
}

fn group(len: u64, names: &[&str]) -> DuplicateGroup {
    let paths = names
        .iter()
        .map(|name| Path::new(*name).to_path_buf())
        .collect();
    DuplicateGroup { len, paths }
}
fn content(groups: Vec<DuplicateGroup>) -> ContentReport {
    ContentReport {
        groups,
        scan_issues: Vec::new(),
        file_issues: Vec::new(),
    }
}
fn diagnostic(text: &str) -> Diagnostic {
    Diagnostic {
        path_debug: None,
        action: String::from(text),
        reason: String::from("cause"),
    }
}

fn row(path: &str, action: &str, reason: &str, debug: &str) -> crate::report::IssueRow {
    (
        Some(Path::new(path).to_path_buf()),
        Diagnostic {
            path_debug: Some(String::from(debug)),
            action: String::from(action),
            reason: String::from(reason),
        },
    )
}

#[test]
fn native_order_complete_ties_control_characters_and_overflow() {
    let report = build_report(content(vec![
        group(0, &["z", "a.txt"][..]),
        group(5, &["a/b", "a/c", "a/d"][..]),
    ]))
    .expect("小型模型");
    assert_eq!(
        report.groups.as_slice()[0].paths.as_slice()[0].as_str(),
        "a/b"
    );
    assert_eq!(report.duplicate_bytes, 10);
    let make_rows = || {
        vec![
            row("a", "B", "A", "first"),
            row("a", "A", "B", "first"),
            row("a", "A", "A", "second"),
            row("a", "A", "A", "first"),
            (None, diagnostic("Z")),
        ]
    };
    let first = order_issues(make_rows());
    let mut rows = make_rows();
    let mut reversed = Vec::new();
    while !rows.as_slice().is_empty() {
        match rows.pop() {
            Some(row) => reversed.push(row),
            None => panic!("已確認非空"),
        }
    }
    assert_eq!(first, order_issues(reversed));
    assert_eq!(first.as_slice()[0].action.as_str(), "Z");
    assert_eq!(first.as_slice()[1].path_debug, Some(String::from("first")));
    let mut name = String::from("換行");
    name.push('\n');
    name.push('"');
    let escaped = build_report(content(vec![group(1, &[name.as_str(), "plain"][..])]))
        .expect("合法Unicode模型");
    let text = crate::render_text(&escaped);
    assert_eq!(text.as_str().lines().fold(0, |n, _| n + 1), 6);
    assert!(text.contains(format!("{name:?}").as_str()));
    let encoded = crate::render_json(&escaped).expect("可編碼");
    assert_eq!(
        serde_json::from_str::<crate::Report>(encoded.as_str()).expect("可讀回"),
        escaped
    );
    for groups in [
        vec![group(u64::MAX, &["a", "b", "c"][..])],
        vec![group(u64::MAX, &["a", "b"][..]), group(1, &["c", "d"][..])],
    ] {
        match build_report(content(groups)) {
            Err(ReportError::Metric(MetricError::Overflow)) => (),
            other => panic!("溢位不能給正常報告：{other:?}"),
        }
    }
}

struct Output {
    bytes: Vec<u8>,
    reject_write: bool,
    reject_flush: bool,
    flushes: usize,
}
impl Write for Output {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.reject_write {
            return Err(io::Error::from(io::ErrorKind::PermissionDenied));
        }
        for byte in bytes {
            self.bytes.push(*byte);
        }
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.flushes += 1;
        if self.reject_flush {
            Err(io::Error::from(io::ErrorKind::PermissionDenied))
        } else {
            Ok(())
        }
    }
}
#[test]
fn writing_or_flushing_failure_overrides_complete_or_partial_status() {
    let mut report = build_report(content(Vec::new())).expect("完整零結果");
    for complete in [false, true] {
        report.complete = complete;
        report.scan_issues = if complete {
            Vec::new()
        } else {
            vec![diagnostic("受控問題")]
        };
        for json in [false, true] {
            for (reject_write, reject_flush) in [(false, false), (true, false), (false, true)] {
                let mut output = Output {
                    bytes: Vec::new(),
                    reject_write,
                    reject_flush,
                    flushes: 0,
                };
                match write_report(&report, json, &mut output) {
                    Ok(status) => {
                        assert!(!reject_write && !reject_flush);
                        assert_eq!(status, if complete { 0 } else { 1 });
                        assert_eq!(output.flushes, 1);
                    }
                    Err(crate::RunError::Output(error)) => {
                        assert!(reject_write || reject_flush);
                        assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
                        if reject_flush {
                            assert!(!output.bytes.as_slice().is_empty());
                            assert_eq!(output.flushes, 1);
                        }
                    }
                    other => panic!("只允許受控輸出結果：{other:?}"),
                }
            }
        }
    }
}
