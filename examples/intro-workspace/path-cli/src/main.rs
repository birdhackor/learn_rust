// --8<-- [start:argument-error]
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ArgumentError {
    MissingFilename,
    TooManyArguments,
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFilename => write!(f, "需要一個檔名"),
            Self::TooManyArguments => write!(f, "只接受一個檔名"),
        }
    }
}
// --8<-- [end:argument-error]

// --8<-- [start:argument-parser]
use std::ffi::OsString;
use std::path::PathBuf;

pub fn parse_filename<I: IntoIterator<Item = OsString>>(
    input: I,
) -> Result<PathBuf, ArgumentError> {
    let mut arguments = input.into_iter();
    let _program_name = arguments.next();
    let filename = match arguments.next() {
        Some(filename) => filename,
        None => return Err(ArgumentError::MissingFilename),
    };
    match arguments.next() {
        Some(_) => Err(ArgumentError::TooManyArguments),
        None => Ok(PathBuf::from(filename)),
    }
}
// --8<-- [end:argument-parser]

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

// --8<-- [start:argument-tests]
#[cfg(test)]
mod tests {
    use crate::ArgumentError;
    use crate::parse_filename;
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn exact_one_filename_keeps_native_path_content() {
        let arguments = [OsString::from("show-note"), OsString::from("中文筆記.txt")];
        let filename = parse_filename(arguments).expect("固定參數包含一個檔名");
        assert_eq!(filename, PathBuf::from("中文筆記.txt"));
    }

    #[test]
    fn missing_and_extra_filenames_keep_distinct_errors() {
        assert_eq!(
            parse_filename([OsString::from("show-note")]),
            Err(ArgumentError::MissingFilename)
        );
        assert_eq!(
            parse_filename([
                OsString::from("show-note"),
                OsString::from("first.txt"),
                OsString::from("second.txt"),
            ]),
            Err(ArgumentError::TooManyArguments)
        );
    }
}
// --8<-- [end:argument-tests]
