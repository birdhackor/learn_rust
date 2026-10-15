use clap::Parser;
use duplicate_a4::app::Settings;
use duplicate_a4::app::run;
use duplicate_a4::app::write_output;
use std::env;
use std::io;
use std::process::ExitCode;

// --8<-- [start:main]
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
        Err(error) => fatal(format!("無法完成 root={:?}：{error}", settings.root).as_str()),
    }
}
// --8<-- [end:main]
