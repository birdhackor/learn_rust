use std::thread;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    thread::scope(|scope| {
        let worker = scope.spawn(|| -> i32 {
            assert_eq!(text.as_str(), "Rust");
            panic!("借用工作故意panic");
        });
        drop(worker);
    });
    println!("scope正常返回");
}
