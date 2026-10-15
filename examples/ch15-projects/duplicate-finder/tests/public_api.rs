use duplicate_finder::RootError;
use duplicate_finder::RunError;
use duplicate_finder::scan_report;
use std::error::Error;
use std::io;
use tempfile::tempdir;

#[test]
fn public_root_error_keeps_typed_reason_and_shared_source() {
    let owner = tempdir().expect("可建立自有 fixture");
    let error = match scan_report(owner.path().join("missing").as_path()) {
        Err(error) => error,
        Ok(_) => panic!("缺 root 不應有報告"),
    };
    match &error {
        RunError::Root(RootError::Io(cause)) => {
            assert_eq!(cause.kind(), io::ErrorKind::NotFound);
            let shared = error.source().expect("Root Io 保存標準原因");
            assert_eq!(format!("{shared}"), format!("{cause}"));
        }
        other => panic!("保留非 exhaustive 其他分支：{other:?}"),
    }
    let leaf = RunError::Root(RootError::NotDirectory);
    match leaf.source() {
        None => (),
        Some(cause) => panic!("無下層的 leaf：{cause}"),
    }
}
