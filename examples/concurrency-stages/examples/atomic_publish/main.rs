use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;

fn main() {
    let report = Arc::new((AtomicUsize::new(0), AtomicBool::new(false)));
    let producer_report = Arc::clone(&report);
    let producer = thread::spawn(move || {
        producer_report.0.store(230, Ordering::Relaxed);
        producer_report.1.store(true, Ordering::Release);
    });
    while !report.1.load(Ordering::Acquire) {
        thread::yield_now();
    }
    let total = report.0.load(Ordering::Relaxed);
    assert_eq!(total, 230);
    match producer.join() {
        Ok(_) => println!("Acquire讀到唯一發布的true，讀到總和{total}；producer已收尾"),
        Err(_) => println!("已讀到本次發布並等待producer，其後工作panic"),
    }
}
