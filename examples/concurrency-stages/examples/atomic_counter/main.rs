use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;

fn main() {
    let count = Arc::new(AtomicUsize::new(0));
    let first_count = Arc::clone(&count);
    let second_count = Arc::clone(&count);
    let first = thread::spawn(move || {
        for _ in 0..3 {
            let _old = first_count.fetch_add(1, Ordering::Relaxed);
        }
    });
    let second = thread::spawn(move || {
        for _ in 0..2 {
            let _old = second_count.fetch_add(1, Ordering::Relaxed);
        }
    });
    let first_result = first.join();
    let second_result = second.join();
    match (first_result, second_result) {
        (Ok(_), Ok(_)) => {
            let current = count.load(Ordering::Relaxed);
            assert_eq!(current, 5);
            println!("兩份worker都正常完成，三次加兩次得到{current}");
        }
        _ => println!("已等待兩份worker，至少一份panic；不當成完整五次工作"),
    }
}
