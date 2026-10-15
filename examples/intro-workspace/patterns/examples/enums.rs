// 本節先建立各種形態，下一節才讀取附帶欄位。
#[allow(dead_code)]
// --8<-- [start:definition]
enum Quote {
    Missing,
    Amount(i32),
    Rejected { code: i32 },
}
// --8<-- [end:definition]

fn main() {
    // --8<-- [start:values]
    let _missing = Quote::Missing;
    let _amount = Quote::Amount(120);
    let _rejected = Quote::Rejected { code: 37 };
    // --8<-- [end:values]
    println!("已建立三筆報價：缺少、金額120、拒絕代碼37");
}
