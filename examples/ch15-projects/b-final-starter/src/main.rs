use clap::Parser;
use std::env;
use std::error::Error;
use std::io;
use std::process::ExitCode;
use url_checker_starter::Settings;
use url_checker_starter::run;
use url_checker_starter::write_output;

// --8<-- [start:explain]
fn explain(error: &dyn Error) -> String {
    let mut text = format!("{error}");
    let mut source = error.source();
    loop {
        match source {
            Some(cause) => {
                text.push_str(format!("\n原因：{cause}").as_str());
                source = cause.source();
            }
            None => return text,
        }
    }
}

// --8<-- [end:explain]

fn fatal(message: &str) -> ExitCode {
    let mut output = io::stderr();
    let text = format!("{message}\n");
    // 診斷自己也可能失敗；已是 fatal，仍回 2，不讓輸出失敗引發 panic。
    let _ = write_output(text.as_str(), &mut output);
    let status: u8 = 2;
    ExitCode::from(status)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let settings = match Settings::try_parse_from(env::args_os()) {
        Ok(settings) => settings,
        Err(error) => {
            let text = format!("{error}");
            let status: u8 = if error.exit_code() == 0 { 0 } else { 2 };
            let result = if error.use_stderr() {
                let mut output = io::stderr();
                write_output(text.as_str(), &mut output)
            } else {
                let mut output = io::stdout();
                write_output(text.as_str(), &mut output)
            };
            return match result {
                Ok(()) => ExitCode::from(status),
                Err(error) => fatal(format!("無法寫出參數訊息：{error}").as_str()),
            };
        }
    };
    let mut output = io::stdout();
    let report = match run(&settings).await {
        Ok(report) => report,
        Err(error) => return fatal(explain(&error).as_str()),
    };
    match url_checker_starter::write_report(&report, settings.json, &mut output) {
        Ok(status) => ExitCode::from(status),
        Err(error) => fatal(
            format!(
                "無法完成 filename={:?}：{}",
                settings.filename,
                explain(&error).as_str()
            )
            .as_str(),
        ),
    }
}
