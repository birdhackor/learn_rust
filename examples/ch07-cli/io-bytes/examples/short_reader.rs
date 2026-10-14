// --8<-- [start:short-reader]
use std::io;
use std::io::Read;

struct ShortReader {
    data: Vec<u8>,
    position: usize,
    max_per_read: usize,
}

impl Read for ShortReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut copied = 0;
        while copied < buffer.len() && copied < self.max_per_read && self.position < self.data.len()
        {
            buffer[copied] = self.data.as_slice()[self.position];
            copied += 1;
            self.position += 1;
        }
        Ok(copied)
    }
}
// --8<-- [end:short-reader]

// --8<-- [start:short-reader-main]
use ch07_io_bytes::read_all;
use std::borrow::ToOwned;

fn main() {
    let mut reader = ShortReader {
        data: "ABCDE".as_bytes().to_owned(),
        position: 0,
        max_per_read: 3,
    };
    let bytes = read_all(&mut reader).expect("這個固定來源只有合法短讀取");
    assert_eq!(bytes.len(), 5);
    let text = String::from_utf8(bytes).expect("固定材料是 ASCII 文字");
    assert_eq!(text.as_str(), "ABCDE");
    println!("短讀取仍取得完整文字={}", text.as_str());
}
// --8<-- [end:short-reader-main]

// --8<-- [start:short-reader-tests]
#[cfg(test)]
mod tests {
    use crate::ShortReader;
    use ch07_io_bytes::read_all;
    use std::borrow::ToOwned;
    use std::io::Read;

    #[test]
    fn incomplete_first_read_does_not_end_the_input() {
        let mut reader = ShortReader {
            data: "ABCDE".as_bytes().to_owned(),
            position: 0,
            max_per_read: 3,
        };
        let mut buffer: [u8; 4] = [0; 4];
        let first = reader.read(&mut buffer[..]).expect("固定來源可讀");
        assert_eq!(first, 3);
        let second = reader.read(&mut buffer[..]).expect("還有兩個 bytes");
        assert_eq!(second, 2);
        assert_eq!(buffer[0], 68);
        assert_eq!(buffer[1], 69);
        assert_eq!(buffer[2], 67);
        assert_eq!(reader.read(&mut buffer[..]).expect("來源已結束"), 0);
    }

    #[test]
    fn utf8_is_validated_after_collecting_all_chunks() {
        for max_per_read in [1, 2, 3, 8] {
            let mut reader = ShortReader {
                data: "Rust\n中\n".as_bytes().to_owned(),
                position: 0,
                max_per_read,
            };
            let bytes = read_all(&mut reader).expect("固定來源可完整讀取");
            assert_eq!(bytes.len(), 9);
            let text = String::from_utf8(bytes).expect("完整資料是合法 UTF-8");
            assert_eq!(text.as_str(), "Rust\n中\n");
        }
    }

    #[test]
    fn empty_source_returns_an_empty_vector() {
        let mut reader = ShortReader {
            data: Vec::new(),
            position: 0,
            max_per_read: 3,
        };
        assert_eq!(read_all(&mut reader).expect("空資料仍是成功").len(), 0);
    }
}
// --8<-- [end:short-reader-tests]
