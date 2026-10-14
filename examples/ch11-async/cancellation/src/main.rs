use ch11_cancellation::finish_signal_controller;
use ch11_cancellation::models::Batch;
use ch11_cancellation::models::EndReason;
use ch11_cancellation::models::StopEvent;
use ch11_cancellation::prepare_jobs;
use ch11_cancellation::run_batch;
use ch11_cancellation::signal_controller;
use ch11_cancellation::stop_demo;
use std::env;
use std::io;
use std::io::Write;
use std::process::ExitCode;
use tokio::sync::mpsc;

// --8<-- [start:mode]
fn signal_mode() -> Result<bool, String> {
    let mut arguments = env::args();
    let _program = arguments.next();
    match (arguments.next(), arguments.next()) {
        (None, None) => Ok(false),
        (Some(value), None) => {
            if value.as_str() == "--signal" {
                Ok(true)
            } else {
                Err("用法：ch11-cancellation [--signal]".to_owned())
            }
        }
        _ => Err("用法：ch11-cancellation [--signal]".to_owned()),
    }
}
// --8<-- [end:mode]

// --8<-- [start:real-signal]
async fn real_signal() -> Result<Batch, String> {
    let specs = vec![(80, false), (120, false), (200, false), (30, false)];
    let (jobs, releases) = prepare_jobs(specs.as_slice());
    let (started_tx, mut started_rx) = mpsc::channel::<usize>(2);
    let (stop_tx, stop_rx) = mpsc::channel::<StopEvent>(1);
    let (entered_tx, mut entered_rx) = mpsc::channel::<()>(1);
    let controller = tokio::spawn(signal_controller(stop_tx, entered_tx));
    let _entered = entered_rx.recv().await;
    let observer = tokio::spawn(async move {
        let first = started_rx.recv().await;
        let second = started_rx.recv().await;
        if let (Some(first), Some(second)) = (first, second) {
            assert!((first == 0 && second == 1) || (first == 1 && second == 0));
            println!("READY：Ctrl+C 監聽已進入；input 0、1 已開始；上限 2");
            {
                let mut output = io::stdout();
                output.flush().expect("ready 訊息能送到 stdout");
            }
        }
        while let Some(_input) = started_rx.recv().await {}
    });
    let batch = run_batch(jobs, 2, started_tx, stop_rx).await;
    let signal_finished = finish_signal_controller(controller).await;
    let observer_finished = observer.await;
    for release in releases.as_slice() {
        match release.send(()).await {
            Err(_) => (),
            Ok(()) => return Err("批次結束後仍留有放行收端".to_owned()),
        }
    }
    signal_finished?;
    match observer_finished {
        Ok(()) => (),
        Err(error) => return Err(format!("啟動觀察 task 失敗：{error}")),
    }
    match batch {
        Ok(batch) => Ok(batch),
        Err(error) => Err(format!("排程設定錯誤：{error:?}")),
    }
}
// --8<-- [end:real-signal]

// --8<-- [start:main]
#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let signal = match signal_mode() {
        Ok(signal) => signal,
        Err(message) => {
            eprintln!("{}", message.as_str());
            return ExitCode::from(2);
        }
    };
    let batch = if signal {
        match real_signal().await {
            Ok(batch) => batch,
            Err(message) => {
                eprintln!("{}", message.as_str());
                return ExitCode::FAILURE;
            }
        }
    } else {
        stop_demo().await
    };
    println!("結束原因：{:?}", batch.reason);
    println!(
        "共提交 {}；追蹤峰值 {}",
        batch.submitted, batch.peak_tracked
    );
    for report in batch.reports.as_slice() {
        println!("input {}：{:?}", report.input, report.outcome);
    }
    println!("已回收全部提交工作，並清理未提交輸入");
    match batch.reason {
        EndReason::Finished => ExitCode::SUCCESS,
        EndReason::Requested => ExitCode::from(130),
        EndReason::SignalFailed(_) | EndReason::StopSourceClosed => ExitCode::FAILURE,
    }
}
// --8<-- [end:main]
