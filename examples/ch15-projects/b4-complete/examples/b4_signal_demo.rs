use ch11_local_service::Fixture;
use std::io;
use std::io::Write;
use std::process::ExitCode;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use url_b4::Batch;
use url_b4::EndReason;
use url_b4::Scheduler;
use url_b4::StopEvent;
use url_b4::finish_signal_controller;
use url_b4::input::prepare_inputs;
use url_b4::signal_controller;
use url_b4::single::build_client;
use url_b4::single::default_limit;

async fn ready(fixture: &mut Fixture) -> Result<(), String> {
    let mut seen = Vec::new();
    for _ in 0..2 {
        match timeout(Duration::from_secs(5), fixture.next_started()).await {
            Ok(Some(index)) => seen.push(index),
            other => return Err(format!("未確認兩份held已進入：{other:?}")),
        }
    }
    let seen = seen.as_slice();
    if !((seen[0] == 0 && seen[1] == 1) || (seen[0] == 1 && seen[1] == 0)) {
        return Err("不是預期的held0／1".to_owned());
    }
    let mut output = io::stdout();
    match writeln!(
        output,
        "READY：listener已進入，兩份held已開始；現在可按Ctrl+C"
    ) {
        Ok(()) => (),
        Err(error) => return Err(format!("ready輸出失敗：{error}")),
    }
    match output.flush() {
        Ok(()) => Ok(()),
        Err(error) => Err(format!("ready flush失敗：{error}")),
    }
}

async fn close(
    scheduler: &mut Scheduler,
    fixture: Fixture,
    controller: JoinHandle<bool>,
) -> Result<usize, String> {
    scheduler.stop_and_finish().await;
    let signal_finished = finish_signal_controller(controller).await;
    let server_finished = fixture.shutdown().await;
    signal_finished?;
    server_finished
}

async fn run_demo() -> Result<(Batch, usize), String> {
    let client = match build_client(true) {
        Ok(client) => client,
        Err(error) => return Err(format!("client建立失敗：{error}")),
    };
    let mut fixture = Fixture::start(2).await?;
    let first = fixture.url("/held/0");
    let second = fixture.url("/held/1");
    let text = format!("\n{first}\n \n{second}\n{first}\n/notes\n");
    let inputs = match prepare_inputs(text.as_str()) {
        Ok(inputs) => inputs,
        Err(error) => {
            let stopped = fixture.shutdown().await;
            return Err(format!("材料讀取失敗：{error:?}；server={stopped:?}"));
        }
    };
    let mut scheduler = match Scheduler::new(&client, inputs, 2, default_limit()) {
        Ok(scheduler) => scheduler,
        Err(error) => {
            let stopped = fixture.shutdown().await;
            return Err(format!("設定失敗：{error:?}；server={stopped:?}"));
        }
    };
    let (stop_tx, mut stop_rx) = mpsc::channel::<StopEvent>(1);
    let (entered_tx, mut entered_rx) = mpsc::channel::<()>(1);
    let controller = tokio::spawn(signal_controller(stop_tx, entered_tx));
    match timeout(Duration::from_secs(5), entered_rx.recv()).await {
        Ok(Some(())) => (),
        // 監聽已失敗或提前收到事件，run會從同一停止channel取得真正理由。
        Ok(None) => (),
        Err(error) => {
            let stopped = close(&mut scheduler, fixture, controller).await;
            return Err(format!("listener觀察到期：{error}；收尾={stopped:?}"));
        }
    }
    let mut observed = None;
    let waited = timeout(Duration::from_secs(15), async {
        tokio::select! {
            _ = async {
                observed = Some(ready(&mut fixture).await);
                std::future::pending::<()>().await;
            } => panic!("ready支路的pending不會完成"),
            reason = scheduler.run(&mut stop_rx) => reason,
        }
    })
    .await;
    let stopped = close(&mut scheduler, fixture, controller).await;
    let hits = stopped?;
    let reason = match waited {
        Ok(reason) => reason,
        Err(error) => return Err(format!("demo觀察到期：{error}")),
    };
    if let Some(Err(message)) = observed {
        return Err(message);
    }
    match &reason {
        EndReason::SignalFailed(message) => return Err(format!("訊號監聽失敗：{message}")),
        EndReason::StopSourceClosed => return Err("訊號來源已關閉".to_owned()),
        _ => (),
    }
    Ok((scheduler.into_batch(reason), hits))
}

fn print_batch(batch: &Batch, hits: usize) -> io::Result<()> {
    let mut output = io::stdout();
    writeln!(output, "結束理由：{:?}", batch.reason)?;
    for entry in batch.entries.as_slice() {
        writeln!(
            output,
            "index={} original={:?} completion={:?}",
            entry.input_index,
            entry.original(),
            entry.completion
        )?;
    }
    writeln!(output, "所有client／listener／server已收尾；工作GET={hits}")?;
    output.flush()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let (batch, hits) = match run_demo().await {
        Ok(result) => result,
        Err(message) => {
            eprintln!("demo失敗：{message}");
            return ExitCode::from(2);
        }
    };
    if let Err(error) = print_batch(&batch, hits) {
        eprintln!("報告輸出失敗：{error}");
        return ExitCode::from(2);
    }
    match batch.reason {
        EndReason::Requested => ExitCode::from(130),
        // 本demo正常做完一定包含/notes項目失敗，不代表通過全部網址。
        EndReason::Finished => ExitCode::from(1),
        _ => ExitCode::from(2),
    }
}
