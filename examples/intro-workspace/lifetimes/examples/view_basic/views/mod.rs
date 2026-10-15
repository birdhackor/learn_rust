// --8<-- [start:text-view]
pub struct TextView<'a> {
    pub text: &'a str,
}
// --8<-- [end:text-view]

// --8<-- [start:make-view]
pub fn make_view(text: &str) -> TextView<'_> {
    TextView { text }
}
// --8<-- [end:make-view]

// --8<-- [start:byte-len]
impl<'a> TextView<'a> {
    pub fn byte_len(&self) -> usize {
        self.text.len()
    }
}
// --8<-- [end:byte-len]
