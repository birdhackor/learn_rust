mod sys;

use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::NulError;
use std::ffi::c_char;

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

// --8<-- [start:greeting-wrapper]
pub struct Greeting {
    pointer: *mut c_char,
}

impl Greeting {
    pub fn try_new() -> Option<Self> {
        // SAFETY: 無輸入；本地 C 函式交出新五-byte malloc 配置或 NULL。
        let pointer = unsafe { sys::tutorial_greeting() };
        if pointer.is_null() {
            None
        } else {
            Some(Self { pointer })
        }
    }

    pub fn as_c_str(&self) -> &CStr {
        // SAFETY: private pointer 是唯一 owner 的原始成功配置，五個 char 已初始化；
        // 同一 allocation、non-null、含 NUL 且長度小於 isize::MAX。無寫入 API。
        // 回傳 borrow 連 self，仍用 view 時不能 Drop owner 或結束來源。
        unsafe { CStr::from_ptr(self.pointer) }
    }
}

impl Drop for Greeting {
    fn drop(&mut self) {
        // SAFETY: private pointer 只從 tutorial_greeting 成功取得，未 Clone／重複接手；
        // 正常 Drop 時借用已結束，交回對應 C 釋放函式一次，不由 Rust allocator 清理。
        unsafe { sys::tutorial_free_greeting(self.pointer) }
    }
}
// --8<-- [end:greeting-wrapper]

#[cfg(test)]
mod tests {
    use crate::CountError;
    use crate::Counts;
    use crate::Greeting;
    use crate::api_version;
    use crate::checked_len;
    use crate::count_bytes;
    use crate::is_rust_label;
    use std::mem::drop;

    #[test]
    fn the_linked_scalar_function_returns_its_version() {
        assert_eq!(api_version(), 1);
    }

    #[test]
    fn explicit_lengths_count_zeros_as_data_and_limit_the_range() {
        let bytes = [10u8, 0, 20];
        assert_eq!(
            count_bytes(&bytes),
            Ok(Counts {
                zeros: 1,
                nonzero: 2
            })
        );
        assert_eq!(
            count_bytes(&bytes[..2]),
            Ok(Counts {
                zeros: 1,
                nonzero: 1
            })
        );
        assert_eq!(
            count_bytes(&[]),
            Ok(Counts {
                zeros: 0,
                nonzero: 0
            })
        );
        assert_eq!(
            count_bytes(&[0, 0]),
            Ok(Counts {
                zeros: 2,
                nonzero: 0
            })
        );
        assert_eq!(
            count_bytes(&[128, 255]),
            Ok(Counts {
                zeros: 0,
                nonzero: 2
            })
        );
    }

    #[test]
    fn length_validation_rejects_truncation_without_allocating_a_giant_buffer() {
        assert_eq!(checked_len(0), Ok(0));
        assert_eq!(checked_len(3), Ok(3));
        if usize::BITS > 32 {
            assert_eq!(checked_len(usize::MAX), Err(CountError::TooLong));
        }
    }

    #[test]
    fn a_string_with_a_zero_is_rejected_before_the_foreign_call() {
        assert_eq!(is_rust_label("Rust"), Ok(true));
        assert_eq!(is_rust_label("Python"), Ok(false));
        assert_eq!(is_rust_label(""), Ok(false));
        assert!(is_rust_label("Ru\0st").is_err());
        assert_eq!(is_rust_label("教學"), Ok(false));
    }

    #[test]
    fn a_rust_copy_remains_after_the_foreign_owner_is_dropped() {
        let first = Greeting::try_new().expect("測試環境可配置五個 bytes");
        let second = first;
        let saved = second
            .as_c_str()
            .to_str()
            .expect("C 固定提供 ASCII Rust")
            .to_owned();
        drop(second);
        assert_eq!(saved.as_str(), "Rust");
    }

    #[test]
    fn releasing_one_foreign_owner_does_not_end_another() {
        let first = Greeting::try_new().expect("測試環境可配置五個 bytes");
        let second = Greeting::try_new().expect("測試環境可配置另一份五-byte 資料");
        drop(first);
        let text = second.as_c_str().to_str().expect("C 固定提供 ASCII Rust");
        assert_eq!(text, "Rust");
    }
}
