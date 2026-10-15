use std::slice;

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
