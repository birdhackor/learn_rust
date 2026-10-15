use crate::Counts;

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
