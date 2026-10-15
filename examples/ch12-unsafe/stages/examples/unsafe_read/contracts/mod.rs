// --8<-- [start:read-i32]
/// 從一個已符合讀取條件的pointer，取得i32副本。
///
/// # Safety
///
/// pointer必須non-null、對i32正確aligned，並衍生自仍有效的storage，
/// 可讀完整一個已初始化且有效的i32。呼叫期間storage不能移動或釋放；
/// 此次讀取須符合來源的aliasing權限，也不能與任何衝突寫入同時進行。
pub unsafe fn read_i32(pointer: *const i32) -> i32 {
    // SAFETY: 呼叫者依上述契約提供有效且有合法讀取權限的i32位置。
    // i32是Copy；只取得副本，不建立更長命的reference或接手storage。
    unsafe { *pointer }
}
// --8<-- [end:read-i32]
