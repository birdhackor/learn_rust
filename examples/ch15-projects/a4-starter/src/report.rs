use crate::collection::ScanIssue;
use crate::content::ContentReport;
use crate::files::FileIssue;
use crate::metrics::MetricError;
use crate::metrics::add_duplicate_bytes;
use crate::metrics::duplicate_bytes;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;
use std::fmt;
use std::path::PathBuf;

// --8<-- [start:models]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportGroup {
    pub len: u64,
    pub paths: Vec<String>,
    pub duplicate_bytes: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Diagnostic {
    pub path_debug: Option<String>,
    pub action: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub complete: bool,
    pub duplicate_bytes: u64,
    pub groups: Vec<ReportGroup>,
    pub scan_issues: Vec<Diagnostic>,
    pub file_issues: Vec<Diagnostic>,
}

pub type IssueRow = (Option<PathBuf>, Diagnostic);

#[derive(Debug)]
pub enum ReportError {
    Metric(MetricError),
    NonUnicode(PathBuf),
}

impl fmt::Display for ReportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Metric(MetricError::Count(_)) => {
                write!(formatter, "路徑數無法表示為 u64")
            }
            Self::Metric(MetricError::Overflow) => {
                write!(formatter, "按路徑的重複內容量超出 u64 範圍")
            }
            Self::NonUnicode(path) => {
                write!(formatter, "重複群路徑不能表示為 Unicode：{path:?}")
            }
        }
    }
}

impl From<MetricError> for ReportError {
    fn from(error: MetricError) -> Self {
        Self::Metric(error)
    }
}
// --8<-- [end:models]

// --8<-- [start:build]
pub fn build_report(content: ContentReport) -> Result<Report, ReportError> {
    let mut rows: Vec<(PathBuf, ReportGroup)> = Vec::new();
    let mut total = 0;
    for mut group in content.groups {
        if group.paths.len() < 2 {
            continue;
        }
        group.paths.as_mut_slice().sort();
        let first_path = group.paths.as_slice()[0].as_path().to_path_buf();
        let bytes = duplicate_bytes(group.len, group.paths.len())?;
        total = add_duplicate_bytes(total, bytes)?;
        let mut paths = Vec::new();
        for path in group.paths {
            match path.to_str() {
                Some(text) => paths.push(String::from(text)),
                None => return Err(ReportError::NonUnicode(path)),
            }
        }
        rows.push((
            first_path,
            ReportGroup {
                len: group.len,
                paths,
                duplicate_bytes: bytes,
            },
        ));
    }
    rows.as_mut_slice().sort_by(
        |left: &(PathBuf, ReportGroup), right: &(PathBuf, ReportGroup)| {
            left.0.as_path().cmp(right.0.as_path())
        },
    );
    let groups = rows.into_iter().map(|(_path, group)| group).collect();
    let scan_issues = order_issues(scan_rows(content.scan_issues));
    let file_issues = order_issues(file_rows(content.file_issues));
    Ok(Report {
        complete: scan_issues.as_slice().is_empty() && file_issues.as_slice().is_empty(),
        duplicate_bytes: total,
        groups,
        scan_issues,
        file_issues,
    })
}
// --8<-- [end:build]

// --8<-- [start:issue-rows]
pub fn scan_rows(issues: Vec<ScanIssue>) -> Vec<IssueRow> {
    let mut rows = Vec::new();
    for issue in issues {
        let path_debug = issue.path.as_ref().map(|path| format!("{path:?}"));
        let diagnostic = Diagnostic {
            path_debug,
            action: String::from(issue.action),
            reason: format!("{}", issue.cause),
        };
        rows.push((issue.path, diagnostic));
    }
    rows
}

pub fn file_rows(issues: Vec<FileIssue>) -> Vec<IssueRow> {
    let mut rows = Vec::new();
    for issue in issues {
        let diagnostic = Diagnostic {
            path_debug: Some(format!("{:?}", issue.path)),
            action: String::from(issue.action),
            reason: format!("{}", issue.cause),
        };
        rows.push((Some(issue.path), diagnostic));
    }
    rows
}
// --8<-- [end:issue-rows]

