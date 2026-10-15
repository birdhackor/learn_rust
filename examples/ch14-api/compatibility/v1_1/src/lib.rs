pub struct Options {
    ignore_case: bool,
    limit: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            ignore_case: false,
            limit: 100,
        }
    }
}

impl Options {
    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }

    pub fn limit(&self) -> usize {
        self.limit
    }
}
