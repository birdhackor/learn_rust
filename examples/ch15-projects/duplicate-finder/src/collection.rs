use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Snapshot {
    pub len: u64,
    pub modified: SystemTime,
}

#[derive(Debug)]
pub struct FileRecord {
    pub path: PathBuf,
    pub snapshot: Snapshot,
}

#[derive(Debug)]
pub enum ScanCause {
    Io(io::Error),
    Walk(walkdir::Error),
    NonUnicode,
    NotRegular,
}

impl Display for ScanCause {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Walk(error) => write!(formatter, "{error}"),
            Self::NonUnicode => write!(formatter, "路徑不能表示為 Unicode"),
            Self::NotRegular => write!(formatter, "目前位置不是一般檔案"),
        }
    }
}

#[derive(Debug)]
pub struct ScanIssue {
    pub path: Option<PathBuf>,
    pub action: &'static str,
    pub cause: ScanCause,
}

pub struct ScanReport {
    pub files: Vec<FileRecord>,
    pub issues: Vec<ScanIssue>,
}

#[derive(Debug)]
pub enum RootError {
    Io(io::Error),
    SymbolicLink,
    NotDirectory,
}

pub fn observe(path: &Path) -> Result<Snapshot, ScanCause> {
    let metadata = match fs::symlink_metadata(path) {
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

pub fn validate_root(root: &Path) -> Result<(), RootError> {
    let metadata = match fs::symlink_metadata(root) {
        Ok(metadata) => metadata,
        Err(error) => return Err(RootError::Io(error)),
    };
    if metadata.file_type().is_symlink() {
        return Err(RootError::SymbolicLink);
    }
    if !metadata.is_dir() {
        return Err(RootError::NotDirectory);
    }
    Ok(())
}

pub fn scan(root: &Path) -> Result<ScanReport, RootError> {
    validate_root(root)?;
    let walker = WalkDir::new(root)
        .follow_links(false)
        .follow_root_links(false);
    Ok(collect_entries(walker))
}

pub fn collect_entries<I: IntoIterator<Item = Result<walkdir::DirEntry, walkdir::Error>>>(
    source: I,
) -> ScanReport {
    let mut files = Vec::new();
    let mut issues = Vec::new();
    for item in source {
        let entry = match item {
            Ok(entry) => entry,
            Err(error) => {
                let path = error.path().map(|path: &Path| path.to_path_buf());
                issues.push(ScanIssue {
                    path,
                    action: "走訪目錄",
                    cause: ScanCause::Walk(error),
                });
                continue;
            }
        };
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().to_str().is_none() {
            issues.push(ScanIssue {
                path: Some(entry.into_path()),
                action: "準備報告路徑",
                cause: ScanCause::NonUnicode,
            });
            continue;
        }
        let snapshot = match observe(entry.path()) {
            Ok(snapshot) => snapshot,
            Err(cause) => {
                issues.push(ScanIssue {
                    path: Some(entry.into_path()),
                    action: "取得檔案資料",
                    cause,
                });
                continue;
            }
        };
        files.push(FileRecord {
            path: entry.into_path(),
            snapshot,
        });
    }
    files
        .as_mut_slice()
        .sort_by(|left: &FileRecord, right: &FileRecord| {
            left.path.as_path().cmp(right.path.as_path())
        });
    ScanReport { files, issues }
}