// --8<-- [start:order]
pub fn order_issues(mut rows: Vec<IssueRow>) -> Vec<Diagnostic> {
    rows.as_mut_slice().sort_by(
        |left: &IssueRow, right: &IssueRow| match left.0.cmp(&right.0) {
            Ordering::Equal => match left.1.action.as_str().cmp(right.1.action.as_str()) {
                Ordering::Equal => match left.1.reason.as_str().cmp(right.1.reason.as_str()) {
                    Ordering::Equal => left.1.path_debug.cmp(&right.1.path_debug),
                    order => order,
                },
                order => order,
            },
            order => order,
        },
    );
    rows.into_iter()
        .map(|(_path, diagnostic)| diagnostic)
        .collect()
}
// --8<-- [end:order]

// --8<-- [start:text]
pub fn render_text(report: &Report) -> String {
    let mut text = format!(
        "掃描完整={}\n按路徑的重複內容={} bytes\n重複群組={}\n",
        report.complete,
        report.duplicate_bytes,
        report.groups.len()
    );
    for group in &report.groups {
        text.push_str(
            format!(
                "群組：{} bytes，{} 個路徑，重複內容={} bytes\n",
                group.len,
                group.paths.len(),
                group.duplicate_bytes
            )
            .as_str(),
        );
        for path in &group.paths {
            text.push_str(format!("  {path:?}\n").as_str());
        }
    }
    for (label, issues) in [
        ("收集問題", report.scan_issues.as_slice()),
        ("內容問題", report.file_issues.as_slice()),
    ] {
        for issue in issues {
            let path = match &issue.path_debug {
                Some(path) => path.as_str(),
                None => "未知位置",
            };
            text.push_str(
                format!(
                    "{label}：{path}；動作={:?}；原因={:?}\n",
                    issue.action, issue.reason
                )
                .as_str(),
            );
        }
    }
    text
}
// --8<-- [end:text]

// --8<-- [start:json]
pub fn render_json(report: &Report) -> Result<String, serde_json::Error> {
    let mut text = serde_json::to_string_pretty(report)?;
    text.push('\n');
    Ok(text)
}
// --8<-- [end:json]

// A3 驗收：將整個 mod tests 加到 src/report.rs 末尾。
#[cfg(test)]
mod tests {
    use crate::collection::ScanCause;
    use crate::collection::ScanIssue;
    use crate::content::ContentReport;
    use crate::content::DuplicateGroup;
    use crate::files::FileCause;
    use crate::files::FileIssue;
    use crate::metrics::MetricError;
    use crate::report::Diagnostic;
    use crate::report::Report;
    use crate::report::ReportError;
    use crate::report::build_report;
    use crate::report::order_issues;
    use crate::report::render_json;
    use crate::report::render_text;
    use std::io;
    use std::path::Path;

    // --8<-- [start:fixture]
    fn group(len: u64, names: &[&str]) -> DuplicateGroup {
        let mut paths = Vec::new();
        for name in names {
            paths.push(Path::new(*name).to_path_buf());
        }
        DuplicateGroup { len, paths }
    }

    fn content(groups: Vec<DuplicateGroup>) -> ContentReport {
        ContentReport {
            groups,
            scan_issues: Vec::new(),
            file_issues: Vec::new(),
        }
    }
    // --8<-- [end:fixture]

    // --8<-- [start:metric-test]
    #[test]
    fn known_groups_preserve_zero_bytes_and_calculate_ten() {
        let input = content(vec![
            group(0, &["empty-b", "empty-a"][..]),
            group(5, &["c", "b", "a"][..]),
        ]);
        let report = build_report(input).expect("固定小數字與合法路徑");
        assert!(report.complete);
        assert_eq!(report.duplicate_bytes, 10);
        assert_eq!(report.groups.len(), 2);
        assert_eq!(
            report.groups.as_slice()[0].paths,
            vec![String::from("a"), String::from("b"), String::from("c")]
        );
        assert_eq!(report.groups.as_slice()[0].duplicate_bytes, 10);
        assert_eq!(report.groups.as_slice()[1].duplicate_bytes, 0);
    }
    // --8<-- [end:metric-test]

