use std::collections::HashMap;
use tokio::task::Id;
use tokio::task::JoinSet;

#[derive(Debug, PartialEq)]
pub enum WorkError {
    Negative(i32),
    ControlClosed,
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Value(i32),
    WorkError(WorkError),
    Panic,
    Cancelled,
    OtherTaskFailure(String),
}

#[derive(Debug, PartialEq)]
pub struct Report {
    pub input: usize,
    pub outcome: Outcome,
}

// --8<-- [start:collect-one]
pub async fn collect_one(
    tasks: &mut JoinSet<Result<i32, WorkError>>,
    inputs: &mut HashMap<Id, usize>,
) -> Option<Report> {
    let result = match tasks.join_next_with_id().await {
        Some(result) => result,
        None => return None,
    };
    let (id, outcome) = match result {
        Ok((id, Ok(value))) => (id, Outcome::Value(value)),
        Ok((id, Err(error))) => (id, Outcome::WorkError(error)),
        Err(error) => {
            let id = error.id();
            let outcome = if error.is_cancelled() {
                Outcome::Cancelled
            } else if error.is_panic() {
                Outcome::Panic
            } else {
                Outcome::OtherTaskFailure(error.to_string())
            };
            (id, outcome)
        }
    };
    let input = inputs
        .remove(&id)
        .expect("每份 task 在 spawn 後已記錄輸入位置");
    Some(Report { input, outcome })
}
// --8<-- [end:collect-one]
