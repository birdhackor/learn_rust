// --8<-- [start:references]
pub fn shorten_shared<'long: 'short, 'short>(value: &'long str) -> &'short str {
    value
}

pub fn shorten_mutable<'long: 'short, 'short>(value: &'long mut i32) -> &'short mut i32 {
    value
}
// --8<-- [end:references]

// --8<-- [start:callbacks]
pub fn adapt_input<'long: 'short, 'short>(
    operation: fn(&'short str) -> usize,
) -> fn(&'long str) -> usize {
    operation
}

pub fn adapt_output<'long: 'short, 'short>(operation: fn() -> &'long str) -> fn() -> &'short str {
    operation
}
// --8<-- [end:callbacks]
