// --8<-- [start:outcomes]
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
    Unstarted,
    OtherTaskFailure(String),
}

#[derive(Debug, PartialEq)]
pub struct Report {
    pub input: usize,
    pub outcome: Outcome,
}
// --8<-- [end:outcomes]

// --8<-- [start:stop-model]
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

pub struct Batch {
    pub reports: Vec<Report>,
    pub reason: EndReason,
    pub submitted: usize,
    pub peak_tracked: usize,
}
// --8<-- [end:stop-model]
