// --8<-- [start:cli-helper]
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use std::process::Output;
use std::str;
use tempfile::tempdir;

fn run_note(directory: &Path, filename: &Path) -> io::Result<Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_intro-cli-fixtures"));
    command.current_dir(directory);
    command.arg(filename.as_os_str());
    command.output()
}
// --8<-- [end:cli-helper]

// --8<-- [start:success-test]
#[test]
fn unicode_filename_with_a_space_preserves_all_stdout_data() {
    let directory = tempdir().expect("測試需要能建立專用臨時目錄");
    let filename = directory.path().join("中文 筆記.txt");
    fs::write(filename.as_path(), "Rust\n中\n").expect("測試需要能寫入自有目錄");

    let output = run_note(directory.path(), Path::new("中文 筆記.txt"))
        .expect("Cargo 建好的程式應能啟動並收集輸出");
    assert!(output.status.success());
    assert_eq!(output.status.code(), Some(0));
    let stdout = str::from_utf8(output.stdout.as_slice()).expect("工具的筆記輸出應是 UTF-8");
    let stderr = str::from_utf8(output.stderr.as_slice()).expect("工具的診斷應是 UTF-8");
    assert_eq!(stdout, "Rust\n中\n");
    assert_eq!(stderr, "");
    directory.close().expect("子行程已結束，檢查清理成功");
}
// --8<-- [end:success-test]

// --8<-- [start:failure-tests]
#[test]
fn missing_file_and_invalid_utf8_are_completed_child_failures() {
    let directory = tempdir().expect("測試需要能建立專用臨時目錄");
    let invalid = directory.path().join("invalid.bin");
    let bytes: [u8; 1] = [255];
    fs::write(invalid.as_path(), &bytes[..]).expect("測試寫入一個非法 UTF-8 byte");

    for name in ["missing.txt", "invalid.bin"] {
        let output =
            run_note(directory.path(), Path::new(name)).expect("失敗的是工具讀取，不是啟動子行程");
        assert!(!output.status.success());
        assert_eq!(output.status.code(), Some(1));
        assert_eq!(output.stdout.len(), 0);
        let stderr = str::from_utf8(output.stderr.as_slice()).expect("診斷應是 UTF-8");
        assert!(stderr.starts_with("無法讀取 "));
    }
    directory.close().expect("子行程已結束，檢查清理成功");
}

#[test]
fn no_filename_is_a_usage_error_with_no_data_on_stdout() {
    let directory = tempdir().expect("測試需要能建立專用臨時目錄");
    let mut command = Command::new(env!("CARGO_BIN_EXE_intro-cli-fixtures"));
    command.current_dir(directory.path());
    let output = command.output().expect("Cargo 建好的程式能啟動");
    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    let stderr = str::from_utf8(output.stderr.as_slice()).expect("診斷應是 UTF-8");
    assert_eq!(stderr, "需要一個檔名\n");
    directory.close().expect("子行程已結束，檢查清理成功");
}

#[test]
fn absent_executable_is_an_io_error_without_output_or_exit_status() {
    let directory = tempdir().expect("測試需要能建立專用臨時目錄");
    let absent_program = directory.path().join("no-such-program");
    let mut command = Command::new(absent_program.as_path().as_os_str());
    match command.output() {
        Err(error) => assert_eq!(error.kind(), io::ErrorKind::NotFound),
        Ok(output) => panic!("自有目錄沒有此程式，卻成功取得 {output:?}"),
    }
    directory.close().expect("檢查清理成功");
}
// --8<-- [end:failure-tests]

// --8<-- [start:empty-solution]
#[test]
fn an_empty_file_is_successful_empty_data_not_a_read_failure() {
    let directory = tempdir().expect("測試需要能建立專用臨時目錄");
    let filename = directory.path().join("empty.txt");
    fs::write(filename.as_path(), "").expect("測試建立空檔");
    let output = run_note(directory.path(), Path::new("empty.txt"))
        .expect("Cargo 建好的程式能啟動並收集輸出");
    assert!(output.status.success());
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout.len(), 0);
    assert_eq!(output.stderr.len(), 0);
    directory.close().expect("子行程已結束，檢查清理成功");
}
// --8<-- [end:empty-solution]
