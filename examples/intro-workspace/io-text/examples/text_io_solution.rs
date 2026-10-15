use std::fs;
use std::io;

fn total_file_bytes(first: &str, second: &str) -> io::Result<usize> {
    let first_text = fs::read_to_string(first)?;
    let second_text = fs::read_to_string(second)?;
    Ok(first_text.as_str().len() + second_text.as_str().len())
}

fn main() {
    let total = total_file_bytes("fixtures/note.txt", "fixtures/empty.txt")
        .expect("兩個 repository fixture 都是已知的 UTF-8 文字");
    assert_eq!(total, 9);
    println!("兩份文字總長={total} bytes");

    match total_file_bytes("fixtures/note.txt", "fixtures/not-utf8.bin") {
        Ok(_) => panic!("第二個 fixture 應因 UTF-8 失敗"),
        Err(error) => assert_eq!(error.kind(), io::ErrorKind::InvalidData),
    }
    println!("第二份文字失敗時，沒有交回成功長度");
}
