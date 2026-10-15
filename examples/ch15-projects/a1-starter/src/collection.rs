use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use walkdir::WalkDir;

// --8<-- [start:records]
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
// --8<-- [end:records]

// --8<-- [start:work]
// A1 起始版：這三個函式尚未實作，不是可用的掃描器。
pub fn observe(_path: &Path) -> Result<Snapshot, ScanCause> {
    // TODO：重新觀察一般檔案，保留長度、修改時間或原因。
    Err(ScanCause::NotRegular)
}

pub fn validate_root(_root: &Path) -> Result<(), RootError> {
    // TODO：拒絕不存在、symbolic link 與非目錄的根路徑。
    // 固定 Err 只標示尚未實作，並未真的觀察 root。
    Err(RootError::NotDirectory)
}

pub fn collect_entries<I: IntoIterator<Item = Result<walkdir::DirEntry, walkdir::Error>>>(
    _source: I,
) -> ScanReport {
    // TODO：走完整個來源，分開保存 files 與 issues，再排序 files。
    ScanReport {
        files: Vec::new(),
        issues: Vec::new(),
    }
}
// --8<-- [end:work]

// --8<-- [start:wire]
pub fn scan(root: &Path) -> Result<ScanReport, RootError> {
    validate_root(root)?;
    let walker = WalkDir::new(root)
        .follow_links(false)
        .follow_root_links(false);
    Ok(collect_entries(walker))
}
// --8<-- [end:wire]
