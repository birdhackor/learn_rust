use blake3::Hash;
use blake3::Hasher;
use ch09_content::groups::add_duplicate_bytes;
use ch09_content::groups::duplicate_bytes;
use ch09_content::groups::refine_bucket;
use ch09_files::scan::FileRecord;
use ch09_files::scan::observe;
use std::collections::HashMap;
use std::fs;
use tempfile::tempdir;

fn main() {
    let directory = tempdir().expect("可建立練習專用目錄");
    let forced_digest = Hasher::new().finalize();
    let mut buckets: HashMap<Hash, Vec<FileRecord>> = HashMap::new();
    for (name, contents) in [("a.txt", "AAA"), ("b2.txt", "BBB"), ("b1.txt", "BBB")] {
        let path = directory.path().join(name);
        fs::write(path.as_path(), contents).expect("可建立固定練習材料");
        let snapshot = observe(path.as_path()).expect("可取得固定檔案資料");
        let record = FileRecord { path, snapshot };
        match buckets.get_mut(&forced_digest) {
            Some(bucket) => bucket.push(record),
            None => {
                buckets.insert(forced_digest, vec![record]);
            }
        }
    }
    let candidates = buckets.get(&forced_digest).expect("三份記錄使用同一測試鍵");
    let groups = refine_bucket(candidates.as_slice()).expect("固定穩定材料可讀");
    assert_eq!(groups.len(), 2);
    assert_eq!(groups.as_slice()[0].members.len(), 1);
    assert_eq!(groups.as_slice()[1].members.len(), 2);
    let mut total = 0;
    for group in &groups {
        let len = group.members.as_slice()[0].snapshot.len;
        let bytes = duplicate_bytes(len, group.members.len()).expect("小數字可表示");
        total = add_duplicate_bytes(total, bytes).expect("小數字可表示");
        println!(
            "群組路徑數={}，按路徑的重複內容量={bytes} bytes",
            group.members.len()
        );
        for member in &group.members {
            println!("  {}", member.path.as_path().display());
        }
    }
    assert_eq!(total, 3);
    println!("合計重複內容量={total} bytes");
}
