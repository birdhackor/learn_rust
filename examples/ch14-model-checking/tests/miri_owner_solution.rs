use std::mem::drop;

#[test]
fn copying_before_drop_keeps_a_plain_value() {
    let owner = Box::new(80);
    let pointer: *const i32 = &raw const *owner;

    // SAFETY: pointer直接由仍保活且已初始化、aligned的Box<i32>衍生。
    // 讀取期間owner未清理，只有此shared讀取，沒有其他寫入。
    let copied = unsafe { *pointer };
    drop(owner);
    assert_eq!(copied, 80);
}
