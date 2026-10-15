use clap::Parser;
use duplicate_finder::Settings;
use duplicate_finder::run;
use duplicate_finder::write_output;
use std::env;
use std::error::Error;
use std::io;
use std::process::ExitCode;

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

fn main() -> ExitCode {
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
    match run(&settings, &mut output) {
        Ok(status) => ExitCode::from(status),
        Err(error) => fatal(
            format!(
                "無法完成 root={:?}：{}",
                settings.root,
                explain(&error).as_str()
            )
            .as_str(),
        ),
    }
}
