use duplicate_a1::collection::scan;
use std::path::Path;

fn main() {
    // 操作入口固定為 examples/ch15-projects/a1-complete。
    let root = Path::new("../materials/duplicate-demo");
    let report = match scan(root) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("A1 根目錄失敗：{error:?}");
            return;
        }
    };
    println!(
        "A1：{} 個待判定檔案，{} 個項目問題",
        report.files.len(),
        report.issues.len()
    );
    for file in report.files {
        println!("{} bytes {}", file.snapshot.len, file.path.display());
    }
    for issue in report.issues {
        eprintln!("{} {:?}：{}", issue.action, issue.path, issue.cause);
    }
}
