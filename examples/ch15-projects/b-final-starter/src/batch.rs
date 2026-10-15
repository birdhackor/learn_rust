use crate::single;

use crate::input::Input;
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::task::Id;
use tokio::task::JoinError;
use tokio::task::JoinHandle;
use tokio::task::JoinSet;

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

pub struct Scheduler {
    pending: std::vec::IntoIter<Input>,
    tasks: JoinSet<single::Check>,
    inputs: HashMap<Id, Input>,
    entries: Vec<Entry>,
    client: Client,
    jobs: usize,
    limit: Duration,
}

impl Scheduler {
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

    fn record(&mut self, result: Result<(Id, single::Check), JoinError>) {
        let id = match &result {
            Ok((id, _)) => *id,
            Err(error) => error.id(),
        };
        let input = self.inputs.remove(&id).expect("每份task已登記完整Input");
        let input_index = input.input_index;
        let completion = match result {
            Ok((_, check)) => Completion::Checked(check),
            Err(error) => {
                if error.is_cancelled() {
                    Completion::Cancelled {
                        original: input.original,
                    }
                } else {
                    Completion::TaskFailure {
                        original: input.original,
                        error,
                    }
                }
            }
        };
        self.entries.push(Entry {
            input_index,
            completion,
        });
    }

    pub async fn stop_and_finish(&mut self) {
        self.tasks.abort_all();
        while let Some(result) = self.tasks.join_next_with_id().await {
            self.record(result);
        }
        for input in self.pending.by_ref() {
            self.entries.push(Entry {
                input_index: input.input_index,
                completion: Completion::Unstarted {
                    original: input.original,
                },
            });
        }
    }

    pub async fn run(&mut self, stop: &mut mpsc::Receiver<StopEvent>) -> EndReason {
        loop {
            match stop.try_recv() {
                Ok(event) => {
                    self.stop_and_finish().await;
                    return stop_reason(Some(event));
                }
                Err(TryRecvError::Disconnected) => {
                    self.stop_and_finish().await;
                    return EndReason::StopSourceClosed;
                }
                Err(TryRecvError::Empty) => (),
            }
            self.fill();
            if self.tasks.is_empty() {
                return EndReason::Finished;
            }
            tokio::select! {
                biased;
                event = stop.recv() => {
                    self.stop_and_finish().await;
                    return stop_reason(event);
                }
                result = self.tasks.join_next_with_id() => {
                    self.record(result.expect("此時集合至少有一份task"));
                }
            }
        }
    }

    pub fn into_batch(mut self, reason: EndReason) -> Batch {
        assert_eq!(self.tasks.len(), 0);
        assert_eq!(self.inputs.len(), 0);
        self.entries.sort_by_key(|entry| entry.input_index);
        Batch {
            entries: self.entries,
            reason,
        }
    }
}

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
