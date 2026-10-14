use std::cell::Cell;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let total = Arc::new(Mutex::new(Cell::new(0)));
    let worker_total = Arc::clone(&total);
    let worker = thread::spawn(move || {
        let guard = worker_total.lock().expect("小數值正常更新不會poison");
        guard.set(guard.get() + 80);
    });
    {
        let guard = total.lock().expect("小數值正常更新不會poison");
        guard.set(guard.get() + 120);
    }
    match worker.join() {
        Ok(_) => {
            let guard = total.lock().expect("兩份更新均正常完成");
            assert_eq!(guard.get(), 200);
            println!("Cell沒有Sync，由Mutex保護輪流操作，總和={}", guard.get());
        }
        Err(_) => println!("worker發生panic，已等待；不把部分資料當完整答案"),
    }
}
