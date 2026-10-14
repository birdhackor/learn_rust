use ch07_search::config::Settings;
use ch07_search::error::AppError;
use ch07_search::run;
use clap::Parser;
use std::env;
use std::io;
use std::io::Write;
use std::process::ExitCode;
use std::string::ToString;

fn report_error(message: &str, status: u8) -> ExitCode {
    let mut output = io::stderr();
    match writeln!(output, "{message}") {
        Ok(()) => ExitCode::from(status),
        Err(_) => ExitCode::FAILURE,
    }
}

fn main() -> ExitCode {
    let settings = match Settings::try_parse_from(env::args_os()) {
        Ok(settings) => settings,
        Err(error) => {
            let code = error.exit_code();
            return match error.print() {
                Ok(()) => {
                    if code == 0 {
                        ExitCode::SUCCESS
                    } else {
                        let usage_status: u8 = 2;
                        ExitCode::from(usage_status)
                    }
                }
                Err(_) => ExitCode::FAILURE,
            };
        }
    };
    let mut output = io::stdout();
    match run(&settings, &mut output) {
        Ok(()) => match output.flush() {
            Ok(()) => ExitCode::SUCCESS,
            Err(cause) => {
                let error = AppError::Output(cause);
                let message = error.to_string();
                report_error(message.as_str(), error.status())
            }
        },
        Err(error) => {
            let message = error.to_string();
            report_error(message.as_str(), error.status())
        }
    }
}
