use ch07_clap::Settings;
use clap::Parser;
use std::env;
use std::io;
use std::io::Write;
use std::process::ExitCode;

fn show_settings(settings: &Settings) -> io::Result<()> {
    let mut output = io::stdout();
    writeln!(output, "query={}", settings.query.as_str())?;
    writeln!(output, "filename={}", settings.filename.as_path().display())?;
    writeln!(output, "ignore_case={}", settings.ignore_case)?;
    Ok(())
}

fn main() -> ExitCode {
    match Settings::try_parse_from(env::args_os()) {
        Ok(settings) => match show_settings(&settings) {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
        Err(error) => {
            let code = error.exit_code();
            match error.print() {
                Ok(()) => {
                    if code == 0 {
                        ExitCode::SUCCESS
                    } else {
                        let usage_status: u8 = 2;
                        ExitCode::from(usage_status)
                    }
                }
                Err(_) => ExitCode::FAILURE,
            }
        }
    }
}
