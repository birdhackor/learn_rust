use crate::collection::ScanIssue;
use crate::content::ContentReport;
use crate::files::FileIssue;
use crate::metrics::MetricError;
use serde::Deserialize;
use serde::Serialize;
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
pub fn build_report(_content: ContentReport) -> Result<Report, ReportError> {
    // TODO：原生排序、checked 指標、Unicode 群組與兩種診斷、完整性。
    // 固定 Overflow 只標示尚未實作，並沒有真的做計算。
    Err(ReportError::Metric(MetricError::Overflow))
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
pub fn order_issues(_rows: Vec<IssueRow>) -> Vec<Diagnostic> {
    // TODO：按原生位置，再 action／reason／debug 表示排完整順序。
    Vec::new()
}
// --8<-- [end:order]

// --8<-- [start:text]
pub fn render_text(_report: &Report) -> String {
    // TODO：同一份 Report 的完整性、指標、群組與兩種問題都要呈現。
    String::new()
}
// --8<-- [end:text]

// --8<-- [start:json]
pub fn render_json(report: &Report) -> Result<String, serde_json::Error> {
    let mut text = serde_json::to_string_pretty(report)?;
    text.push('\n');
    Ok(text)
}
// --8<-- [end:json]
