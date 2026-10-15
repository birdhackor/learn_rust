mod sys;

use std::ffi::CString;
use std::ffi::NulError;

// --8<-- [start:version]
pub fn api_version() -> u32 {
    // SAFETY: 對應同 target 的 C uint32_t tutorial_version(void)；無輸入、回固定 1。
    unsafe { sys::tutorial_version() }
}
// --8<-- [end:version]

// --8<-- [start:counts-model]
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Counts {
    pub zeros: u32,
    pub nonzero: u32,
}

#[derive(Debug, PartialEq)]
pub enum CountError {
    TooLong,
}
// --8<-- [end:counts-model]

// --8<-- [start:count-wrapper]
fn checked_len(len: usize) -> Result<u32, CountError> {
    match u32::try_from(len) {
        Ok(len) => Ok(len),
        Err(_) => Err(CountError::TooLong),
    }
}

pub fn count_bytes(bytes: &[u8]) -> Result<Counts, CountError> {
    let len = checked_len(bytes.len())?;
    // SAFETY: byte slice 提供同一 allocation 的有效已初始化範圍，len 完整且可表示。
    // C 同步只讀、不保存或釋放來源；空輸入不存取 pointer。repr(C)／u32 對應 C header。
    Ok(unsafe { sys::tutorial_count_bytes(bytes.as_ptr(), len) })
}
// --8<-- [end:count-wrapper]

// --8<-- [start:label-wrapper]
pub fn is_rust_label(text: &str) -> Result<bool, NulError> {
    let word = CString::new(text)?;
    // SAFETY: word 擁有已初始化、NUL 結尾的 char 資料，直到同步呼叫完成。
    // C 只讀，不保存、寫入或釋放 pointer；c_char／c_int 對應 header 的 char／int。
    let answer = unsafe { sys::tutorial_is_rust(word.as_c_str().as_ptr()) };
    Ok(answer == 1)
}
// --8<-- [end:label-wrapper]
