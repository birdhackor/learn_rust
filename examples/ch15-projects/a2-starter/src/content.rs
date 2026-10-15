use crate::collection::FileRecord;
use crate::collection::ScanIssue;
use crate::collection::ScanReport;
use crate::files::FileIssue;
use crate::files::equal_files;
use crate::files::hash_file;
use blake3::Hash;
use std::collections::HashMap;
use std::path::PathBuf;

// --8<-- [start:models]
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
// --8<-- [end:models]

// --8<-- [start:digest]
pub fn digest_candidates<H: FnMut(&FileRecord) -> Result<Hash, FileIssue>>(
    _records: Vec<FileRecord>,
    _hash: H,
    _issues: &mut Vec<FileIssue>,
) -> DigestBuckets {
    // TODO：先按大小排除單筆，再逐檔 hash，保留失敗並繼續。
    HashMap::new()
}
// --8<-- [end:digest]

// --8<-- [start:refine]
pub fn refine_bucket<E: FnMut(&FileRecord, &FileRecord) -> Result<bool, FileIssue>>(
    candidates: &[FileRecord],
    _compare: E,
) -> Result<Vec<ContentGroup<'_>>, FileIssue> {
    // TODO：確認所有真正內容群，並以原生路徑去重與排序。
    // 固定 Changed 只標示待做，並未真的觀察到變動。
    if candidates.is_empty() {
        return Ok(Vec::new());
    }
    Err(FileIssue {
        path: candidates[0].path.as_path().to_path_buf(),
        action: "A2 尚未實作",
        cause: crate::files::FileCause::Changed,
    })
}
// --8<-- [end:refine]

// --8<-- [start:recover]
pub fn confirm_bucket<E: FnMut(&FileRecord, &FileRecord) -> Result<bool, FileIssue>>(
    _candidates: Vec<FileRecord>,
    _issues: &mut Vec<FileIssue>,
    _compare: E,
) -> Vec<DuplicateGroup> {
    // TODO：保存確切失敗者，移除後重建；成功才交出雙路徑以上群組。
    Vec::new()
}
// --8<-- [end:recover]

// --8<-- [start:analyze]
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
// --8<-- [end:analyze]
