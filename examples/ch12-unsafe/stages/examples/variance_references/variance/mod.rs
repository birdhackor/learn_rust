// --8<-- [start:references]
pub fn shorten_shared<'long: 'short, 'short>(value: &'long str) -> &'short str {
    value
}

pub fn shorten_mutable<'long: 'short, 'short>(value: &'long mut i32) -> &'short mut i32 {
    value
}
// --8<-- [end:references]
