// --8<-- [start:handle-example]
use std::fs::File;
use std::io;
use std::io::Read;

fn read_with_prefix(filename: &str) -> io::Result<(String, usize)> {
    let mut file = File::open(filename)?;
    let mut text = String::new();
    text.push_str("筆記：\n");
    let added_bytes = file.read_to_string(&mut text)?;
    Ok((text, added_bytes))
}

fn main() {
    match read_with_prefix("fixtures/note.txt") {
        Ok((text, added_bytes)) => {
            assert_eq!(added_bytes, 9);
            assert_eq!(text.as_str(), "筆記：\nRust\n中\n");
            println!("新讀入={added_bytes} bytes");
            println!("包含前綴={} bytes", text.as_str().len());
            println!("完整內容={:?}", text.as_str());
        }
        Err(error) => println!("讀取失敗：{error}"),
    }
}
// --8<-- [end:handle-example]

// --8<-- [start:handle-tests]
#[cfg(test)]
mod tests {
    use crate::read_with_prefix;

    #[test]
    fn count_is_new_bytes_not_existing_prefix() {
        let (text, added_bytes) =
            read_with_prefix("fixtures/note.txt").expect("repository fixture 可讀");
        assert_eq!(added_bytes, 9);
        assert_eq!(text.as_str().len(), 19);
        assert_eq!(text.as_str(), "筆記：\nRust\n中\n");
    }
}
// --8<-- [end:handle-tests]
