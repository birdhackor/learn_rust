use ch09_files::scan::scan;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let report =
        scan(Path::new("examples/ch09-files/fixtures/tree")).expect("教材固定目錄是有效的一般目錄");
    let mut sizes: HashMap<PathBuf, u64> = HashMap::new();
    for record in report.files {
        sizes.insert(record.path, record.snapshot.len);
    }
    let nested = Path::new("examples/ch09-files/fixtures/tree/nested/second.txt");
    assert_eq!(sizes.get(nested), Some(&5));
    println!("保留原生 Path 作查詢鍵，取得 5 bytes");
    println!("項目失敗數={}", report.issues.len());
}
