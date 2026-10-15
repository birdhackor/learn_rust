use ch09_files::scan::FileRecord;
use ch09_files::scan::scan;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let report = scan(Path::new("examples/ch09-files/fixtures/tree")).expect("教材固定根目錄有效");
    let mut by_size: HashMap<u64, Vec<&FileRecord>> = HashMap::new();
    for record in &report.files {
        match by_size.get_mut(&record.snapshot.len) {
            Some(bucket) => bucket.push(record),
            None => {
                by_size.insert(record.snapshot.len, vec![record]);
            }
        }
    }
    let mut rows: Vec<(u64, Vec<&FileRecord>)> = by_size.into_iter().collect();
    rows.as_mut_slice()
        .sort_by_key(|row: &(u64, Vec<&FileRecord>)| row.0);
    for (len, candidates) in rows {
        if candidates.len() >= 2 {
            println!("{len} bytes 的候選數={}", candidates.len());
        }
    }
    println!("掃描項目失敗數={}", report.issues.len());
}
