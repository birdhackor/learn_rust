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
use crate::report::render;
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
    let text = render(report, json)?;
    write_output(text.as_str(), output)?;
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

#[cfg(test)]
mod tests {
    use crate::app::load_inputs;
    use crate::app::write_report;
    use crate::config::Settings;
    use crate::error::RunError;
    use crate::report::Report;
    use crate::report::Row;
    use std::fs;
    use std::io;
    use std::io::Write;
    use tempfile::tempdir;

    fn report() -> Report {
        Report {
            cancelled: true,
            items: vec![Row {
                input_index: 4,
                original: "/notes".to_owned(),
                kind: "unstarted".to_owned(),
                passed: false,
                status: None,
                final_url: None,
                elapsed_ms: None,
                detail: None,
            }],
        }
    }

    // --8<-- [start:writer]
    struct ControlledWriter {
        bytes: Vec<u8>,
        refuse_write: bool,
        refuse_flush: bool,
        flushed: bool,
    }
    impl Write for ControlledWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            if self.refuse_write {
                return Err(io::Error::from(io::ErrorKind::PermissionDenied));
            }
            for byte in buffer {
                self.bytes.push(*byte);
            }
            Ok(buffer.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            self.flushed = true;
            if self.refuse_flush {
                Err(io::Error::from(io::ErrorKind::PermissionDenied))
            } else {
                Ok(())
            }
        }
    }
    // --8<-- [end:writer]

    #[test]
    fn invalid_configuration_precedes_file_read_and_empty_is_fatal() {
        let directory = tempdir().expect("自有fixture");
        let missing = directory.path().join("未建立.txt");
        let mut settings = Settings {
            filename: missing,
            jobs: 0,
            timeout_secs: 10,
            json: false,
        };
        let zero_jobs = load_inputs(&settings);
        settings.jobs = 2;
        settings.timeout_secs = 0;
        let zero_timeout = load_inputs(&settings);
        settings.timeout_secs = 10;
        let read_failed = load_inputs(&settings);
        let empty = directory.path().join("空清單.txt");
        fs::write(empty.as_path(), "\n  \n").expect("寫自有材料");
        settings.filename = empty;
        let empty_failed = load_inputs(&settings);
        directory.close().expect("先清理fixture");
        match zero_jobs {
            Err(RunError::ZeroJobs) => (),
            other => panic!("先拒絕零jobs：{other:?}"),
        }
        match zero_timeout {
            Err(RunError::ZeroTimeout) => (),
            other => panic!("先拒絕零期限：{other:?}"),
        }
        match read_failed {
            Err(RunError::Read(error)) => assert_eq!(error.kind(), io::ErrorKind::NotFound),
            other => panic!("缺檔是read：{other:?}"),
        }
        match empty_failed {
            Err(RunError::Empty) => (),
            other => panic!("空清單是fatal：{other:?}"),
        }
    }

    #[test]
    fn write_failure_overrides_cancellation_data_status() {
        let mut writer = ControlledWriter {
            bytes: Vec::new(),
            refuse_write: true,
            refuse_flush: false,
            flushed: false,
        };
        match write_report(&report(), true, &mut writer) {
            Err(RunError::Output(error)) => {
                assert_eq!(error.kind(), io::ErrorKind::PermissionDenied)
            }
            other => panic!("寫入失敗必須fatal，不是130：{other:?}"),
        }
        assert_eq!(writer.bytes.len(), 0);
        assert!(!writer.flushed);
    }

    // --8<-- [start:flush-test]
    #[test]
    fn flush_failure_overrides_cancellation_after_bytes_were_accepted() {
        let mut writer = ControlledWriter {
            bytes: Vec::new(),
            refuse_write: false,
            refuse_flush: true,
            flushed: false,
        };
        match write_report(&report(), true, &mut writer) {
            Err(RunError::Output(error)) => {
                assert_eq!(error.kind(), io::ErrorKind::PermissionDenied)
            }
            other => panic!("已收bytes也不能忽略flush失敗：{other:?}"),
        }
        assert!(!writer.bytes.is_empty());
        assert!(writer.flushed);
    }
    // --8<-- [end:flush-test]

    #[test]
    fn successful_report_is_flushed_and_only_then_returns_130() {
        let expected = report();
        let mut writer = ControlledWriter {
            bytes: Vec::new(),
            refuse_write: false,
            refuse_flush: false,
            flushed: false,
        };
        assert_eq!(
            write_report(&expected, true, &mut writer).expect("寫出與flush"),
            130
        );
        assert!(writer.flushed);
        let text = std::str::from_utf8(writer.bytes.as_slice()).expect("報告UTF-8");
        let recovered = serde_json::from_str::<Report>(text).expect("整份stdout形狀");
        assert_eq!(recovered, expected);
    }
}
