pub mod input;
pub mod single;

use crate::input::Input;
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::Id;
use tokio::task::JoinError;
use tokio::task::JoinHandle;
use tokio::task::JoinSet;

// --8<-- [start:models]
#[derive(Debug)]
pub enum Completion {
    Checked(single::Check),
    TaskFailure { original: String, error: JoinError },
    Cancelled { original: String },
    Unstarted { original: String },
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
            Completion::Cancelled { original } => original.as_str(),
            Completion::Unstarted { original } => original.as_str(),
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

#[derive(Debug, PartialEq)]
pub enum StopEvent {
    Requested,
    SignalFailed(String),
}

#[derive(Debug, PartialEq)]
pub enum EndReason {
    Finished,
    Requested,
    SignalFailed(String),
    StopSourceClosed,
}

#[derive(Debug)]
pub struct Batch {
    pub entries: Vec<Entry>,
    pub reason: EndReason,
}
// --8<-- [end:models]

// --8<-- [start:state]
#[allow(dead_code)]
pub struct Scheduler {
    pending: std::vec::IntoIter<Input>,
    tasks: JoinSet<single::Check>,
    inputs: HashMap<Id, Input>,
    entries: Vec<Entry>,
    client: Client,
    jobs: usize,
    limit: Duration,
}
// --8<-- [end:state]

#[allow(dead_code)]
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
        Ok(Self {
            pending: inputs.into_iter(),
            tasks: JoinSet::new(),
            inputs: HashMap::new(),
            entries: Vec::new(),
            client: client.clone(),
            jobs,
            limit,
        })
    }
    // --8<-- [end:constructor]

    // --8<-- [start:fill]
    fn fill(&mut self) {
        while self.tasks.len() < self.jobs {
            let input = match self.pending.next() {
                Some(input) => input,
                None => break,
            };
            let client = self.client.clone();
            let original = input.original.as_str().to_owned();
            let limit = self.limit;
            let abort = self
                .tasks
                .spawn(async move { single::check(&client, original, limit).await });
            self.inputs.insert(abort.id(), input);
        }
    }
    // --8<-- [end:fill]

    // --8<-- [start:classify]
    fn record(&mut self, _result: Result<(Id, single::Check), JoinError>) {
        // 尚未移除登記或保存分類，請替換body。
    }
    // --8<-- [end:classify]

    // --8<-- [start:stop]
    pub async fn stop_and_finish(&mut self) {
        // 尚未取消／回收tasks，也沒有保存pending，請替換body。
    }
    // --8<-- [end:stop]

    // --8<-- [start:run]
    pub async fn run(&mut self, _stop: &mut mpsc::Receiver<StopEvent>) -> EndReason {
        // 尚未監聽停止、提交或收取；固定Finished不是完成。
        EndReason::Finished
    }
    // --8<-- [end:run]

    // --8<-- [start:batch]
    pub fn into_batch(mut self, reason: EndReason) -> Batch {
        assert_eq!(self.tasks.len(), 0);
        assert_eq!(self.inputs.len(), 0);
        self.entries.sort_by_key(|entry| entry.input_index);
        Batch {
            entries: self.entries,
            reason,
        }
    }
    // --8<-- [end:batch]
}

// --8<-- [start:runner]
#[allow(dead_code)]
fn stop_reason(event: Option<StopEvent>) -> EndReason {
    match event {
        Some(StopEvent::Requested) => EndReason::Requested,
        Some(StopEvent::SignalFailed(message)) => EndReason::SignalFailed(message),
        None => EndReason::StopSourceClosed,
    }
}

pub async fn run_batch(
    client: &Client,
    inputs: Vec<Input>,
    jobs: usize,
    limit: Duration,
    mut stop: mpsc::Receiver<StopEvent>,
) -> Result<Batch, ConfigError> {
    let mut scheduler = Scheduler::new(client, inputs, jobs, limit)?;
    let reason = scheduler.run(&mut stop).await;
    Ok(scheduler.into_batch(reason))
}
// --8<-- [end:runner]

// --8<-- [start:signal]
pub async fn signal_controller(stop: mpsc::Sender<StopEvent>, entered: mpsc::Sender<()>) -> bool {
    let event = tokio::select! {
        biased;
        result = tokio::signal::ctrl_c() => {
            match result {
                Ok(()) => StopEvent::Requested,
                Err(error) => StopEvent::SignalFailed(error.to_string()),
            }
        }
        _ = async {
            entered.send(()).await.expect("caller保持controller入口收端");
            std::future::pending::<()>().await;
        } => panic!("ready支路不會完成"),
    };
    match stop.send(event).await {
        Ok(()) => true,
        Err(_) => false,
    }
}

pub async fn finish_signal_controller(controller: JoinHandle<bool>) -> Result<(), String> {
    controller.abort();
    match controller.await {
        Ok(_) => Ok(()),
        Err(error) => {
            if error.is_cancelled() {
                Ok(())
            } else {
                Err(format!("signal controller task失敗：{error}"))
            }
        }
    }
}
// --8<-- [end:signal]
