unsafe fn read_i32(pointer: *const i32) -> i32 {
    // SAFETY: 此unsafe函式要求caller提供有效、aligned且有讀取權限的i32；
    // 呼叫期間來源仍有效且沒有衝突寫入。i32是Copy，不接手storage。
    unsafe { *pointer }
}

fn main() {
    let value = 80;
    let pointer = &raw const value;
    let observed = read_i32(pointer);
    println!("讀到={observed}");
}
