// --8<-- [start:streams]
use std::io;
use std::io::Write;
use std::process::ExitCode;

fn report_error(message: &str, status: ExitCode) -> ExitCode {
    let mut output = io::stderr();
    match writeln!(output, "{message}") {
        Ok(()) => status,
        Err(_) => ExitCode::FAILURE,
    }
}
// --8<-- [end:streams]

// --8<-- [start:cli-main]
use ch07_path_cli::parse_filename;
use std::env;
use std::fs;
use std::string::ToString;

fn main() -> ExitCode {
    let filename = match parse_filename(env::args_os()) {
        Ok(filename) => filename,
        Err(error) => {
            let message = error.to_string();
            let usage_status: u8 = 2;
            return report_error(message.as_str(), ExitCode::from(usage_status));
        }
    };
    let text = match fs::read_to_string(filename.as_path()) {
        Ok(text) => text,
        Err(error) => {
            let message = format!("無法讀取 {}：{error}", filename.as_path().display());
            return report_error(message.as_str(), ExitCode::FAILURE);
        }
    };
    let mut output = io::stdout();
    let view = text.as_str();
    match output.write_all(view.as_bytes()) {
        Ok(()) => (),
        Err(error) => {
            let message = format!("無法寫出筆記：{error}");
            return report_error(message.as_str(), ExitCode::FAILURE);
        }
    }
    match output.flush() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let message = format!("無法完成筆記輸出：{error}");
            report_error(message.as_str(), ExitCode::FAILURE)
        }
    }
}
// --8<-- [end:cli-main]
