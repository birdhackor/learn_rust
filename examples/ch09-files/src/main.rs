use ch09_files::scan::scan;
use std::path::Path;

fn main() {
    let root = Path::new("examples/ch09-files/fixtures/tree");
    match scan(root) {
        Ok(report) => {
            for record in report.files {
                println!(
                    "{}：{} bytes",
                    record.path.as_path().display(),
                    record.snapshot.len
                );
            }
            for issue in &report.issues {
                println!(
                    "項目失敗：{}；{:?}；{}",
                    issue.action, issue.path, issue.cause
                );
            }
            println!("掃描完整={}", report.issues.as_slice().is_empty());
        }
        Err(error) => println!("根設定失敗：{error:?}"),
    }
}
