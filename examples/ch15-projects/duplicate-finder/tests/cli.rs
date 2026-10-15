use duplicate_finder::Report;
use duplicate_finder::render_text;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use std::process::Output;
use std::str;
use tempfile::tempdir;

// --8<-- [start:child-fixture]
fn run_cli(directory: &Path, root: &Path, json: bool) -> io::Result<Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_duplicate-finder"));
    command.current_dir(directory);
    command.arg(root.as_os_str());
    if json {
        command.arg(OsString::from("--json").as_os_str());
    }
    command.output()
}

fn materials(directory: &Path) -> io::Result<()> {
    for (name, text) in [
        ("a.txt", "Rust\n"),
        ("中文 b.txt", "Rust\n"),
        (".hidden.txt", "Rust\n"),
        ("different.txt", "rust\n"),
        ("empty-a", ""),
        ("empty-b", ""),
    ] {
        fs::write(directory.join(name).as_path(), text)?;
    }
    Ok(())
}
// --8<-- [end:child-fixture]

#[test]
fn real_cli_text_and_json_preserve_the_same_complete_data() {
    let directory = tempdir().expect("可建立自有 fixture");
    materials(directory.path()).expect("可建立六份已知 bytes");
    let ordinary = run_cli(directory.path(), Path::new("."), false).expect("child 可啟動");
    assert_eq!(ordinary.status.code(), Some(0));
    assert_eq!(ordinary.stderr.len(), 0);
    let ordinary_text = str::from_utf8(ordinary.stdout.as_slice()).expect("文字報告 UTF8");
    let json = run_cli(directory.path(), Path::new("."), true).expect("同 root child 可啟動");
    assert_eq!(json.status.code(), Some(0));
    assert_eq!(json.stderr.len(), 0);
    let encoded = str::from_utf8(json.stdout.as_slice()).expect("JSON UTF8");
    let report = serde_json::from_str::<Report>(encoded).expect("整份 stdout 是 JSON");
    assert!(report.complete);
    assert_eq!(report.duplicate_bytes, 10);
    assert_eq!(report.groups.len(), 2);
    assert_eq!(report.scan_issues.len(), 0);
    assert_eq!(report.file_issues.len(), 0);
    assert_eq!(ordinary_text, render_text(&report).as_str());
}

#[test]
fn empty_root_is_complete_zero_not_a_fatal_or_partial_result() {
    let directory = tempdir().expect("可建立空 fixture");
    let output = run_cli(directory.path(), Path::new("."), true).expect("child 可啟動");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stderr.len(), 0);
    let text = str::from_utf8(output.stdout.as_slice()).expect("JSON UTF8");
    let report = serde_json::from_str::<Report>(text).expect("完整 stdout 是 JSON");
    assert!(report.complete);
    assert_eq!(report.groups.len(), 0);
    assert_eq!(report.duplicate_bytes, 0);
}

#[test]
fn missing_and_file_roots_are_fatal_with_no_report_on_stdout() {
    let directory = tempdir().expect("可建立自有 fixture");
    fs::write(directory.path().join("file").as_path(), "data").expect("可建一般檔案");
    for name in ["missing", "file"] {
        let output = run_cli(directory.path(), Path::new(name), true).expect("child 正常完成失敗");
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(output.stdout.len(), 0);
        let diagnostic = str::from_utf8(output.stderr.as_slice()).expect("診斷 UTF8");
        assert!(diagnostic.starts_with("無法完成 root="));
    }
}

#[test]
fn usage_help_and_version_keep_their_separate_channels_and_statuses() {
    for request in [None, Some("--unknown"), Some("--help"), Some("--version")] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_duplicate-finder"));
        if let Some(request) = request {
            command.arg(OsString::from(request).as_os_str());
        }
        let output = command.output().expect("Cargo child 可啟動");
        if request == Some("--help") || request == Some("--version") {
            assert_eq!(output.status.code(), Some(0));
            assert_eq!(output.stderr.len(), 0);
            let text = str::from_utf8(output.stdout.as_slice()).expect("說明 UTF8");
            if request == Some("--help") {
                assert!(text.contains("--json"));
                assert!(text.contains("ROOT"));
            } else {
                assert!(text.starts_with("duplicate-finder 0.1.0"));
            }
        } else {
            assert_eq!(output.status.code(), Some(2));
            assert_eq!(output.stdout.len(), 0);
            assert!(!output.stderr.as_slice().is_empty());
        }
    }
}