    #[test]
    fn empty_success_and_singleton_are_not_duplicate_groups() {
        let empty = build_report(content(Vec::new())).expect("空結果可以完成");
        assert!(empty.complete);
        assert_eq!(empty.duplicate_bytes, 0);
        assert_eq!(empty.groups.len(), 0);
        let singleton =
            build_report(content(vec![group(9, &["single"][..])])).expect("單一路徑不產生重複內容");
        assert_eq!(singleton.groups.len(), 0);
        assert_eq!(singleton.duplicate_bytes, 0);
    }

    #[test]
    fn groups_are_ordered_by_native_paths_before_string_conversion() {
        let first = build_report(content(vec![
            group(2, &["z", "a.txt"][..]),
            group(3, &["a/b", "a/c"][..]),
        ]))
        .expect("固定合法報告");
        let second = build_report(content(vec![
            group(3, &["a/c", "a/b"][..]),
            group(2, &["a.txt", "z"][..]),
        ]))
        .expect("改輸入順序不改資料");
        assert_eq!(first, second);
        assert_eq!(
            first.groups.as_slice()[0].paths.as_slice()[0].as_str(),
            "a/b"
        );
        assert_eq!(
            first.groups.as_slice()[1].paths.as_slice()[0].as_str(),
            "a.txt"
        );
        assert_eq!(render_text(&first), render_text(&second));
        assert_eq!(
            render_json(&first).expect("模型可編碼"),
            render_json(&second).expect("同一模型可編碼")
        );
    }

    fn diagnostic(action: &str, reason: &str, debug: Option<&str>) -> Diagnostic {
        Diagnostic {
            path_debug: debug.map(String::from),
            action: String::from(action),
            reason: String::from(reason),
        }
    }

    // --8<-- [start:order-test]
    #[test]
    fn diagnostics_use_native_key_and_all_display_ties() {
        let make_rows = || {
            vec![
                (
                    Some(Path::new("a.txt").to_path_buf()),
                    diagnostic("A", "A", Some("file")),
                ),
                (
                    Some(Path::new("a/b").to_path_buf()),
                    diagnostic("B", "A", Some("child")),
                ),
                (
                    Some(Path::new("a/b").to_path_buf()),
                    diagnostic("A", "B", Some("second")),
                ),
                (
                    Some(Path::new("a/b").to_path_buf()),
                    diagnostic("A", "A", Some("second")),
                ),
                (
                    Some(Path::new("a/b").to_path_buf()),
                    diagnostic("A", "A", Some("first")),
                ),
                (None, diagnostic("Z", "Z", None)),
            ]
        };
        let first = order_issues(make_rows());
        let mut original = make_rows();
        let mut reversed = Vec::new();
        while !original.as_slice().is_empty() {
            match original.pop() {
                Some(row) => reversed.push(row),
                None => panic!("已確認來源非空"),
            }
        }
        let second = order_issues(reversed);
        assert_eq!(first, second);
        assert_eq!(first.len(), 6);
        assert_eq!(first.as_slice()[0].path_debug, None);
        assert_eq!(first.as_slice()[1].path_debug, Some(String::from("first")));
        assert_eq!(first.as_slice()[2].path_debug, Some(String::from("second")));
        assert_eq!(first.as_slice()[5].path_debug, Some(String::from("file")));
    }
    // --8<-- [end:order-test]

