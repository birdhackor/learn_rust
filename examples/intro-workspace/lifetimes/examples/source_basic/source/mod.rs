pub fn same(text: &str) -> &str {
    text
}

// --8<-- [start:longer]
pub fn longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}
// --8<-- [end:longer]
