use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

fn main() {
    let count = AtomicUsize::new(0);
    let first_seen = count.load(Ordering::SeqCst);
    let second_seen = count.load(Ordering::SeqCst);
    count.store(first_seen + 1, Ordering::SeqCst);
    count.store(second_seen + 1, Ordering::SeqCst);
    let current = count.load(Ordering::SeqCst);
    assert_eq!(current, 1);
    println!("即使五次存取都SeqCst，兩個分開的讀改寫仍只得到{current}");
}
