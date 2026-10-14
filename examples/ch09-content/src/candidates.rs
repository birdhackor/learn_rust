use crate::files::FileIssue;
use crate::files::hash_file;
use blake3::Hash;
use ch09_files::scan::FileRecord;
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
