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
