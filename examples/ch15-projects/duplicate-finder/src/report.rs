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

pub fn render_json(report: &Report) -> Result<String, serde_json::Error> {
    let mut text = serde_json::to_string_pretty(report)?;
    text.push('\n');
    Ok(text)
}
