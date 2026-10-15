#[derive(Default)]
pub struct Options {
    ignore_case: bool,
}

impl Options {
    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}
