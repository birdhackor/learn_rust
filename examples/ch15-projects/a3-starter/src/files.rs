use crate::collection::FileRecord;
use crate::collection::ScanCause;
use crate::collection::Snapshot;
use crate::collection::observe;
use crate::stream::CompareReadError;
use crate::stream::equal_readers;
use crate::stream::hash_reader;
use blake3::Hash;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::path::PathBuf;

// --8<-- [start:issues]
#[derive(Debug)]
pub enum FileCause {
    Io(io::Error),
    Observation(ScanCause),
    Changed,
}

impl Display for FileCause {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Observation(error) => write!(formatter, "{error}"),
            Self::Changed => write!(formatter, "觀察到檔案資料變動"),
        }
    }
}

#[derive(Debug)]
pub struct FileIssue {
    pub path: PathBuf,
    pub action: &'static str,
    pub cause: FileCause,
}

impl Display for FileIssue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}；{:?}；{}",
            self.action, self.path, self.cause
        )
    }
}

fn issue(record: &FileRecord, action: &'static str, cause: FileCause) -> FileIssue {
    FileIssue {
        path: record.path.as_path().to_path_buf(),
        action,
        cause,
    }
}
// --8<-- [end:issues]

// --8<-- [start:observe]
fn observe_handle(file: &File) -> Result<Snapshot, ScanCause> {
    let metadata = match file.metadata() {
        Ok(metadata) => metadata,
        Err(error) => return Err(ScanCause::Io(error)),
    };
    if !metadata.is_file() {
        return Err(ScanCause::NotRegular);
    }
    let modified = match metadata.modified() {
        Ok(modified) => modified,
        Err(error) => return Err(ScanCause::Io(error)),
    };
    Ok(Snapshot {
        len: metadata.len(),
        modified,
    })
}

pub fn verify_unchanged(
    record: &FileRecord,
    file: &File,
    action: &'static str,
) -> Result<(), FileIssue> {
    let by_path = match observe(record.path.as_path()) {
        Ok(snapshot) => snapshot,
        Err(error) => return Err(issue(record, action, FileCause::Observation(error))),
    };
    let by_handle = match observe_handle(file) {
        Ok(snapshot) => snapshot,
        Err(error) => return Err(issue(record, action, FileCause::Observation(error))),
    };
    if by_path != record.snapshot || by_handle != record.snapshot {
        return Err(issue(record, action, FileCause::Changed));
    }
    Ok(())
}

fn open_checked(record: &FileRecord) -> Result<File, FileIssue> {
    let before = match observe(record.path.as_path()) {
        Ok(snapshot) => snapshot,
        Err(error) => return Err(issue(record, "開啟前觀察", FileCause::Observation(error))),
    };
    if before != record.snapshot {
        return Err(issue(record, "開啟前觀察", FileCause::Changed));
    }
    let file = match File::open(record.path.as_path()) {
        Ok(file) => file,
        Err(error) => return Err(issue(record, "開啟檔案", FileCause::Io(error))),
    };
    verify_unchanged(record, &file, "讀取前觀察")?;
    Ok(file)
}
// --8<-- [end:observe]

// --8<-- [start:hash]
pub fn hash_file(record: &FileRecord) -> Result<Hash, FileIssue> {
    let mut file = open_checked(record)?;
    let digest = match hash_reader(&mut file) {
        Ok(digest) => digest,
        Err(error) => return Err(issue(record, "hash 讀取", FileCause::Io(error))),
    };
    verify_unchanged(record, &file, "hash 後觀察")?;
    Ok(digest)
}
// --8<-- [end:hash]

// --8<-- [start:equal]
pub fn equal_files(left: &FileRecord, right: &FileRecord) -> Result<bool, FileIssue> {
    let mut left_file = open_checked(left)?;
    let mut right_file = open_checked(right)?;
    let equal = match equal_readers(&mut left_file, &mut right_file) {
        Ok(equal) => equal,
        Err(CompareReadError::Left(error)) => {
            return Err(issue(left, "內容比較讀取", FileCause::Io(error)));
        }
        Err(CompareReadError::Right(error)) => {
            return Err(issue(right, "內容比較讀取", FileCause::Io(error)));
        }
    };
    verify_unchanged(left, &left_file, "內容比較後觀察")?;
    verify_unchanged(right, &right_file, "內容比較後觀察")?;
    Ok(equal)
}
// --8<-- [end:equal]
