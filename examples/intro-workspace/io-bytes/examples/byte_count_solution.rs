use std::fs::File;
use std::io;
use std::io::Read;

fn count_bytes<R: Read>(reader: &mut R) -> io::Result<usize> {
    let mut total = 0;
    let mut buffer: [u8; 4] = [0; 4];
    loop {
        match reader.read(&mut buffer[..]) {
            Ok(count) => {
                if count == 0 {
                    return Ok(total);
                }
                total += count;
            }
            Err(error) => {
                if error.kind() == io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(error);
            }
        }
    }
}

fn main() {
    for (filename, expected) in [
        ("../io-text/fixtures/note.txt", 9),
        ("../io-text/fixtures/empty.txt", 0),
        ("../io-text/fixtures/not-utf8.bin", 1),
    ] {
        let mut file = File::open(filename).expect("教材的固定檔案存在");
        let count = count_bytes(&mut file).expect("固定檔案可讀");
        assert_eq!(count, expected);
        println!("{filename}：{count} bytes");
    }
}
