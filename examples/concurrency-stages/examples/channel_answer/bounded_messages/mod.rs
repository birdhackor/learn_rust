use std::sync::mpsc::SendError;
use std::sync::mpsc::SyncSender;

pub fn send_bounded_values(
    sender: SyncSender<i32>,
    values: Vec<i32>,
) -> Result<usize, SendError<i32>> {
    let mut sent = 0;
    for value in values {
        sender.send(value)?;
        sent += 1;
    }
    Ok(sent)
}
