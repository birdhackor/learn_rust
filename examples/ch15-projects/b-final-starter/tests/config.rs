use clap::Parser;
use std::ffi::OsString;
use std::path::PathBuf;
use url_checker_starter::Settings;

#[test]
fn defaults_and_explicit_numbers_keep_the_native_filename() {
    let settings = Settings::try_parse_from([
        OsString::from("url-checker"),
        OsString::from("中文 清單.txt"),
    ])
    .expect("必要檔名");
    assert_eq!(settings.filename, PathBuf::from("中文 清單.txt"));
    assert_eq!(settings.jobs, 8);
    assert_eq!(settings.timeout_secs, 10);
    assert!(!settings.json);
    let changed = Settings::try_parse_from([
        OsString::from("url-checker"),
        OsString::from("中文 清單.txt"),
        OsString::from("--jobs"),
        OsString::from("2"),
        OsString::from("--timeout-secs"),
        OsString::from("5"),
        OsString::from("--json"),
    ])
    .expect("已宣告的數值與開關");
    assert_eq!(changed.jobs, 2);
    assert_eq!(changed.timeout_secs, 5);
    assert!(changed.json);
    assert_eq!(changed.filename, settings.filename);
}

#[test]
fn help_version_are_display_requests_and_invalid_numbers_are_usage_errors() {
    for request in ["--help", "--version"] {
        match Settings::try_parse_from([OsString::from("url-checker"), OsString::from(request)]) {
            Err(error) => {
                assert!(!error.use_stderr());
                assert_eq!(error.exit_code(), 0);
            }
            Ok(value) => panic!("資訊請求不是工作設定：{value:?}"),
        }
    }
    match Settings::try_parse_from([
        OsString::from("url-checker"),
        OsString::from("urls.txt"),
        OsString::from("--jobs"),
        OsString::from("many"),
    ]) {
        Err(error) => {
            assert!(error.use_stderr());
            assert_eq!(error.exit_code(), 2);
        }
        Ok(value) => panic!("無法解析成整數：{value:?}"),
    }
}
