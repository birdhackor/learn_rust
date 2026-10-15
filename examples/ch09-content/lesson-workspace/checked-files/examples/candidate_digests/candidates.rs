use blake3::Hash;
use ch09_files::scan::FileRecord;
use lesson_checked_files::FileIssue;
use lesson_checked_files::hash_file;
use std::collections::HashMap;

pub type DigestBuckets = HashMap<(u64, Hash), Vec<FileRecord>>;

pub fn hash_candidates(records: Vec<FileRecord>) -> Result<DigestBuckets, FileIssue> {
    let mut buckets: DigestBuckets = HashMap::new();
    for record in records {
        let digest = hash_file(&record)?;
        let key = (record.snapshot.len, digest);
        match buckets.get_mut(&key) {
            Some(bucket) => bucket.push(record),
            None => {
                buckets.insert(key, vec![record]);
            }
        }
    }
    Ok(buckets)
}

#[cfg(test)]
mod tests {
    use super::hash_candidates;
    use ch09_files::scan::FileRecord;
    use ch09_files::scan::observe;
    use lesson_checked_files::hash_file;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    fn record(path: &Path) -> FileRecord {
        FileRecord {
            path: path.to_path_buf(),
            snapshot: observe(path).expect("固定材料可取得 metadata 與修改時間"),
        }
    }

    #[test]
    fn real_candidate_keys_use_both_length_and_digest() {
        let directory = tempdir().expect("可建立測試目錄");
        let mut records = Vec::new();
        for (name, contents) in [("a", "AAA"), ("b", "AAA"), ("c", "BBB"), ("d", "AAAA")] {
            let path = directory.path().join(name);
            fs::write(path.as_path(), contents).expect("可寫材料");
            records.push(record(path.as_path()));
        }
        let first_digest = hash_file(&records.as_slice()[0]).expect("固定穩定材料");
        let buckets = hash_candidates(records).expect("固定穩定材料");
        assert_eq!(buckets.len(), 3);
        assert_eq!(
            buckets
                .get(&(3, first_digest))
                .expect("兩份 AAA 同桶")
                .len(),
            2
        );
        for ((len, _digest), candidates) in buckets {
            for candidate in candidates {
                assert_eq!(candidate.snapshot.len, len);
            }
        }
    }

    #[test]
    fn failed_hash_candidate_does_not_return_a_partial_success_map() {
        let directory = tempdir().expect("可建立測試目錄");
        let path = directory.path().join("a");
        fs::write(path.as_path(), "AAA").expect("可寫材料");
        let first = record(path.as_path());
        let missing = directory.path().join("missing");
        let missing_record = FileRecord {
            path: missing.as_path().to_path_buf(),
            snapshot: first.snapshot,
        };
        match hash_candidates(vec![first, missing_record]) {
            Err(failure) => assert_eq!(failure.path, missing),
            Ok(_) => panic!("首個成功不能變成完整成功 map"),
        }
    }
}
