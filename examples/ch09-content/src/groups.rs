use crate::files::FileIssue;
use crate::files::equal_files;
use ch09_files::scan::FileRecord;
use std::collections::HashMap;
use std::num::TryFromIntError;
use std::path::PathBuf;

// --8<-- [start:refine]
pub struct ContentGroup<'a> {
    pub members: Vec<&'a FileRecord>,
}

pub fn refine_bucket(candidates: &[FileRecord]) -> Result<Vec<ContentGroup<'_>>, FileIssue> {
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
            if equal_files(representative, candidate)? {
                matched = Some(index);
                break;
            }
            index += 1;
        }
        match matched {
            Some(index) => groups.as_mut_slice()[index].members.push(candidate),
            None => {
                hash_file_for_singleton(candidate)?;
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

fn hash_file_for_singleton(record: &FileRecord) -> Result<(), FileIssue> {
    crate::files::hash_file(record)?;
    Ok(())
}
// --8<-- [end:refine]

// --8<-- [start:metrics]
#[derive(Debug)]
pub enum MetricError {
    Count(TryFromIntError),
    Overflow,
}

pub fn duplicate_bytes(len: u64, path_count: usize) -> Result<u64, MetricError> {
    if path_count < 2 {
        return Ok(0);
    }
    let extra_paths = match u64::try_from(path_count - 1) {
        Ok(count) => count,
        Err(error) => return Err(MetricError::Count(error)),
    };
    match len.checked_mul(extra_paths) {
        Some(bytes) => Ok(bytes),
        None => Err(MetricError::Overflow),
    }
}

pub fn add_duplicate_bytes(total: u64, bytes: u64) -> Result<u64, MetricError> {
    match total.checked_add(bytes) {
        Some(sum) => Ok(sum),
        None => Err(MetricError::Overflow),
    }
}
// --8<-- [end:metrics]
