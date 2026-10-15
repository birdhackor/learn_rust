// --8<-- [start:read-example]
use std::fs;
use std::io;

fn show_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(text) => {
            println!("{filename}：{} bytes", text.as_str().len());
            println!("內容={:?}", text.as_str());
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => println!("{filename}：找不到檔案"),
            io::ErrorKind::InvalidData => println!("{filename}：內容不是合法 UTF-8"),
            _ => println!("{filename}：讀取失敗，{error}"),
        },
    }
}

fn main() {
    for filename in [
        "fixtures/note.txt",
        "fixtures/missing.txt",
        "fixtures/not-utf8.bin",
    ] {
        show_file(filename);
    }
    println!("三個讀取結果都已處理");
}
// --8<-- [end:read-example]

// --8<-- [start:read-tests]
#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;

    #[test]
    fn text_keeps_newlines_and_unicode() {
        let text = fs::read_to_string("fixtures/note.txt").expect("教材範例檔是已知的 UTF-8 文字");
        assert_eq!(text.as_str(), "Rust\n中\n");
        assert_eq!(text.as_str().len(), 9);
    }

    #[test]
    fn absent_file_and_invalid_text_keep_distinct_kinds() {
        for (filename, expected) in [
            ("fixtures/missing.txt", io::ErrorKind::NotFound),
            ("fixtures/not-utf8.bin", io::ErrorKind::InvalidData),
        ] {
            match fs::read_to_string(filename) {
                Ok(_) => panic!("這份 fixture 應讀取失敗：{filename}"),
                Err(error) => assert_eq!(error.kind(), expected),
            }
        }
    }
}
// --8<-- [end:read-tests]
