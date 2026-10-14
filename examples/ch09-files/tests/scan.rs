use ch09_files::scan::RootError;
use ch09_files::scan::ScanCause;
use ch09_files::scan::collect_entries;
use ch09_files::scan::observe;
use ch09_files::scan::scan;
use std::fs;
use std::io;
use tempfile::tempdir;
use walkdir::WalkDir;

#[test]
fn controlled_walker_failure_is_retained_between_successful_entries() {
    let directory = tempdir().expect("可建立測試目錄");
    let first = directory.path().join("a.txt");
    let last = directory.path().join("z.txt");
    let missing = directory.path().join("missing");
    fs::write(first.as_path(), "A").expect("可建立第一份檔案");
    fs::write(last.as_path(), "Z").expect("可建立最後一份檔案");
    let first_item = WalkDir::new(first.as_path())
        .into_iter()
        .next()
        .expect("root 一般檔案產生一筆 item");
    let failure_item = WalkDir::new(missing.as_path())
        .into_iter()
        .next()
        .expect("不存在的 root 產生一筆 Err item");
    let last_item = WalkDir::new(last.as_path())
        .into_iter()
        .next()
        .expect("root 一般檔案產生一筆 item");
    let report = collect_entries(vec![first_item, failure_item, last_item]);
    assert_eq!(report.files.len(), 2);
    assert_eq!(report.issues.len(), 1);
    let issue = &report.issues.as_slice()[0];
    assert_eq!(issue.path, Some(missing));
    assert_eq!(issue.action, "走訪目錄");
    match &issue.cause {
        ScanCause::Walk(_) => (),
        _ => panic!("應保留 walkdir 原始失敗"),
    }
}

#[test]
fn nested_and_hidden_files_are_sorted_without_reading_their_contents() {
    let directory = tempdir().expect("可建立測試目錄");
    let nested = directory.path().join(".hidden");
    fs::create_dir(nested.as_path()).expect("父目錄已存在");
    let first = nested.as_path().join("a.txt");
    let second = directory.path().join("z.txt");
    fs::write(second.as_path(), "ABCDE").expect("可建立一般檔案");
    fs::write(first.as_path(), "").expect("可建立空檔");
    let report = scan(directory.path()).expect("有效根目錄");
    assert_eq!(report.files.len(), 2);
    assert_eq!(report.issues.len(), 0);
    assert_eq!(report.files.as_slice()[0].path, first);
    assert_eq!(report.files.as_slice()[0].snapshot.len, 0);
    assert_eq!(report.files.as_slice()[1].path, second);
    assert_eq!(report.files.as_slice()[1].snapshot.len, 5);
}

#[test]
fn empty_directory_is_a_complete_empty_result() {
    let directory = tempdir().expect("可建立測試目錄");
    let report = scan(directory.path()).expect("有效根目錄");
    assert_eq!(report.files.len(), 0);
    assert_eq!(report.issues.len(), 0);
}

#[test]
fn root_file_and_missing_root_are_distinct_failures() {
    let directory = tempdir().expect("可建立測試目錄");
    let file = directory.path().join("file.txt");
    fs::write(file.as_path(), "A").expect("可建立一般檔案");
    match scan(file.as_path()) {
        Err(RootError::NotDirectory) => (),
        _ => panic!("root 是一般檔案時應拒絕設定"),
    }
    let missing = directory.path().join("missing");
    match scan(missing.as_path()) {
        Err(RootError::Io(error)) => assert_eq!(error.kind(), io::ErrorKind::NotFound),
        _ => panic!("root 不存在時應保留 I/O 原因"),
    }
}

#[test]
fn observation_rejects_a_directory_as_a_file() {
    let directory = tempdir().expect("可建立測試目錄");
    match observe(directory.path()) {
        Err(ScanCause::NotRegular) => (),
        _ => panic!("目錄不能取得一般檔案記錄"),
    }
}

#[cfg(unix)]
#[test]
fn root_link_is_rejected_before_walking() {
    use std::os::unix::fs::symlink;

    let directory = tempdir().expect("可建立測試目錄");
    let target = directory.path().join("target");
    fs::create_dir(target.as_path()).expect("父目錄存在");
    let link = directory.path().join("root-link");
    symlink(target.as_path(), link.as_path()).expect("Unix 測試可建立 link");
    match scan(link.as_path()) {
        Err(RootError::SymbolicLink) => (),
        _ => panic!("即使 link 指向有效目錄，root link 也要拒絕"),
    }
}

#[cfg(unix)]
#[test]
fn inner_file_and_directory_links_are_not_followed() {
    use std::os::unix::fs::symlink;

    let directory = tempdir().expect("可建立測試目錄");
    let root = directory.path().join("root");
    let outside = directory.path().join("outside");
    fs::create_dir(root.as_path()).expect("父目錄存在");
    fs::create_dir(outside.as_path()).expect("父目錄存在");
    let file = outside.as_path().join("outside.txt");
    fs::write(file.as_path(), "A").expect("可建立檔案");
    let file_link = root.as_path().join("file-link");
    let directory_link = root.as_path().join("directory-link");
    symlink(file.as_path(), file_link.as_path()).expect("可建立檔案 link");
    symlink(outside.as_path(), directory_link.as_path()).expect("可建立目錄 link");
    let report = scan(root.as_path()).expect("root 是一般目錄");
    assert_eq!(report.files.len(), 0);
    assert_eq!(report.issues.len(), 0);
}

#[cfg(unix)]
#[test]
fn non_unicode_path_is_reported_with_its_original_native_identity() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let directory = tempdir().expect("可建立測試目錄");
    let native = OsString::from_vec(vec![255]);
    let invalid = directory.path().join(native);
    fs::write(invalid.as_path(), "A").expect("Unix 可保存這份原生檔名");
    let good = directory.path().join("good.txt");
    fs::write(good.as_path(), "B").expect("可建立 Unicode 檔名");
    let report = scan(directory.path()).expect("root 是有效目錄");
    assert_eq!(report.files.len(), 1);
    assert_eq!(report.files.as_slice()[0].path, good);
    assert_eq!(report.issues.len(), 1);
    let issue = &report.issues.as_slice()[0];
    assert_eq!(issue.path, Some(invalid));
    assert_eq!(issue.action, "準備報告路徑");
    match &issue.cause {
        ScanCause::NonUnicode => (),
        _ => panic!("應保留非 Unicode 原因"),
    }
}
