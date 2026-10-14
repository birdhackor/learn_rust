use std::sync::Arc;
use std::thread;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let shared = Arc::new(text);
    let worker_shared = Arc::clone(&shared);
    let worker = thread::spawn(move || {
        assert_eq!(worker_shared.as_str(), "Rust");
        worker_shared.len()
    });
    assert_eq!(shared.as_str(), "Rust");
    println!("main也讀同一份文字={}", shared.as_str());
    match worker.join() {
        Ok(length) => {
            assert_eq!(length, 4);
            assert_eq!(shared.as_str(), "Rust");
            println!("worker讀到長度={length}，已收尾；main仍持有Arc");
        }
        Err(_) => println!("worker發生panic，已等待其結束"),
    }
    drop(shared);
}
