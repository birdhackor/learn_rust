use ch10_concurrency::workers::panic_message;
use std::thread;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let result = thread::scope(|scope| {
        let worker = scope.spawn(|| -> i32 {
            assert_eq!(text.as_str(), "Rust");
            panic!("借用工作故意panic");
        });
        worker.join()
    });

    match result {
        Ok(_) => panic!("這個範例應收到工作panic"),
        Err(payload) => {
            let message = panic_message(payload);
            assert_eq!(message.as_str(), "借用工作故意panic");
            println!("手動join收到panic：{message}");
        }
    }
    println!("scope正常返回，原文字={}", text.as_str());
}
