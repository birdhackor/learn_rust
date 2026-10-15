// A4 library 驗收：將整個含條件屬性的 tests module 加到 src/app.rs 末尾。
#[cfg(test)]
mod tests {
    use crate::app::RunError;
    use crate::app::Settings;
    use crate::app::run;
    use crate::app::write_report;
    use crate::collection::RootError;
    use crate::report::Diagnostic;
    use crate::report::Report;
    use crate::report::ReportGroup;
    use crate::report::render_text;
    use std::fs;
    use std::io;
    use std::io::Write;
    use std::str;
    use tempfile::tempdir;

    // --8<-- [start:writer-fixture]
    struct ControlledWriter {
        bytes: Vec<u8>,
        write_fails: bool,
        flush_fails: bool,
        writes: usize,
        flushes: usize,
    }

    impl Write for ControlledWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.writes += 1;
            if self.write_fails {
                return Err(io::Error::from(io::ErrorKind::PermissionDenied));
            }
            for byte in bytes {
                self.bytes.push(*byte);
            }
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            self.flushes += 1;
            if self.flush_fails {
                Err(io::Error::from(io::ErrorKind::PermissionDenied))
            } else {
                Ok(())
            }
        }
    }

    fn writer(write_fails: bool, flush_fails: bool) -> ControlledWriter {
        ControlledWriter {
            bytes: Vec::new(),
            write_fails,
            flush_fails,
            writes: 0,
            flushes: 0,
        }
    }
    // --8<-- [end:writer-fixture]

    fn report(complete: bool) -> Report {
        let scan_issues = if complete {
            Vec::new()
        } else {
            vec![Diagnostic {
                path_debug: None,
                action: String::from("fixture"),
                reason: String::from("受控項目問題"),
            }]
        };
        Report {
            complete,
            duplicate_bytes: 3,
            groups: vec![ReportGroup {
                len: 3,
                paths: vec![String::from("a"), String::from("b")],
                duplicate_bytes: 3,
            }],
            scan_issues,
            file_issues: Vec::new(),
        }
    }

    #[test]
    fn complete_text_and_json_are_written_and_flushed_before_zero() {
        let report = report(true);
        for json in [false, true] {
            let mut output = writer(false, false);
            assert_eq!(
                write_report(&report, json, &mut output).expect("可完成輸出"),
                0
            );
            assert!(output.writes > 0);
            assert_eq!(output.flushes, 1);
            let text = str::from_utf8(output.bytes.as_slice()).expect("renderer 產生 UTF8");
            if json {
                let recovered = serde_json::from_str::<Report>(text).expect("完整 stdout 是 JSON");
                assert_eq!(recovered, report);
            } else {
                assert_eq!(text, render_text(&report).as_str());
            }
        }
    }

    #[test]
    fn a_partial_report_is_still_written_before_one() {
        let report = report(false);
        for json in [false, true] {
            let mut output = writer(false, false);
            assert_eq!(
                write_report(&report, json, &mut output).expect("可寫出部分結果"),
                1
            );
            assert_eq!(output.flushes, 1);
            let text = str::from_utf8(output.bytes.as_slice()).expect("renderer 產生 UTF8");
            if json {
                let recovered = serde_json::from_str::<Report>(text).expect("完整 stdout 是 JSON");
                assert_eq!(recovered, report);
            } else {
                assert_eq!(text, render_text(&report).as_str());
            }
        }
    }

    #[test]
    fn a_write_error_does_not_become_complete_or_partial_success() {
        for complete in [false, true] {
            let mut output = writer(true, false);
            match write_report(&report(complete), false, &mut output) {
                Err(RunError::Output(error)) => {
                    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied)
                }
                other => panic!("未寫成應回 Output：{other:?}"),
            }
            assert!(output.writes > 0);
            assert_eq!(output.bytes.len(), 0);
            assert_eq!(output.flushes, 0);
        }
    }

    // --8<-- [start:flush-test]
    #[test]
    fn a_flush_error_after_accepted_bytes_is_still_fatal() {
        for complete in [false, true] {
            let mut output = writer(false, true);
            match write_report(&report(complete), false, &mut output) {
                Err(RunError::Output(error)) => {
                    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied)
                }
                other => panic!("flush 失敗不能回 0 或 1：{other:?}"),
            }
            assert!(!output.bytes.as_slice().is_empty());
            assert_eq!(output.flushes, 1);
        }
    }
    // --8<-- [end:flush-test]

    #[test]
    fn a_root_failure_preserves_its_type_without_writing_report_data() {
        let directory = tempdir().expect("可建立自有目錄");
        let settings = Settings {
            root: directory.path().join("missing"),
            json: true,
        };
        let mut output = writer(false, false);
        match run(&settings, &mut output) {
            Err(RunError::Root(RootError::Io(error))) => {
                assert_eq!(error.kind(), io::ErrorKind::NotFound)
            }
            other => panic!("缺 root 應是 Root Io：{other:?}"),
        }
        assert_eq!(output.writes, 0);
        assert_eq!(output.flushes, 0);
    }

    #[test]
    fn run_connects_real_files_to_the_same_json_report() {
        let directory = tempdir().expect("可建立自有目錄");
        for (name, text) in [("a", "ABC"), ("b", "ABC"), ("different", "abc")] {
            fs::write(directory.path().join(name).as_path(), text).expect("可寫 fixture");
        }
        let settings = Settings {
            root: directory.path().to_path_buf(),
            json: true,
        };
        let mut output = writer(false, false);
        assert_eq!(run(&settings, &mut output).expect("有效 root 與輸出"), 0);
        let text = str::from_utf8(output.bytes.as_slice()).expect("JSON 是 UTF8");
        let report = serde_json::from_str::<Report>(text).expect("只有一份 JSON");
        assert!(report.complete);
        assert_eq!(report.groups.len(), 1);
        assert_eq!(report.groups.as_slice()[0].paths.len(), 2);
        assert_eq!(report.duplicate_bytes, 3);
        assert_eq!(output.flushes, 1);
    }
}
