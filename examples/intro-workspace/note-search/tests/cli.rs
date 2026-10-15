// --8<-- [start:cli-helper]
use intro_note_search::model::Report;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Output;
use std::str;
use tempfile::tempdir;

fn execute(directory: &Path, arguments: &[OsString]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_intro-note-search"));
    command.current_dir(directory);
    for argument in arguments {
        command.arg(argument.as_os_str());
    }
    command.output().expect("Cargo 建好的工具能啟動並收集輸出")
}

fn assert_failure(output: &Output, code: i32, prefix: &str) {
    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(code));
    assert_eq!(output.stdout.len(), 0);
    let stderr = str::from_utf8(output.stderr.as_slice()).expect("診斷應是 UTF-8");
    assert!(stderr.starts_with(prefix));
}
// --8<-- [end:cli-helper]

// --8<-- [start:cli-success-tests]
#[test]
fn normal_and_ascii_search_keep_order_and_original_spelling() {
    let directory = tempdir().expect("測試建立專用臨時目錄");
    let filename = directory.path().join("中文 筆記.txt");
    fs::write(
        filename.as_path(),
        "Rust 很清楚\nPython\nrust 也有資料\n中有 Rust\n",
    )
    .expect("測試寫入已知 UTF-8 內容");
    let arguments = [OsString::from("Rust"), OsString::from("中文 筆記.txt")];
    let ordinary = execute(directory.path(), &arguments[..]);
    assert_eq!(ordinary.status.code(), Some(0));
    assert_eq!(ordinary.stderr.len(), 0);
    assert_eq!(
        str::from_utf8(ordinary.stdout.as_slice()).expect("資料應是 UTF-8"),
        "1:Rust 很清楚\n4:中有 Rust\n"
    );
    let arguments = [
        OsString::from("RUST"),
        OsString::from("中文 筆記.txt"),
        OsString::from("--ignore-case"),
    ];
    let folded = execute(directory.path(), &arguments[..]);
    assert_eq!(folded.status.code(), Some(0));
    assert_eq!(folded.stderr.len(), 0);
    assert_eq!(
        str::from_utf8(folded.stdout.as_slice()).expect("資料應是 UTF-8"),
        "1:Rust 很清楚\n3:rust 也有資料\n4:中有 Rust\n"
    );
    directory.close().expect("child 已結束，清理成功");
}

// --8<-- [start:json-cli-test]
#[test]
fn json_stdout_is_only_json_and_preserves_quote_data() {
    let directory = tempdir().expect("測試建立專用臨時目錄");
    let filename = directory.path().join("note.txt");
    fs::write(filename.as_path(), "Python\nRust \"筆記\"\n").expect("測試寫入已知內容");
    let arguments = [
        OsString::from("Rust"),
        OsString::from("note.txt"),
        OsString::from("--json"),
    ];
    let output = execute(directory.path(), &arguments[..]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stderr.len(), 0);
    let text = str::from_utf8(output.stdout.as_slice()).expect("JSON 應是 UTF-8");
    let report = serde_json::from_str::<Report>(text).expect("完整 stdout 只能是合法報告 JSON");
    assert_eq!(report.query.as_str(), "Rust");
    assert!(!report.ignore_case);
    assert_eq!(report.matches.len(), 1);
    assert_eq!(report.matches.as_slice()[0].line_number, 2);
    assert_eq!(report.matches.as_slice()[0].text.as_str(), "Rust \"筆記\"");
    directory.close().expect("child 已結束，清理成功");
}
// --8<-- [end:json-cli-test]

#[test]
fn no_match_is_success_in_plain_text_and_json() {
    let directory = tempdir().expect("測試建立專用臨時目錄");
    let filename = directory.path().join("note.txt");
    fs::write(filename.as_path(), "Python\n").expect("測試寫入已知內容");
    let arguments = [OsString::from("Rust"), OsString::from("note.txt")];
    let plain = execute(directory.path(), &arguments[..]);
    assert_eq!(plain.status.code(), Some(0));
    assert_eq!(plain.stdout.len(), 0);
    assert_eq!(plain.stderr.len(), 0);
    let arguments = [
        OsString::from("Rust"),
        OsString::from("note.txt"),
        OsString::from("--json"),
    ];
    let json = execute(directory.path(), &arguments[..]);
    assert_eq!(json.status.code(), Some(0));
    assert_eq!(json.stderr.len(), 0);
    let text = str::from_utf8(json.stdout.as_slice()).expect("JSON 應是 UTF-8");
    let report = serde_json::from_str::<Report>(text).expect("空結果仍是合法報告");
    assert_eq!(report.matches.len(), 0);
    directory.close().expect("child 已結束，清理成功");
}
// --8<-- [end:cli-success-tests]

// --8<-- [start:cli-error-tests]
#[test]
fn io_failures_and_empty_query_keep_distinct_statuses_and_no_data() {
    let directory = tempdir().expect("測試建立專用臨時目錄");
    let invalid = directory.path().join("invalid.bin");
    let bytes: [u8; 1] = [255];
    fs::write(invalid.as_path(), &bytes[..]).expect("測試建立非法 UTF-8");
    for filename in ["missing.txt", "invalid.bin"] {
        let arguments = [OsString::from("Rust"), OsString::from(filename)];
        assert_failure(
            &execute(directory.path(), &arguments[..]),
            1,
            "文字讀取失敗：",
        );
    }
    let arguments = [OsString::from(""), OsString::from("missing.txt")];
    assert_failure(
        &execute(directory.path(), &arguments[..]),
        2,
        "搜尋文字不能是空字串",
    );
    directory.close().expect("child 已結束，清理成功");
}

#[test]
fn parser_usage_errors_and_display_requests_have_their_own_streams() {
    let directory = tempdir().expect("測試建立專用臨時目錄");
    let arguments = [OsString::from("Rust")];
    assert_failure(&execute(directory.path(), &arguments[..]), 2, "error:");
    let arguments = [
        OsString::from("Rust"),
        OsString::from("note.txt"),
        OsString::from("--unknown"),
    ];
    assert_failure(&execute(directory.path(), &arguments[..]), 2, "error:");
    for request in ["--help", "--version"] {
        let arguments = [OsString::from(request)];
        let output = execute(directory.path(), &arguments[..]);
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(output.stderr.len(), 0);
        assert!(!output.stdout.as_slice().is_empty());
    }
    directory.close().expect("child 已結束，清理成功");
}
// --8<-- [end:cli-error-tests]
