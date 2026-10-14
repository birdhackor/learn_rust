use ch12_ffi::Greeting;
use ch12_ffi::count_bytes;
use ch12_ffi::is_rust_label;
use std::mem::drop;

fn main() {
    let greeting = match Greeting::try_new() {
        Some(greeting) => greeting,
        None => {
            println!("C配置失敗");
            return;
        }
    };
    let view = greeting.as_c_str();
    let counts = count_bytes(view.to_bytes()).expect("固定四-byte文字可由u32表示");
    let text = view.to_str().expect("本地C提供ASCII Rust");
    let same = is_rust_label(text).expect("四字元沒有內部零");
    let saved = text.to_owned();
    drop(greeting);
    println!("不含終止零：零={}，非零={}", counts.zeros, counts.nonzero);
    println!("相同標籤={same}，獨立Rust文字={saved}");
}
