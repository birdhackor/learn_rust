use std::io;
use std::io::Read;

pub fn read_all<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut buffer: [u8; 4] = [0; 4];
    loop {
        let count = match reader.read(&mut buffer[..]) {
            Ok(count) => count,
            Err(error) => {
                if error.kind() == io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(error);
            }
        };
        if count == 0 {
            return Ok(bytes);
        }
        for byte in &buffer[..count] {
            bytes.push(*byte);
        }
    }
}
