use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs;
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

// --8<-- [start:observe]
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
// --8<-- [end:observe]

// --8<-- [start:root]
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
// --8<-- [end:root]

// --8<-- [start:scan]
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
// --8<-- [end:scan]

// A1 驗收：將整個 mod tests 加到 src/collection.rs 末尾。
#[cfg(test)]
mod tests {
    use crate::collection::FileRecord;
    use crate::collection::RootError;
    use crate::collection::ScanCause;
    use crate::collection::collect_entries;
    use crate::collection::observe;
    use crate::collection::scan;
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use walkdir::WalkDir;

    #[test]
    fn controlled_walker_failure_is_retained_between_successful_entries() {
        let directory = tempdir().expect("可建立測試目錄");
        let first = directory.path().join("a.txt");
        let last = directory.path().join("z.txt");
        let missing = directory.path().join("missing");
        fs::write(first.as_path(), "A").expect("可建立第一份檔案");
        fs::write(last.as_path(), "Z").expect("可建立最後一份檔案");
        let first_item = match WalkDir::new(first.as_path()).into_iter().next() {
            Some(item) => item,
            None => panic!("本測試的單一路徑應產生一筆 item"),
        };
        let failure_item = match WalkDir::new(missing.as_path()).into_iter().next() {
            Some(item) => item,
            None => panic!("本測試的單一路徑應產生一筆 item"),
        };
        let last_item = match WalkDir::new(last.as_path()).into_iter().next() {
            Some(item) => item,
            None => panic!("本測試的單一路徑應產生一筆 item"),
        };
        let report = collect_entries(vec![first_item, failure_item, last_item]);
        assert_eq!(report.files.len(), 2);
        assert_eq!(report.issues.len(), 1);
        let issue = &report.issues.as_slice()[0];
        assert_eq!(issue.path, Some(missing));
        assert_eq!(issue.action, "走訪目錄");
        match &issue.cause {
            ScanCause::Walk(_) => (),
            _ => panic!("應保留 walkdir 原始失敗"),
        }
    }

    // --8<-- [start:material-test]
    #[test]
    fn six_material_files_include_hidden_and_empty_entries() {
        let directory = tempdir().expect("可建立測試目錄");
        let hidden = directory.path().join(".saved");
        let nested = directory.path().join("sub");
        fs::create_dir(hidden.as_path()).expect("父目錄已存在");
        fs::create_dir(nested.as_path()).expect("父目錄已存在");
        let names = [
            ("notes-a.txt", "Rust\n"),
            ("sub/notes-b.txt", "Rust\n"),
            (".saved/notes-c.txt", "Rust\n"),
            ("different.txt", "rust\n"),
            ("empty-a.bin", ""),
            ("empty-b.bin", ""),
        ];
        let mut expected = Vec::new();
        for (name, body) in names {
            let path = directory.path().join(name);
            fs::write(path.as_path(), body).expect("可建立本頁材料");
            expected.push(path);
        }
        expected.as_mut_slice().sort();
        let report = scan(directory.path()).expect("有效根目錄");
        assert_eq!(report.files.len(), 6);
        assert_eq!(report.issues.len(), 0);
        for file in &report.files {
            let empty_a = directory.path().join("empty-a.bin");
            let empty_b = directory.path().join("empty-b.bin");
            if file.path == empty_a || file.path == empty_b {
                assert_eq!(file.snapshot.len, 0);
            } else {
                assert_eq!(file.snapshot.len, 5);
            }
        }
        let paths: Vec<PathBuf> = report
            .files
            .into_iter()
            .map(|file: FileRecord| file.path)
            .collect();
        assert_eq!(paths, expected);
    }
    // --8<-- [end:material-test]

    #[test]
    fn empty_directory_is_a_complete_empty_result() {
        let directory = tempdir().expect("可建立測試目錄");
        let report = scan(directory.path()).expect("有效根目錄");
        assert_eq!(report.files.len(), 0);
        assert_eq!(report.issues.len(), 0);
    }

    #[test]
    fn root_file_and_missing_root_are_distinct_failures() {
        let directory = tempdir().expect("可建立測試目錄");
        let file = directory.path().join("file.txt");
        fs::write(file.as_path(), "A").expect("可建立一般檔案");
        match scan(file.as_path()) {
            Err(RootError::NotDirectory) => (),
            _ => panic!("root 是一般檔案時應拒絕設定"),
        }
        let missing = directory.path().join("missing");
        match scan(missing.as_path()) {
            Err(RootError::Io(error)) => assert_eq!(error.kind(), io::ErrorKind::NotFound),
            _ => panic!("root 不存在時應保留 I/O 原因"),
        }
    }

    #[test]
    fn observation_rejects_a_directory_as_a_file() {
        let directory = tempdir().expect("可建立測試目錄");
        match observe(directory.path()) {
            Err(ScanCause::NotRegular) => (),
            _ => panic!("目錄不能取得一般檔案記錄"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn root_link_is_rejected_before_walking() {
        use std::os::unix::fs::symlink;

        let directory = tempdir().expect("可建立測試目錄");
        let target = directory.path().join("target");
        fs::create_dir(target.as_path()).expect("父目錄存在");
        let link = directory.path().join("root-link");
        symlink(target.as_path(), link.as_path()).expect("Unix 測試可建立 link");
        match scan(link.as_path()) {
            Err(RootError::SymbolicLink) => (),
            _ => panic!("即使 link 指向有效目錄，root link 也要拒絕"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn inner_file_and_directory_links_are_not_followed() {
        use std::os::unix::fs::symlink;

        let directory = tempdir().expect("可建立測試目錄");
        let root = directory.path().join("root");
        let outside = directory.path().join("outside");
        fs::create_dir(root.as_path()).expect("父目錄存在");
        fs::create_dir(outside.as_path()).expect("父目錄存在");
        let file = outside.as_path().join("outside.txt");
        fs::write(file.as_path(), "A").expect("可建立檔案");
        let file_link = root.as_path().join("file-link");
        let directory_link = root.as_path().join("directory-link");
        symlink(file.as_path(), file_link.as_path()).expect("可建立檔案 link");
        symlink(outside.as_path(), directory_link.as_path()).expect("可建立目錄 link");
        let report = scan(root.as_path()).expect("root 是一般目錄");
        assert_eq!(report.files.len(), 0);
        assert_eq!(report.issues.len(), 0);
    }

    #[cfg(unix)]
    #[test]
    fn non_unicode_path_is_reported_with_its_original_native_identity() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let directory = tempdir().expect("可建立測試目錄");
        let native = OsString::from_vec(vec![255]);
        let invalid = directory.path().join(native);
        fs::write(invalid.as_path(), "A").expect("Unix 可保存這份原生檔名");
        let good = directory.path().join("good.txt");
        fs::write(good.as_path(), "B").expect("可建立 Unicode 檔名");
        let report = scan(directory.path()).expect("root 是有效目錄");
        assert_eq!(report.files.len(), 1);
        assert_eq!(report.files.as_slice()[0].path, good);
        assert_eq!(report.issues.len(), 1);
        let issue = &report.issues.as_slice()[0];
        assert_eq!(issue.path, Some(invalid));
        assert_eq!(issue.action, "準備報告路徑");
        match &issue.cause {
            ScanCause::NonUnicode => (),
            _ => panic!("應保留非 Unicode 原因"),
        }
    }
}
