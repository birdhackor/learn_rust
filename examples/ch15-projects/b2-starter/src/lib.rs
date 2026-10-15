pub mod single;

use reqwest::Client;
use std::time::Duration;

// --8<-- [start:models]
#[derive(Debug)]
pub struct Input {
    pub input_index: usize,
    pub original: String,
}

#[derive(Debug)]
pub struct Entry {
    pub input_index: usize,
    pub check: single::Check,
}

#[derive(Debug)]
pub enum InputError {
    Empty,
}
// --8<-- [end:models]

// --8<-- [start:stubs]
pub fn prepare_inputs(_text: &str) -> Result<Vec<Input>, InputError> {
    // 尚未處理任何行，空Ok也不符合本題的Empty政策。
    Ok(Vec::new())
}

pub async fn run_sequential(
    _client: &Client,
    _inputs: Vec<Input>,
    _limit: Duration,
) -> Result<Vec<Entry>, InputError> {
    // 尚未送出任何請求，也未保存任何單筆結果。
    Ok(Vec::new())
}
// --8<-- [end:stubs]
