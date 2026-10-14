// --8<-- [start:settings]
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "query-settings",
    version = "0.1.0",
    about = "檢查文字搜尋工具的參數設定"
)]
pub struct Settings {
    #[arg(help = "要找的文字")]
    pub query: String,
    #[arg(help = "要讀取的文字檔")]
    pub filename: PathBuf,
    #[arg(long, help = "要求搜尋忽略大小寫")]
    pub ignore_case: bool,
}
// --8<-- [end:settings]

// --8<-- [start:settings-tests]
#[cfg(test)]
mod tests {
    use crate::Settings;
    use clap::Parser;
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn positionals_keep_text_and_native_filename_with_flag_off() {
        let settings = Settings::try_parse_from([
            OsString::from("query-settings"),
            OsString::from("Rust"),
            OsString::from("中文筆記.txt"),
        ])
        .expect("固定參數提供兩個必要位置參數");
        assert_eq!(settings.query.as_str(), "Rust");
        assert_eq!(settings.filename, PathBuf::from("中文筆記.txt"));
        assert!(!settings.ignore_case);
    }

    #[test]
    fn a_long_bool_flag_turns_the_setting_on() {
        let settings = Settings::try_parse_from([
            OsString::from("query-settings"),
            OsString::from("Rust"),
            OsString::from("中文筆記.txt"),
            OsString::from("--ignore-case"),
        ])
        .expect("固定參數包含已宣告的開關");
        assert_eq!(settings.query.as_str(), "Rust");
        assert_eq!(settings.filename, PathBuf::from("中文筆記.txt"));
        assert!(settings.ignore_case);
    }

    #[test]
    fn missing_positionals_and_unknown_flags_are_usage_errors() {
        match Settings::try_parse_from([OsString::from("query-settings"), OsString::from("Rust")]) {
            Err(error) => {
                assert!(error.use_stderr());
                assert_eq!(error.exit_code(), 2);
            }
            Ok(settings) => panic!("缺少檔名卻得到設定：{settings:?}"),
        }
        match Settings::try_parse_from([
            OsString::from("query-settings"),
            OsString::from("Rust"),
            OsString::from("note.txt"),
            OsString::from("--unknown"),
        ]) {
            Err(error) => {
                assert!(error.use_stderr());
                assert_eq!(error.exit_code(), 2);
            }
            Ok(settings) => panic!("未知開關卻得到設定：{settings:?}"),
        }
    }

    #[test]
    fn help_and_version_are_successful_display_requests() {
        for request in ["--help", "--version"] {
            match Settings::try_parse_from([
                OsString::from("query-settings"),
                OsString::from(request),
            ]) {
                Err(error) => {
                    assert!(!error.use_stderr());
                    assert_eq!(error.exit_code(), 0);
                }
                Ok(settings) => panic!("顯示請求卻得到搜尋設定：{settings:?}"),
            }
        }
    }
}
// --8<-- [end:settings-tests]
