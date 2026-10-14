use ch09_content::candidates::hash_candidates;
use ch09_files::scan::FileRecord;
use ch09_files::scan::scan;
use std::path::Path;

fn main() {
    let report = scan(Path::new("examples/ch09-files/fixtures/tree")).expect("教材固定根目錄有效");
    let selected: Vec<FileRecord> = report
        .files
        .into_iter()
        .filter(|record: &FileRecord| record.snapshot.len == 5)
        .collect();
    let buckets = hash_candidates(selected).expect("固定的 5-byte 候選穩定可讀");
    assert_eq!(buckets.len(), 1);
    for ((len, digest), candidates) in buckets {
        println!("{len} bytes，digest={digest}，候選數={}", candidates.len());
    }
    println!(
        "目前只形成候選，尚未判定重複；掃描失敗數={}",
        report.issues.len()
    );
}
