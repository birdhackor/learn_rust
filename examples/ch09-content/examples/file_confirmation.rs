use ch09_content::files::equal_files;
use ch09_content::files::hash_file;
use ch09_files::scan::FileRecord;
use ch09_files::scan::observe;
use std::path::Path;

fn record(path: &Path) -> FileRecord {
    FileRecord {
        path: path.to_path_buf(),
        snapshot: observe(path).expect("教材固定材料可取得檔案資料"),
    }
}

fn main() {
    let first = record(Path::new("examples/ch09-files/fixtures/tree/first.txt"));
    let second = record(Path::new(
        "examples/ch09-files/fixtures/tree/nested/second.txt",
    ));
    let different = record(Path::new(
        "examples/ch09-files/fixtures/tree/.hidden/third.txt",
    ));
    let first_digest = hash_file(&first).expect("固定穩定材料可讀");
    let second_digest = hash_file(&second).expect("固定穩定材料可讀");
    assert_eq!(first_digest, second_digest);
    println!("相同 digest={first_digest}");
    let same = equal_files(&first, &second).expect("固定材料可比較");
    assert!(same);
    println!("真正內容確認相同={same}");
    let same = equal_files(&first, &different).expect("固定材料可比較");
    assert!(!same);
    println!("換一份內容後相同={same}");
}
