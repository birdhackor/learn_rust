use intro_io_bytes::read_all;
use std::fs::File;

fn main() {
    match File::open("../io-text/fixtures/note.txt") {
        Ok(mut file) => match read_all(&mut file) {
            Ok(bytes) => {
                println!("取得={} bytes", bytes.len());
                match String::from_utf8(bytes) {
                    Ok(text) => println!("完整文字={:?}", text.as_str()),
                    Err(error) => println!("已讀到 bytes，但不是 UTF-8：{error}"),
                }
            }
            Err(error) => println!("開啟後，讀取失敗：{error}"),
        },
        Err(error) => println!("開啟失敗：{error}"),
    }
}
