use crate::collection::FileRecord;
use crate::collection::ScanIssue;
use crate::collection::ScanReport;
use crate::files::FileIssue;
use crate::files::equal_files;
use crate::files::hash_file;
use blake3::Hash;
use std::collections::HashMap;
use std::path::PathBuf;

pub type DigestBuckets = HashMap<(u64, Hash), Vec<FileRecord>>;

pub struct ContentGroup<'a> {
    pub members: Vec<&'a FileRecord>,
}

#[derive(Debug, PartialEq)]
pub struct DuplicateGroup {
    pub len: u64,
    pub paths: Vec<PathBuf>,
}

pub struct ContentReport {
    pub groups: Vec<DuplicateGroup>,
    pub scan_issues: Vec<ScanIssue>,
    pub file_issues: Vec<FileIssue>,
}

pub fn digest_candidates<H: FnMut(&FileRecord) -> Result<Hash, FileIssue>>(
    records: Vec<FileRecord>,
    mut hash: H,
    issues: &mut Vec<FileIssue>,
) -> DigestBuckets {
    let mut by_size: HashMap<u64, Vec<FileRecord>> = HashMap::new();
    for record in records {
        match by_size.get_mut(&record.snapshot.len) {
            Some(bucket) => bucket.push(record),
            None => {
                by_size.insert(record.snapshot.len, vec![record]);
            }
        }
    }
    let mut buckets: DigestBuckets = HashMap::new();
    for (len, candidates) in by_size {
        if candidates.len() < 2 {
            continue;
        }
        for candidate in candidates {
            let digest = match hash(&candidate) {
                Ok(digest) => digest,
                Err(issue) => {
                    issues.push(issue);
                    continue;
                }
            };
            let key = (len, digest);
            match buckets.get_mut(&key) {
                Some(bucket) => bucket.push(candidate),
                None => {
                    buckets.insert(key, vec![candidate]);
                }
            }
        }
    }
    buckets
}

pub fn refine_bucket<E: FnMut(&FileRecord, &FileRecord) -> Result<bool, FileIssue>>(
    candidates: &[FileRecord],
    mut compare: E,
) -> Result<Vec<ContentGroup<'_>>, FileIssue> {
    let mut groups: Vec<ContentGroup<'_>> = Vec::new();
    let mut seen: HashMap<PathBuf, ()> = HashMap::new();
    for candidate in candidates {
        match seen.get(candidate.path.as_path()) {
            Some(_) => continue,
            None => {
                seen.insert(candidate.path.as_path().to_path_buf(), ());
            }
        }
        let mut matched = None;
        let mut index = 0;
        while index < groups.len() {
            let representative = groups.as_slice()[index].members.as_slice()[0];
            if compare(representative, candidate)? {
                matched = Some(index);
                break;
            }
            index += 1;
        }
        match matched {
            Some(index) => groups.as_mut_slice()[index].members.push(candidate),
            None => {
                hash_file(candidate)?;
                groups.push(ContentGroup {
                    members: vec![candidate],
                });
            }
        }
    }
    for group in &mut groups {
        group
            .members
            .as_mut_slice()
            .sort_by(|left: &&FileRecord, right: &&FileRecord| {
                left.path.as_path().cmp(right.path.as_path())
            });
    }
    groups
        .as_mut_slice()
        .sort_by(|left: &ContentGroup<'_>, right: &ContentGroup<'_>| {
            left.members.as_slice()[0]
                .path
                .as_path()
                .cmp(right.members.as_slice()[0].path.as_path())
        });
    Ok(groups)
}

pub fn confirm_bucket<E: FnMut(&FileRecord, &FileRecord) -> Result<bool, FileIssue>>(
    mut candidates: Vec<FileRecord>,
    issues: &mut Vec<FileIssue>,
    mut compare: E,
) -> Vec<DuplicateGroup> {
    loop {
        match refine_bucket(candidates.as_slice(), |left, right| compare(left, right)) {
            Ok(groups) => {
                let mut duplicates = Vec::new();
                for group in groups {
                    if group.members.len() < 2 {
                        continue;
                    }
                    let len = group.members.as_slice()[0].snapshot.len;
                    let mut paths = Vec::new();
                    for member in group.members {
                        paths.push(member.path.as_path().to_path_buf());
                    }
                    duplicates.push(DuplicateGroup { len, paths });
                }
                return duplicates;
            }
            Err(failure) => {
                let before = candidates.len();
                let remaining: Vec<FileRecord> = candidates
                    .into_iter()
                    .filter(|record: &FileRecord| record.path != failure.path)
                    .collect();
                if remaining.len() == before {
                    panic!("compare 必須保留本次候選裡的確切失敗路徑");
                }
                candidates = remaining;
                issues.push(failure);
            }
        }
    }
}

pub fn analyze(report: ScanReport) -> ContentReport {
    let mut file_issues = Vec::new();
    let buckets = digest_candidates(report.files, hash_file, &mut file_issues);
    let mut groups = Vec::new();
    for ((_len, _digest), candidates) in buckets {
        if candidates.len() < 2 {
            continue;
        }
        for group in confirm_bucket(candidates, &mut file_issues, equal_files) {
            groups.push(group);
        }
    }
    groups
        .as_mut_slice()
        .sort_by(|left: &DuplicateGroup, right: &DuplicateGroup| {
            left.paths.as_slice()[0]
                .as_path()
                .cmp(right.paths.as_slice()[0].as_path())
        });
    ContentReport {
        groups,
        scan_issues: report.issues,
        file_issues,
    }
}
