use crate::Counts;

use std::ffi::c_char;
use std::ffi::c_int;

// --8<-- [start:version]
unsafe extern "C" {
    pub fn tutorial_version() -> u32;
}
// --8<-- [end:version]

// --8<-- [start:counts]
unsafe extern "C" {
    pub fn tutorial_count_bytes(data: *const u8, len: u32) -> Counts;
}
// --8<-- [end:counts]

// --8<-- [start:label]
unsafe extern "C" {
    pub fn tutorial_is_rust(word: *const c_char) -> c_int;
}
// --8<-- [end:label]