    #[test]
    fn both_issue_kinds_remain_visible_and_make_the_report_partial() {
        let mut input = content(vec![group(3, &["a", "b"][..])]);
        input.scan_issues.push(ScanIssue {
            path: None,
            action: "走訪目錄",
            cause: ScanCause::Io(io::Error::from(io::ErrorKind::PermissionDenied)),
        });
        input.file_issues.push(FileIssue {
            path: Path::new("unread").to_path_buf(),
            action: "內容比較讀取",
            cause: FileCause::Changed,
        });
        let report = build_report(input).expect("項目問題不阻止建報告");
        assert!(!report.complete);
        assert_eq!(report.groups.len(), 1);
        assert_eq!(report.scan_issues.len(), 1);
        assert_eq!(report.file_issues.len(), 1);
        assert_eq!(report.scan_issues.as_slice()[0].path_debug, None);
        let text = render_text(&report);
        assert!(text.contains("掃描完整=false"));
        assert!(text.contains("收集問題：未知位置"));
        assert!(text.contains("內容問題："));
        let json = render_json(&report).expect("這些欄位可編碼");
        let recovered = serde_json::from_str::<Report>(json.as_str()).expect("可讀回同一schema");
        assert_eq!(recovered, report);
    }

    #[test]
    fn text_escapes_control_characters_and_json_round_trip_keeps_data() {
        let mut name = String::from("兩行");
        name.push('"');
        name.push('\n');
        name.push_str("檔名");
        let native = Path::new(name.as_str()).to_path_buf();
        let input = content(vec![DuplicateGroup {
            len: 2,
            paths: vec![native, Path::new("plain").to_path_buf()],
        }]);
        let report = build_report(input).expect("原生位置可表示為Unicode");
        let text = render_text(&report);
        let lines = text.as_str().lines().fold(0, |count, _line| count + 1);
        assert_eq!(lines, 6);
        assert!(text.contains(format!("{name:?}").as_str()));
        let json = render_json(&report).expect("字元由格式實作編碼");
        let recovered = serde_json::from_str::<Report>(json.as_str()).expect("合法JSON");
        assert_eq!(recovered, report);
        let mut found = false;
        for path in &recovered.groups.as_slice()[0].paths {
            if path.as_str() == name.as_str() {
                found = true;
            }
        }
        assert!(found);
    }

    #[test]
    fn multiplication_and_total_overflow_are_fatal_report_errors() {
        for input in [
            content(vec![group(u64::MAX, &["a", "b", "c"][..])]),
            content(vec![
                group(u64::MAX, &["a", "b"][..]),
                group(1, &["c", "d"][..]),
            ]),
        ] {
            match build_report(input) {
                Err(ReportError::Metric(MetricError::Overflow)) => (),
                _ => panic!("溢位不能當成部分成功報告"),
            }
        }
    }

    #[cfg(unix)]
    #[test]
    fn non_unicode_item_is_diagnostic_and_keeps_native_identity_until_ordered() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;
        use std::path::PathBuf;

        let native = PathBuf::from(OsString::from_vec(vec![255]));
        let expected_debug = format!("{native:?}");
        let mut input = content(Vec::new());
        input.scan_issues.push(ScanIssue {
            path: Some(native),
            action: "準備報告路徑",
            cause: ScanCause::NonUnicode,
        });
        let report = build_report(input).expect("診斷不要求原生位置為Unicode");
        assert!(!report.complete);
        assert_eq!(
            report.scan_issues.as_slice()[0].path_debug,
            Some(expected_debug)
        );
        let json = render_json(&report).expect("debug文字是合法UTF8");
        let recovered = serde_json::from_str::<Report>(json.as_str()).expect("可讀回診斷");
        assert_eq!(recovered, report);
    }

    #[cfg(unix)]
    #[test]
    fn hard_link_paths_contribute_logical_bytes() {
        use crate::collection::scan;
        use crate::content::analyze;
        use std::fs;
        use tempfile::tempdir;

        let directory = tempdir().expect("可建立獨立fixture");
        let first = directory.path().join("a");
        let second = directory.path().join("b");
        fs::write(first.as_path(), "ABC").expect("可建立一般檔案");
        fs::hard_link(first.as_path(), second.as_path()).expect("同檔案系統可建hardlink");
        let report = build_report(analyze(scan(directory.path()).expect("有效根目錄")))
            .expect("兩個原生路徑可以建報告");
        assert_eq!(report.groups.len(), 1);
        assert_eq!(report.duplicate_bytes, 3);
    }
}
