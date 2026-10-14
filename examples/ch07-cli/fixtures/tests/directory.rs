use std::fs;
use tempfile::tempdir;

#[test]
fn fixture_has_known_contents_and_explicit_cleanup_is_checked() {
    let directory = tempdir().expect("測試需要能建立專用臨時目錄");
    let saved_path = directory.path().to_path_buf();
    let filename = directory.path().join("中文 筆記.txt");
    fs::write(filename.as_path(), "Rust\n中\n").expect("測試需要能寫入自有目錄");
    let text = fs::read_to_string(filename.as_path()).expect("剛寫入的文字可讀回");
    assert_eq!(text.as_str(), "Rust\n中\n");
    assert!(saved_path.as_path().try_exists().expect("查詢自有目錄"));

    directory.close().expect("測試要確認清理成功");
    assert!(!saved_path.as_path().try_exists().expect("查詢原目錄位置"));
}
