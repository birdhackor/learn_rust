// 這份故意違約的fixture僅能由Miri解譯，拒絕普通建置。
#[cfg(not(miri))]
compile_error!("use-after-free fixture 僅可用固定nightly的 cargo miri run");

use std::mem::drop;

fn main() {
    let owner = Box::new(80);
    let pointer: *const i32 = &raw const *owner;
    drop(owner);

    // WRONG: owner已清理並釋放allocation；舊pointer不再可讀。
    // 這是故意的UB，不能加入普通test或host執行。
    let copied = unsafe { *pointer };
    println!("這份讀取不應有正常結果：{copied}");
}
