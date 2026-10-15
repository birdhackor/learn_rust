use loom::sync::Arc;
use loom::sync::atomic::AtomicUsize;
use loom::sync::atomic::Ordering;
use loom::thread;

#[test]
fn split_load_and_store_can_lose_an_update() {
    loom::model(|| {
        let count = Arc::new(AtomicUsize::new(0));
        let first_count = Arc::clone(&count);
        let second_count = Arc::clone(&count);
        let first = thread::spawn(move || {
            let observed = first_count.load(Ordering::Relaxed);
            first_count.store(observed + 1, Ordering::Relaxed);
        });
        let second = thread::spawn(move || {
            let observed = second_count.load(Ordering::Relaxed);
            second_count.store(observed + 1, Ordering::Relaxed);
        });
        let first_result = first.join();
        let second_result = second.join();
        match (first_result, second_result) {
            (Ok(_), Ok(_)) => assert_eq!(count.load(Ordering::Relaxed), 2),
            _ => panic!("兩份worker都應正常完成"),
        }
    });
}
