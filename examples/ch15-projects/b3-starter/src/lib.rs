pub mod input;
pub mod single;

use crate::input::Input;
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
use tokio::task::Id;
use tokio::task::JoinError;
use tokio::task::JoinSet;

// --8<-- [start:models]
#[derive(Debug)]
pub enum Completion {
    Checked(single::Check),
    TaskFailure { original: String, error: JoinError },
}

#[derive(Debug)]
pub struct Entry {
    pub input_index: usize,
    pub completion: Completion,
}

impl Entry {
    pub fn original(&self) -> &str {
        match &self.completion {
            Completion::Checked(check) => check.original.as_str(),
            Completion::TaskFailure { original, error: _ } => original.as_str(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    ZeroJobs,
    Empty,
}

pub fn default_jobs() -> usize {
    8
}
// --8<-- [end:models]

// --8<-- [start:state]
// 起始版尚未使用全部欄位；本文先解釋這份暫時的lint設定。
#[allow(dead_code)]
pub struct Scheduler {
    pending: std::vec::IntoIter<Input>,
    tasks: JoinSet<single::Check>,
    inputs: HashMap<Id, Input>,
    client: Client,
    jobs: usize,
    limit: Duration,
}
// --8<-- [end:state]

impl Scheduler {
    // --8<-- [start:constructor]
    pub fn new(
        client: &Client,
        inputs: Vec<Input>,
        jobs: usize,
        limit: Duration,
    ) -> Result<Self, ConfigError> {
        if jobs == 0 {
            return Err(ConfigError::ZeroJobs);
        }
        if inputs.is_empty() {
            return Err(ConfigError::Empty);
        }
        let mut scheduler = Self {
            pending: inputs.into_iter(),
            tasks: JoinSet::new(),
            inputs: HashMap::new(),
            client: client.clone(),
            jobs,
            limit,
        };
        scheduler.fill();
        Ok(scheduler)
    }
    // --8<-- [end:constructor]

    // --8<-- [start:fill]
    fn fill(&mut self) {
        // 尚未提交任何task，請替換body。
    }
    // --8<-- [end:fill]

    // --8<-- [start:next]
    pub async fn next(&mut self) -> Option<Entry> {
        // 尚未回收，也沒有補工；固定None不表示任務真的完成。
        None
    }
    // --8<-- [end:next]

    pub fn tracked(&self) -> usize {
        self.tasks.len()
    }
}

// --8<-- [start:ordered]
pub fn order_entries(entries: &mut [Entry]) {
    entries.sort_by_key(|entry| entry.input_index);
}

pub async fn run_bounded(
    _client: &Client,
    _inputs: Vec<Input>,
    _jobs: usize,
    _limit: Duration,
) -> Result<Vec<Entry>, ConfigError> {
    // 尚未連接constructor、回收迴圈與最後排序。
    Ok(Vec::new())
}
// --8<-- [end:ordered]
