mod sys;

// --8<-- [start:version]
pub fn api_version() -> u32 {
    // SAFETY: 對應同 target 的 C uint32_t tutorial_version(void)；無輸入、回固定 1。
    unsafe { sys::tutorial_version() }
}
// --8<-- [end:version]
