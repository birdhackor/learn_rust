use crate::batch::Scheduler;
use crate::batch::StopEvent;
use crate::batch::finish_signal_controller;
use crate::batch::run_batch;
use crate::batch::signal_controller;
use crate::config::Settings;
use crate::error::RunError;
use crate::input::Input;
use crate::input::prepare_inputs;
use crate::report::Report;
use crate::report::build_report;
use crate::single::build_client;
use reqwest::Client;
use std::fs;
use std::io;
use std::io::Write;
use std::time::Duration;
use tokio::sync::mpsc;

pub fn write_output<W: Write>(text: &str, output: &mut W) -> io::Result<()> {
    output.write_all(text.as_bytes())?;
    output.flush()
}

// --8<-- [start:write]
/// 完成編碼、write_all與flush後，才交回資料狀態。
pub fn write_report<W: Write>(report: &Report, json: bool, output: &mut W) -> Result<u8, RunError> {
    let _json = json;
    let _output = output; // 尚未真正編碼、寫出或flush。
    Ok(report.status())
}
// --8<-- [end:write]

fn validate_limits(jobs: usize, timeout_secs: u64) -> Result<(), RunError> {
    if jobs == 0 {
        return Err(RunError::ZeroJobs);
    }
    if timeout_secs == 0 {
        return Err(RunError::ZeroTimeout);
    }
    Ok(())
}

fn validate(settings: &Settings) -> Result<(), RunError> {
    validate_limits(settings.jobs, settings.timeout_secs)?;
    Ok(())
}

fn load_inputs(settings: &Settings) -> Result<Vec<Input>, RunError> {
    validate(settings)?;
    let text = match fs::read_to_string(settings.filename.as_path()) {
        Ok(text) => text,
        Err(error) => return Err(RunError::Read(error)),
    };
    Ok(prepare_inputs(text.as_str())?)
}

/// 借用共用client，取得原文字的owned報告。以stop送Requested並await收尾；不要丟棄整份future替代清理。
pub async fn check_text(
    client: &Client,
    text: &str,
    jobs: usize,
    timeout_secs: u64,
    stop: mpsc::Receiver<StopEvent>,
) -> Result<Report, RunError> {
    validate_limits(jobs, timeout_secs)?;
    let inputs = prepare_inputs(text)?;
    let batch = run_batch(
        client,
        inputs,
        jobs,
        Duration::from_secs(timeout_secs),
        stop,
    )
    .await?;
    build_report(batch)
}

// --8<-- [start:run]
/// 讀取UTF-8清單、完成所有client工作，並結束listener；報告輸出是另一個步驟。
pub async fn run(settings: &Settings) -> Result<Report, RunError> {
    let inputs = load_inputs(settings)?;
    let client = build_client(false)?;
    let mut scheduler = Scheduler::new(
        &client,
        inputs,
        settings.jobs,
        Duration::from_secs(settings.timeout_secs),
    )?;
    let (stop_tx, mut stop_rx) = mpsc::channel::<StopEvent>(1);
    let (entered_tx, mut entered_rx) = mpsc::channel::<()>(1);
    let controller = tokio::spawn(signal_controller(stop_tx, entered_tx));
    // None表示事件已提前發生或來源已失去；真正理由仍從停止channel取得。
    let _entered = entered_rx.recv().await;
    let reason = scheduler.run(&mut stop_rx).await;
    let finished = finish_signal_controller(controller).await;
    let batch = scheduler.into_batch(reason);
    match finished {
        Ok(()) => build_report(batch),
        Err(message) => Err(RunError::Controller(message)),
    }
}
// --8<-- [end:run]
