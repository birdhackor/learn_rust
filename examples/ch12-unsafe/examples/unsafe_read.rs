use ch12_unsafe::contracts::read_i32;

fn main() {
    let amount = 80;
    let pointer: *const i32 = &raw const amount;
    // SAFETY: pointer直接來自仍有效、已初始化且aligned的amount。
    // 呼叫期間只讀取，沒有衝突寫入、資料移動或storage釋放。
    let observed = unsafe { read_i32(pointer) };
    println!("unsafe函式讀到副本={observed}，原資料仍是={amount}");
}
