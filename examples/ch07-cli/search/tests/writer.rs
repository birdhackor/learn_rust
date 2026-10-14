use ch07_search::error::AppError;
use ch07_search::search::search;
use ch07_search::write_report;
use std::io;
use std::io::Write;

struct RefusingWriter {
    called: bool,
}

impl Write for RefusingWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        self.called = true;
        Err(io::Error::from(io::ErrorKind::PermissionDenied))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn a_failed_output_is_not_misreported_as_a_read_failure() {
    let report = search("Rust", "Rust\n", false).expect("搜尋文字非空");
    let mut writer = RefusingWriter { called: false };
    match write_report(&report, false, &mut writer) {
        Err(AppError::Output(error)) => {
            assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        }
        other => panic!("資料已建立，應是輸出失敗：{other:?}"),
    }
    assert!(writer.called);
}