#[cfg(unix)]
#[test]
fn non_unicode_item_makes_a_real_partial_report_and_exit_one() {
    use std::os::unix::ffi::OsStringExt;

    let directory = tempdir().expect("可建立自有 fixture");
    materials(directory.path()).expect("可建立成功部分");
    let name = OsString::from_vec(vec![255]);
    fs::write(directory.path().join(name.as_os_str()).as_path(), "other").expect("可建原生名字");
    let output = run_cli(directory.path(), Path::new("."), true).expect("child 可啟動");
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stderr.len(), 0);
    let text = str::from_utf8(output.stdout.as_slice()).expect("JSON UTF8");
    let report = serde_json::from_str::<Report>(text).expect("部分結果仍是完整 JSON");
    assert!(!report.complete);
    assert_eq!(report.groups.len(), 2);
    assert_eq!(report.duplicate_bytes, 10);
    assert_eq!(report.scan_issues.len(), 1);
    let native = Path::new(".").join(name.as_os_str());
    assert_eq!(
        report.scan_issues.as_slice()[0].path_debug,
        Some(format!("{native:?}"))
    );
}

#[cfg(unix)]
#[test]
fn a_symbolic_root_is_rejected_by_the_real_cli() {
    use std::os::unix::fs::symlink;

    let directory = tempdir().expect("可建立自有 fixture");
    let link = directory.path().join("root-link");
    symlink(directory.path(), link.as_path()).expect("Unix 可建自有 symbolic link");
    let output = run_cli(directory.path(), Path::new("root-link"), false).expect("child 可啟動");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    let text = str::from_utf8(output.stderr.as_slice()).expect("診斷 UTF8");
    assert!(text.contains("symbolic link"));
}

#[cfg(unix)]
#[test]
fn inner_links_are_skipped_and_hard_links_count_distinct_paths() {
    use std::os::unix::fs::symlink;

    let owner = tempdir().expect("可建立自有 fixture");
    let root = owner.path().join("root");
    let outside = owner.path().join("outside");
    fs::create_dir(root.as_path()).expect("可建 root");
    fs::create_dir(outside.as_path()).expect("可建外側目錄");
    let file = root.as_path().join("a");
    fs::write(file.as_path(), "ABC").expect("可建 root 文件");
    fs::write(outside.as_path().join("b").as_path(), "ABC").expect("可建外側文件");
    symlink(outside.as_path(), root.as_path().join("dir-link").as_path()).expect("可建目錄link");
    symlink(
        outside.as_path().join("b").as_path(),
        root.as_path().join("file-link").as_path(),
    )
    .expect("可建檔案link");
    let first = run_cli(owner.path(), root.as_path(), true).expect("child 可啟動");
    assert_eq!(first.status.code(), Some(0));
    let first_text = str::from_utf8(first.stdout.as_slice()).expect("JSON UTF8");
    assert_eq!(
        serde_json::from_str::<Report>(first_text)
            .expect("完整 JSON")
            .groups
            .len(),
        0
    );
    fs::hard_link(file.as_path(), root.as_path().join("second").as_path())
        .expect("可建第二原生路徑");
    let second = run_cli(owner.path(), root.as_path(), true).expect("child 可啟動");
    assert_eq!(second.status.code(), Some(0));
    let second_text = str::from_utf8(second.stdout.as_slice()).expect("JSON UTF8");
    let report = serde_json::from_str::<Report>(second_text).expect("完整 JSON");
    assert_eq!(report.groups.len(), 1);
    assert_eq!(report.groups.as_slice()[0].paths.len(), 2);
    assert_eq!(report.duplicate_bytes, 3);
}
