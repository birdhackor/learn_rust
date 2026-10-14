use ch11_http_single::config::Settings;
use ch11_http_single::run;
use clap::Parser;
use std::env;
use std::io;
use std::io::Write;
use std::process::ExitCode;
use std::string::ToString;

fn report_error(message: &str, status: u8) -> ExitCode {
    let mut output = io::stderr();
    match writeln!(output, "{message}") {
        Ok(()) => match output.flush() {
            Ok(()) => ExitCode::from(status),
            Err(_) => ExitCode::FAILURE,
        },
        Err(_) => ExitCode::FAILURE,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
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
    match run(&settings, &mut output).await {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(error) => {
            let message = error.to_string();
            report_error(message.as_str(), error.status())
        }
    }
}
