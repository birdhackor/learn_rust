use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

fn main() {
    let count = AtomicUsize::new(0);
    let first_seen = count.load(Ordering::Relaxed);
    let second_seen = count.load(Ordering::Relaxed);
    count.store(first_seen + 1, Ordering::Relaxed);
    count.store(second_seen + 1, Ordering::Relaxed);
    let current = count.load(Ordering::Relaxed);
    assert_eq!(current, 1);
    println!("兩路徑各讀0再各存1，最後只剩{current}；沒有完成兩次增加");
}
