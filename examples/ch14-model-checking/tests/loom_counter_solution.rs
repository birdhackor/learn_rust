use loom::sync::Arc;
use loom::sync::atomic::AtomicUsize;
use loom::sync::atomic::Ordering;
use loom::thread;

#[test]
fn unequal_updates_preserve_both_workers_contributions() {
    loom::model(|| {
        let count = Arc::new(AtomicUsize::new(80));
        let first_count = Arc::clone(&count);
        let second_count = Arc::clone(&count);
        let first = thread::spawn(move || {
            let _old = first_count.fetch_add(20, Ordering::Relaxed);
        });
        let second = thread::spawn(move || {
            let _old = second_count.fetch_add(5, Ordering::Relaxed);
        });
        let first_result = first.join();
        let second_result = second.join();
        match (first_result, second_result) {
            (Ok(_), Ok(_)) => assert_eq!(count.load(Ordering::Relaxed), 105),
            _ => panic!("兩份worker都應正常完成"),
        }
    });
}
