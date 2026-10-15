pub mod conversions;
pub mod formatting;
pub mod policy;

use crate::conversions::Count;

fn main() {
    for value in [0, 3, 100, -1, 101] {
        match Count::try_from(value) {
            Ok(count) => {
                println!("已驗證件數：{}", count.value());
                let raw: i32 = count.into();
                println!("取回 i32：{raw}");
            }
            Err(error) => println!("轉換失敗：{error}"),
        }
    }
}
