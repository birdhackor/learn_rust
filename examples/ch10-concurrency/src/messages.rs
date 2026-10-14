use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;

// --8<-- [start:send-values]
pub fn send_values(sender: Sender<i32>, values: Vec<i32>) -> Result<usize, SendError<i32>> {
    let mut sent = 0;
    for value in values {
        sender.send(value)?;
        sent += 1;
    }
    Ok(sent)
}
// --8<-- [end:send-values]
