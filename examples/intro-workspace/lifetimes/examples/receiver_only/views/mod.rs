pub struct TextView<'a> {
    pub text: &'a str,
}

impl<'a> TextView<'a> {
    pub fn byte_len(&self) -> usize {
        self.text.len()
    }

    // --8<-- [start:reference-methods]
    pub fn text(&self) -> &str {
        self.text
    }

    pub fn original(&self) -> &'a str {
        self.text
    }
    // --8<-- [end:reference-methods]
}
