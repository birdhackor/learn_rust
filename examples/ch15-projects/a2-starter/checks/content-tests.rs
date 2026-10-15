// A2 驗收：將整個 mod tests 加到 src/content.rs 末尾。
#[cfg(test)]
mod tests {
    use crate::collection::FileRecord;
    use crate::collection::ScanCause;
    use crate::collection::ScanIssue;
    use crate::collection::ScanReport;
    use crate::collection::observe;
    use crate::collection::scan;
    use crate::content::analyze;
    use crate::content::confirm_bucket;
    use crate::content::digest_candidates;
    use crate::content::refine_bucket;
    use crate::files::FileCause;
    use crate::files::FileIssue;
    use crate::files::equal_files;
    use crate::files::hash_file;
    use crate::files::verify_unchanged;
    use crate::stream::CompareReadError;
    use crate::stream::equal_readers;
    use crate::stream::hash_reader;
    use blake3::Hasher;
    use std::borrow::ToOwned;
    use std::fs;
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::path::Path;
    use tempfile::TempDir;
    use tempfile::tempdir;

    // --8<-- [start:short]
    pub struct ShortReader {
        data: Vec<u8>,
        position: usize,
        max_per_read: usize,
    }

    pub fn short_reader(data: &[u8], max_per_read: usize) -> ShortReader {
        if max_per_read == 0 {
            panic!("測試來源的每次上限必須是正數");
        }
        ShortReader {
            data: data.to_owned(),
            position: 0,
            max_per_read,
        }
    }

