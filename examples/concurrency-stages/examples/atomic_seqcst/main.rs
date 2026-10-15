use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;

fn main() {
    let flags = Arc::new((AtomicUsize::new(0), AtomicUsize::new(0)));
    let first_flags = Arc::clone(&flags);
    let second_flags = Arc::clone(&flags);
    let first = thread::spawn(move || {
        first_flags.0.store(1, Ordering::SeqCst);
        first_flags.1.load(Ordering::SeqCst)
    });
    let second = thread::spawn(move || {
        second_flags.1.store(1, Ordering::SeqCst);
        second_flags.0.load(Ordering::SeqCst)
    });
    let first_result = first.join();
    let second_result = second.join();
    match (first_result, second_result) {
        (Ok(seen_by_first), Ok(seen_by_second)) => {
            assert!(seen_by_first == 1 || seen_by_second == 1);
            println!("SeqCst本次觀察：A={seen_by_first}、B={seen_by_second}；不會同為0");
        }
        _ => println!("已等待兩份worker，至少一份panic"),
    }
}
