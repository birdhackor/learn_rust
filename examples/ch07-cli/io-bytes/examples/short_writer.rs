// --8<-- [start:short-writer]
use std::io;
use std::io::Write;

struct ShortWriter {
    bytes: Vec<u8>,
    accepts_data: bool,
}

impl Write for ShortWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if !self.accepts_data {
            return Ok(0);
        }
        let count = if buffer.len() > 2 { 2 } else { buffer.len() };
        for byte in &buffer[..count] {
            self.bytes.push(*byte);
        }
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
// --8<-- [end:short-writer]

// --8<-- [start:short-writer-main]
fn main() {
    let input = "ABCDE".as_bytes();
    let mut writer = ShortWriter {
        bytes: Vec::new(),
        accepts_data: true,
    };
    let written = writer.write(input).expect("固定出口可接受前兩個 bytes");
    assert_eq!(written, 2);
    println!("一次 write 寫入={written} bytes，仍有 3 bytes");
    writer
        .write_all(&input[written..])
        .expect("固定出口可繼續接受剩餘資料");
    writer.flush().expect("此記憶體出口沒有中間暫存待推送");
    let text = String::from_utf8(writer.bytes).expect("固定材料是 ASCII");
    assert_eq!(text.as_str(), "ABCDE");
    println!("完成後={}", text.as_str());
}
// --8<-- [end:short-writer-main]

// --8<-- [start:short-writer-tests]
#[cfg(test)]
mod tests {
    use crate::ShortWriter;
    use std::io;
    use std::io::Write;

    #[test]
    fn write_all_repeats_short_writes_without_skipping_or_duplicating_bytes() {
        let mut writer = ShortWriter {
            bytes: Vec::new(),
            accepts_data: true,
        };
        writer
            .write_all("ABCDE".as_bytes())
            .expect("固定出口可寫入");
        let text = String::from_utf8(writer.bytes).expect("固定材料是 ASCII");
        assert_eq!(text.as_str(), "ABCDE");
    }

    #[test]
    fn write_all_reports_zero_progress_for_nonempty_input() {
        let mut writer = ShortWriter {
            bytes: Vec::new(),
            accepts_data: false,
        };
        match writer.write_all("A".as_bytes()) {
            Ok(()) => panic!("非空資料沒有寫入，不能回報完成"),
            Err(error) => assert_eq!(error.kind(), io::ErrorKind::WriteZero),
        }
        assert_eq!(writer.bytes.len(), 0);
    }
}
// --8<-- [end:short-writer-tests]
