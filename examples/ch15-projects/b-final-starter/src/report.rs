use crate::batch::Batch;
use crate::batch::EndReason;
use crate::error::RunError;
use serde::Deserialize;
use serde::Serialize;

// --8<-- [start:model]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Row {
    pub input_index: usize,
    pub original: String,
    pub kind: String,
    pub passed: bool,
    pub status: Option<u16>,
    pub final_url: Option<String>,
    pub elapsed_ms: Option<u128>,
    pub detail: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub cancelled: bool,
    pub items: Vec<Row>,
}
// --8<-- [end:model]

// --8<-- [start:map]
pub fn build_report(batch: Batch) -> Result<Report, RunError> {
    let cancelled = match batch.reason {
        EndReason::Finished => false,
        EndReason::Requested => true,
        EndReason::SignalFailed(message) => return Err(RunError::Signal(message)),
        EndReason::StopSourceClosed => return Err(RunError::StopSourceClosed),
    };
    let _entries = batch.entries;
    let items = Vec::new(); // 尚未把原Batch逐筆交成Row。
    Ok(Report { cancelled, items })
}
// --8<-- [end:map]

// --8<-- [start:status]
impl Report {
    /// 只計算資料狀態；caller仍須成功寫出及flush，才交回這個狀態。
    pub fn status(&self) -> u8 {
        0
    }
}
// --8<-- [end:status]

// --8<-- [start:render]
pub fn render(report: &Report, json: bool) -> Result<String, serde_json::Error> {
    if json {
        let mut text = serde_json::to_string_pretty(report)?;
        text.push('\n');
        return Ok(text);
    }
    let mut text = format!(
        "cancelled={} items={}\n",
        report.cancelled,
        report.items.len()
    );
    for row in report.items.as_slice() {
        let original = serde_json::to_string(row.original.as_str())?;
        let status = serde_json::to_string(&row.status)?;
        let final_url = serde_json::to_string(&row.final_url)?;
        let elapsed = serde_json::to_string(&row.elapsed_ms)?;
        let detail = serde_json::to_string(&row.detail)?;
        let line = format!(
            "index={} kind={} passed={} original={} status={} final_url={} elapsed_ms={} detail={}\n",
            row.input_index,
            row.kind.as_str(),
            row.passed,
            original.as_str(),
            status.as_str(),
            final_url.as_str(),
            elapsed.as_str(),
            detail.as_str()
        );
        text.push_str(line.as_str());
    }
    Ok(text)
}
// --8<-- [end:render]
