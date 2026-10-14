use std::path::Path;
use std::path::PathBuf;

fn main() {
    let directory = Path::new("notes");
    let mut filename: PathBuf = directory.join("2026");
    filename.push("note.txt");
    let view: &Path = filename.as_path();
    assert_eq!(directory, Path::new("notes"));
    assert_eq!(view, Path::new("notes/2026/note.txt"));
    println!("原目錄={}", directory.display());
    println!("組好的檔名={}", view.display());
    match view.to_str() {
        Some(text) => println!("這份路徑可借為 UTF-8={text}"),
        None => println!("這份路徑保留原生形式，沒有 UTF-8 文字表示"),
    }
}
