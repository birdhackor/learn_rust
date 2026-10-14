#![deny(improper_ctypes)]

struct Counts {
    zeros: u32,
    nonzero: u32,
}

unsafe extern "C" {
    fn tutorial_count_bytes(data: *const u8, len: u32) -> Counts;
}

fn main() {}
