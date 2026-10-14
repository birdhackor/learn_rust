use ch12_ffi::Greeting;
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
    println!("借讀C配置={}", view.to_str().expect("本地C提供ASCII Rust"));
    let saved = view.to_str().expect("相同ASCII文字").to_owned();
    drop(greeting);
    println!("之後只用Rust副本={saved}");
}
