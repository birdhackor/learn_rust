use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;

fn main() {
    let report = Arc::new((
        AtomicUsize::new(0),
        AtomicUsize::new(0),
        AtomicBool::new(false),
    ));
    let producer_report = Arc::clone(&report);
    let producer = thread::spawn(move || {
        let amounts = [80_usize, 120, 30];
        let length = amounts.len();
        let mut total = 0;
        for amount in amounts {
            total += amount;
        }
        producer_report.0.store(length, Ordering::Relaxed);
        producer_report.1.store(total, Ordering::Relaxed);
        producer_report.2.store(true, Ordering::Release);
    });
    while !report.2.load(Ordering::Acquire) {
        thread::yield_now();
    }
    let length = report.0.load(Ordering::Relaxed);
    let total = report.1.load(Ordering::Relaxed);
    assert_eq!(length, 3);
    assert_eq!(total, 230);
    match producer.join() {
        Ok(_) => println!("一次發布後讀到長度{length}與總和{total}，producer也已收尾"),
        Err(_) => println!("已讀到本次發布並等待producer，其後工作panic"),
    }
}
