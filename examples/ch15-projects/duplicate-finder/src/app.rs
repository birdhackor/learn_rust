use crate::collection::RootError;
use crate::report::Report;
use crate::report::ReportError;
use clap::Parser;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "duplicate-finder",
    version = "0.1.0",
    about = "尋找一個目錄內內容相同的檔案，只產生報告"
)]
pub struct Settings {
    #[arg(help = "要掃描的一個根目錄")]
    pub root: PathBuf,
    #[arg(long, help = "以 JSON 輸出同一份報告")]
    pub json: bool,
}

// --8<-- [start:error-boundary]
#[derive(Debug)]
#[non_exhaustive]
pub enum RunError {
    Root(RootError),
    Report(ReportError),
    Json(serde_json::Error),
    Output(io::Error),
}

impl fmt::Display for RunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Root(RootError::Io(_)) => write!(formatter, "無法讀取根目錄資料"),
            Self::Root(RootError::SymbolicLink) => write!(formatter, "根目錄不能是 symbolic link"),
            Self::Root(RootError::NotDirectory) => write!(formatter, "根位置不是目錄"),
            Self::Report(ReportError::Metric(crate::metrics::MetricError::Count(_))) => {
                write!(formatter, "建立報告時無法將路徑數表示為 u64")
            }
            Self::Report(error) => write!(formatter, "無法建立正確報告：{error}"),
            Self::Json(_) => write!(formatter, "無法編碼 JSON"),
            Self::Output(_) => write!(formatter, "無法完成輸出"),
        }
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Root(RootError::Io(error)) => Some(error),
            Self::Report(ReportError::Metric(crate::metrics::MetricError::Count(error))) => {
                Some(error)
            }
            Self::Json(error) => Some(error),
            Self::Output(error) => Some(error),
            _ => None,
        }
    }
}

// --8<-- [end:error-boundary]

impl From<RootError> for RunError {
    fn from(error: RootError) -> Self {
        Self::Root(error)
    }
}

impl From<ReportError> for RunError {
    fn from(error: ReportError) -> Self {
        Self::Report(error)
    }
}

impl From<serde_json::Error> for RunError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<io::Error> for RunError {
    fn from(error: io::Error) -> Self {
        Self::Output(error)
    }
}

pub fn write_output<W: Write>(text: &str, output: &mut W) -> io::Result<()> {
    output.write_all(text.as_bytes())?;
    output.flush()
}

pub fn write_report<W: Write>(report: &Report, json: bool, output: &mut W) -> Result<u8, RunError> {
    let text = if json {
        crate::report::render_json(report)?
    } else {
        crate::report::render_text(report)
    };
    write_output(text.as_str(), output)?;
    if report.complete { Ok(0) } else { Ok(1) }
}

// --8<-- [start:scan-api]
/// 借一個原生根目錄取得 owned 報告；項目問題仍是 Ok 的部分結果。
///
/// # Examples
///
/// ```
/// use duplicate_finder::scan_report;
/// use tempfile::tempdir;
/// let directory = tempdir().expect("可以建立自有空目錄");
/// let report = scan_report(directory.path()).expect("根目錄有效");
/// assert!(report.complete);
/// assert_eq!(report.groups.len(), 0);
/// ```
pub fn scan_report(root: &Path) -> Result<Report, RunError> {
    let collected = crate::collection::scan(root)?;
    let report = crate::report::build_report(crate::content::analyze(collected))?;
    Ok(report)
}

// --8<-- [end:scan-api]

/// 成功完成 write_all 與 flush 後，才回掃描完整性的 0／1。
pub fn run<W: Write>(settings: &Settings, output: &mut W) -> Result<u8, RunError> {
    let report = scan_report(settings.root.as_path())?;
    write_report(&report, settings.json, output)
}
