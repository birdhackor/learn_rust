// --8<-- [start:error-reader]
use std::io;
use std::io::Read;

struct InterruptedThenFailure {
    step: usize,
}

impl Read for InterruptedThenFailure {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        let step = self.step;
        self.step += 1;
        if step == 0 {
            Err(io::Error::from(io::ErrorKind::Interrupted))
        } else if step == 1 {
            buffer[0] = 82;
            Ok(1)
        } else {
            Err(io::Error::from(io::ErrorKind::PermissionDenied))
        }
    }
}
// --8<-- [end:error-reader]

// --8<-- [start:error-main]
use ch07_io_bytes::read_all;

fn main() {
    let mut reader = InterruptedThenFailure { step: 0 };
    match read_all(&mut reader) {
        Ok(_) => panic!("第三次讀取應失敗"),
        Err(error) => {
            assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
            assert_eq!(reader.step, 3);
            println!("Interrupted 已重試，之後的讀取失敗有保留原因");
        }
    }
}
// --8<-- [end:error-main]

#[cfg(test)]
mod tests {
    use crate::InterruptedThenFailure;
    use ch07_io_bytes::read_all;
    use std::io;

    #[test]
    fn interrupted_is_retried_and_later_error_does_not_become_partial_success() {
        let mut reader = InterruptedThenFailure { step: 0 };
        match read_all(&mut reader) {
            Ok(_) => panic!("不能把已讀一個 byte 當成完整成功"),
            Err(error) => assert_eq!(error.kind(), io::ErrorKind::PermissionDenied),
        }
        assert_eq!(reader.step, 3);
    }
}