    impl Read for ShortReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            let mut copied = 0;
            while copied < buffer.len()
                && copied < self.max_per_read
                && self.position < self.data.len()
            {
                buffer[copied] = self.data.as_slice()[self.position];
                copied += 1;
                self.position += 1;
            }
            Ok(copied)
        }
    }
    // --8<-- [end:short]

    // --8<-- [start:failure]
    pub struct InterruptedThenFailure {
        step: usize,
    }

    pub fn failing_reader() -> InterruptedThenFailure {
        InterruptedThenFailure { step: 0 }
    }

    impl Read for InterruptedThenFailure {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if buffer.is_empty() {
                return Ok(0);
            }
            let step = self.step;
            self.step += 1;
            if step == 0 {
                Err(io::Error::from(io::ErrorKind::Interrupted))
            } else if step == 1 {
                buffer[0] = 82;
                Ok(1)
            } else {
                Err(io::Error::from(io::ErrorKind::PermissionDenied))
            }
        }
    }
    // --8<-- [end:failure]

    // --8<-- [start:fixture]
    fn record(path: &Path) -> FileRecord {
        FileRecord {
            path: path.to_path_buf(),
            snapshot: observe(path).expect("固定材料可取得檔案資料"),
        }
    }

    fn fixture(contents: &[(&str, &str)]) -> (TempDir, Vec<FileRecord>) {
        let directory = tempdir().expect("可建立獨立測試目錄");
        let mut records = Vec::new();
        for entry in contents {
            let path = directory.path().join(entry.0);
            fs::write(path.as_path(), entry.1).expect("可建立固定檔案");
            records.push(record(path.as_path()));
        }
        (directory, records)
    }
    // --8<-- [end:fixture]

    // --8<-- [start:material-test]
    #[test]
    fn material_bytes_form_rust_and_empty_groups_but_keep_lowercase_apart() {
        let (directory, _records) = fixture(&[
            ("a", "Rust\n"),
            ("b", "Rust\n"),
            ("c", "Rust\n"),
            ("different", "rust\n"),
            ("empty-a", ""),
            ("empty-b", ""),
        ]);
        let result = analyze(scan(directory.path()).expect("固定有效 root"));
        assert_eq!(result.groups.len(), 2);
        assert_eq!(result.groups.as_slice()[0].len, 5);
        assert_eq!(result.groups.as_slice()[0].paths.len(), 3);
        assert_eq!(result.groups.as_slice()[1].len, 0);
        assert_eq!(result.groups.as_slice()[1].paths.len(), 2);
        assert_eq!(result.scan_issues.len(), 0);
        assert_eq!(result.file_issues.len(), 0);
    }
    // --8<-- [end:material-test]

    #[test]
    fn incoming_scan_issue_survives_successful_content_confirmation() {
        let (directory, records) = fixture(&[("a", "AAA"), ("b", "AAA")]);
        let missing = directory.path().join("missing");
        let report = ScanReport {
            files: records,
            issues: vec![ScanIssue {
                path: Some(missing.as_path().to_path_buf()),
                action: "受控已有收集問題",
                cause: ScanCause::Io(io::Error::from(io::ErrorKind::NotFound)),
            }],
        };
        let result = analyze(report);
        assert_eq!(result.groups.len(), 1);
        assert_eq!(result.scan_issues.len(), 1);
        assert_eq!(result.scan_issues.as_slice()[0].path, Some(missing));
        assert_eq!(result.file_issues.len(), 0);
    }

    #[test]
    fn size_singleton_does_not_need_hash_work() {
        let (_directory, records) = fixture(&[("a", "R"), ("b", "R"), ("unique", "LONG")]);
        let mut visits = 0;
        let mut issues = Vec::new();
        let buckets = digest_candidates(
            records,
            |record: &FileRecord| {
                visits += 1;
                hash_file(record)
            },
            &mut issues,
        );
        assert_eq!(visits, 2);
        assert_eq!(buckets.len(), 1);
        assert_eq!(issues.len(), 0);
    }

    // --8<-- [start:collision-test]
    #[test]
    fn forced_digest_bucket_has_two_true_groups_and_one_duplicate_group() {
        let (directory, records) = fixture(&[("a", "AAA"), ("b2", "BBB"), ("b1", "BBB")]);
        let forced = Hasher::new().finalize();
        let mut issues = Vec::new();
        let buckets = digest_candidates(records, |_record: &FileRecord| Ok(forced), &mut issues);
        assert_eq!(buckets.len(), 1);
        for (_key, candidates) in buckets {
            let groups = refine_bucket(candidates.as_slice(), equal_files).expect("固定穩定材料");
            assert_eq!(groups.len(), 2);
            assert_eq!(groups.as_slice()[0].members.len(), 1);
            assert_eq!(groups.as_slice()[1].members.len(), 2);
            let duplicates = confirm_bucket(candidates, &mut issues, equal_files);
            assert_eq!(duplicates.len(), 1);
            assert_eq!(
                duplicates.as_slice()[0].paths,
                vec![directory.path().join("b1"), directory.path().join("b2")]
            );
        }
        assert_eq!(issues.len(), 0);
    }
    // --8<-- [end:collision-test]

    #[test]
    fn repeated_native_path_is_not_a_duplicate_pair() {
        let (_directory, mut records) = fixture(&[("a", "AAA")]);
        let same = record(records.as_slice()[0].path.as_path());
        records.push(same);
        let mut issues = Vec::new();
        let groups = confirm_bucket(records, &mut issues, equal_files);
        assert_eq!(groups.len(), 0);
        assert_eq!(issues.len(), 0);
    }

    // --8<-- [start:recovery-test]
    #[test]
    fn left_reader_failure_removes_a_and_recovers_b_pair() {
        let (directory, records) = fixture(&[("a", "AAA"), ("b2", "BBB"), ("b1", "BBB")]);
        let offender = directory.path().join("a");
        let mut issues = Vec::new();
        let groups = confirm_bucket(
            records,
            &mut issues,
            |left: &FileRecord, right: &FileRecord| {
                if left.path == offender {
                    let mut broken = failing_reader();
                    let mut good = short_reader("BBB".as_bytes(), 1);
                    return match equal_readers(&mut broken, &mut good) {
                        Err(CompareReadError::Left(error)) => Err(FileIssue {
                            path: left.path.as_path().to_path_buf(),
                            action: "內容比較讀取",
                            cause: FileCause::Io(error),
                        }),
                        _ => panic!("受控左來源應在重試後失敗"),
                    };
                }
                equal_files(left, right)
            },
        );
        assert_eq!(issues.len(), 1);
        assert_eq!(issues.as_slice()[0].path, offender);
        match &issues.as_slice()[0].cause {
            FileCause::Io(error) => assert_eq!(error.kind(), io::ErrorKind::PermissionDenied),
            _ => panic!("保留實際 reader 失敗原因"),
        }
        assert_eq!(groups.len(), 1);
        assert_eq!(
            groups.as_slice()[0].paths,
            vec![directory.path().join("b1"), directory.path().join("b2")]
        );
    }
    // --8<-- [end:recovery-test]

    #[test]
    fn hash_read_failure_is_retained_and_other_candidates_continue() {
        let (directory, records) = fixture(&[("a", "AAA"), ("b1", "BBB"), ("b2", "BBB")]);
        let offender = directory.path().join("a");
        let mut issues = Vec::new();
        let buckets = digest_candidates(
            records,
            |candidate: &FileRecord| {
                if candidate.path == offender {
                    let mut broken = failing_reader();
                    return match hash_reader(&mut broken) {
                        Err(error) => Err(FileIssue {
                            path: candidate.path.as_path().to_path_buf(),
                            action: "hash 讀取",
                            cause: FileCause::Io(error),
                        }),
                        Ok(_) => panic!("受控來源應在重試後失敗"),
                    };
                }
                hash_file(candidate)
            },
            &mut issues,
        );
        assert_eq!(issues.len(), 1);
        assert_eq!(issues.as_slice()[0].path, offender);
        assert_eq!(buckets.len(), 1);
        for (_key, candidates) in buckets {
            assert_eq!(candidates.len(), 2);
        }
    }

    #[test]
    fn observable_change_is_a_problem_and_does_not_hide_valid_pairs() {
        let (directory, records) = fixture(&[("a", "AAA"), ("b1", "BBB"), ("b2", "BBB")]);
        let offender = directory.path().join("a");
        fs::write(offender.as_path(), "changed length").expect("可控制地改變長度");
        let result = analyze(ScanReport {
            files: records,
            issues: Vec::new(),
        });
        assert_eq!(result.file_issues.len(), 1);
        assert_eq!(result.file_issues.as_slice()[0].path, offender);
        match &result.file_issues.as_slice()[0].cause {
            FileCause::Changed => (),
            _ => panic!("應保留可觀察變動"),
        }
        assert_eq!(result.groups.len(), 1);
        assert_eq!(result.groups.as_slice()[0].paths.len(), 2);
    }

    #[test]
    fn after_read_observation_checks_the_open_handle_and_native_path() {
        let (directory, records) = fixture(&[("a", "AAA")]);
        let original = &records.as_slice()[0];
        let mut file = File::open(original.path.as_path()).expect("可打開固定材料");
        hash_reader(&mut file).expect("可讀完固定材料");
        let path = directory.path().join("a");
        fs::write(path.as_path(), "B").expect("另一項工作可改動已打開的檔案");
        match verify_unchanged(original, &file, "受控讀取後觀察") {
            Err(issue) => {
                assert_eq!(issue.path, path);
                match issue.cause {
                    FileCause::Changed => (),
                    _ => panic!("應保留可觀察變動"),
                }
            }
            Ok(()) => panic!("長度已改變"),
        }
    }

    #[test]
    fn different_positive_short_reads_and_changed_tail_are_distinguished() {
        let bytes: [u8; 9001] = [82; 9001];
        let mut changed: [u8; 9001] = [82; 9001];
        changed[9000] = 83;
        let mut left = short_reader(&bytes[..], 3);
        let mut right = short_reader(&bytes[..], 7);
        assert!(equal_readers(&mut left, &mut right).expect("合法短來源"));
        let mut left = short_reader(&bytes[..], 7);
        let mut right = short_reader(&changed[..], 3);
        assert!(!equal_readers(&mut left, &mut right).expect("尾端不同"));
        let mut left = short_reader(&bytes[..], 7);
        let mut right = short_reader(&bytes[..9000], 3);
        assert!(!equal_readers(&mut left, &mut right).expect("長度不同"));
    }

    #[test]
    fn right_read_failure_is_not_a_normal_different_result() {
        let mut good = short_reader("R".as_bytes(), 1);
        let mut broken = failing_reader();
        match equal_readers(&mut good, &mut broken) {
            Err(CompareReadError::Right(error)) => {
                assert_eq!(error.kind(), io::ErrorKind::PermissionDenied)
            }
            _ => panic!("右來源失敗不是 Ok(false)"),
        }
    }

    #[test]
    fn binary_and_empty_sources_have_valid_digests() {
        for bytes in [vec![255, 0], Vec::new()] {
            let mut once = Hasher::new();
            once.update(bytes.as_slice());
            let mut source = short_reader(bytes.as_slice(), 1);
            assert_eq!(
                hash_reader(&mut source).expect("bytes 不必是文字"),
                once.finalize()
            );
        }
    }
}
