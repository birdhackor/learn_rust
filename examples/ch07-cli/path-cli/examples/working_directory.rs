use std::env;
use std::fs;

fn main() {
    match env::current_dir() {
        Ok(directory) => {
            let filename = directory.as_path().join("../io-text/fixtures/note.txt");
            println!("工作目錄={}", directory.as_path().display());
            match fs::read_to_string(filename.as_path()) {
                Ok(text) => {
                    assert_eq!(text.as_str(), "Rust\n中\n");
                    println!("以 &Path 讀取={:?}", text.as_str());
                }
                Err(error) => println!("讀取失敗：{error}"),
            }
        }
        Err(error) => println!("無法取得工作目錄：{error}"),
    }
}
