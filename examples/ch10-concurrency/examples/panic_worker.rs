use ch10_concurrency::workers::panic_message;
use std::thread;

fn main() {
    let handle = thread::spawn(|| -> i32 {
        panic!("工作故意panic");
    });

    match handle.join() {
        Ok(_) => panic!("這個範例應收到工作panic"),
        Err(payload) => {
            let message = panic_message(payload);
            assert_eq!(message.as_str(), "工作故意panic");
            println!("主thread收到panic：{message}");
        }
    }
    println!("主thread完成收尾");
}
