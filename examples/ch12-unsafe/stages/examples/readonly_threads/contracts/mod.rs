use std::slice;

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

// --8<-- [start:split]
pub fn split_i32_at_mut(values: &mut [i32], mid: usize) -> Option<(&mut [i32], &mut [i32])> {
    let len = values.len();
    if mid > len {
        return None;
    }
    let data = values.as_mut_ptr();

    // SAFETY: 有效的mutable輸入slice提供已初始化且aligned的i32範圍，
    // pointer保留來源資格，非空範圍位於同一allocation，大小及地址不overflow。
    // mid<=len，故兩段0..mid與mid..len均在原範圍，且沒有重疊元素。
    // len=0時data仍non-null且aligned，add(0)不位移，不存取任何元素。
    // mid=len時right可由one-past與長度0形成，不dereference端點。
    // 回傳兩段都借自輸入；原slice在兩段被使用期間不能另行讀寫。
    unsafe {
        let left = slice::from_raw_parts_mut(data, mid);
        let right = slice::from_raw_parts_mut(data.add(mid), len - mid);
        Some((left, right))
    }
}
// --8<-- [end:split]

// --8<-- [start:readonly-model]
pub struct ReadOnlyI32<'a> {
    source: &'a i32,
    pointer: *const i32,
}

impl<'a> ReadOnlyI32<'a> {
    pub fn new(source: &'a i32) -> Self {
        Self {
            source,
            pointer: &raw const *source,
        }
    }

    pub fn original(&self) -> &'a i32 {
        self.source
    }

    pub fn read(&self) -> i32 {
        // SAFETY: constructor從有效reference建立private欄位，
        // 讓pointer與source連到同一有效i32。
        // source的shared borrow維持來源有效，所有公開操作只讀取；
        // i32沒有interior mutability。沒有可安全改寫pointer或來源的介面。
        unsafe { read_i32(self.pointer) }
    }
}
// --8<-- [end:readonly-model]

// --8<-- [start:readonly-traits]
// SAFETY: 移交wrapper只移交借用資訊，不移動referent。來源由真實&'a i32
// 維持有效且只能shared存取；i32是Sync。private pointer只能讀同一來源，
// 沒有thread專屬資源、寫入或owned allocation的清理操作。
unsafe impl Send for ReadOnlyI32<'_> {}

// SAFETY: 共享wrapper只能呼叫read與original，都是對同一i32的shared讀取。
// i32是Sync且沒有interior mutability；不產生寫入或修改管理狀態。
// 所有pointer都由仍有效的source衍生，公開介面不允許替換此關係。
unsafe impl Sync for ReadOnlyI32<'_> {}
// --8<-- [end:readonly-traits]
