use crate::collection::RootError;
use crate::report::Report;
use crate::report::ReportError;
use clap::Parser;
use std::fmt;
use std::io;
use std::io::Write;
use std::path::PathBuf;

// --8<-- [start:settings]
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
// --8<-- [end:settings]

// --8<-- [start:errors]
#[derive(Debug)]
pub enum RunError {
    Root(RootError),
    Report(ReportError),
    Json(serde_json::Error),
    Output(io::Error),
}

impl fmt::Display for RunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Root(RootError::Io(error)) => write!(formatter, "無法讀取根目錄：{error}"),
            Self::Root(RootError::SymbolicLink) => write!(formatter, "根目錄不能是 symbolic link"),
            Self::Root(RootError::NotDirectory) => write!(formatter, "根位置不是目錄"),
            Self::Report(error) => write!(formatter, "無法建立正確報告：{error}"),
            Self::Json(error) => write!(formatter, "無法編碼 JSON：{error}"),
            Self::Output(error) => write!(formatter, "無法完成輸出：{error}"),
        }
    }
}

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
// --8<-- [end:errors]

// --8<-- [start:output]
pub fn write_output<W: Write>(text: &str, output: &mut W) -> io::Result<()> {
    output.write_all(text.as_bytes())?;
    output.flush()
}
// --8<-- [end:output]

// --8<-- [start:write-report]
pub fn write_report<W: Write>(
    _report: &Report,
    _json: bool,
    _output: &mut W,
) -> Result<u8, RunError> {
    // TODO：格式、真正輸出完成、完整性與 0／1。
    // 固定 0 只標示待做，沒有成功輸出的意義。
    Ok(0)
}
// --8<-- [end:write-report]

// --8<-- [start:run]
pub fn run<W: Write>(_settings: &Settings, _output: &mut W) -> Result<u8, RunError> {
    // TODO：接上 A1→A2→A3，沿型別交回致命錯誤。
    Err(RunError::Report(ReportError::Metric(
        crate::metrics::MetricError::Overflow,
    )))
}
// --8<-- [end:run]
