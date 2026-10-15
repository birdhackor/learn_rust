use crate::BatchLimit;

// --8<-- [start:options]
#[derive(Debug, Clone, Default)]
pub struct Options {
    ignore_case: bool,
    limit: BatchLimit,
}

impl Options {
    pub fn with_ignore_case(mut self, enabled: bool) -> Self {
        self.ignore_case = enabled;
        self
    }

    pub fn with_limit(mut self, limit: BatchLimit) -> Self {
        self.limit = limit;
        self
    }

    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }

    pub fn limit(&self) -> BatchLimit {
        self.limit
    }
}
// --8<-- [end:options]
