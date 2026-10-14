use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

fn main() {
    let native = OsString::from("中文筆記.txt");
    let view: &OsStr = native.as_os_str();
    println!("原生資料檢查={view:?}");
    match view.to_str() {
        Some(text) => println!("借用 UTF-8={text}"),
        None => println!("原生資料不能借為 UTF-8"),
    }
    let filename = PathBuf::from(native);
    println!("原生資料交成路徑={}", filename.as_path().display());

    let owned = OsString::from("Rust");
    match owned.into_string() {
        Ok(text) => {
            assert_eq!(text.as_str(), "Rust");
            println!("取得 owned String={}", text.as_str());
        }
        Err(original) => println!("轉換失敗，原生資料仍有 owner：{original:?}"),
    }
}
