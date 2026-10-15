use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let total = Arc::new(Mutex::new(0));
    let worker_total = Arc::clone(&total);
    let worker = thread::spawn(move || {
        let mut guard = worker_total.lock().expect("小數值正常更新不會poison");
        *guard += 80;
    });
    {
        let mut guard = total.lock().expect("小數值正常更新不會poison");
        *guard += 120;
    }
    match worker.join() {
        Ok(_) => {
            let guard = total.lock().expect("兩份更新均正常完成");
            assert_eq!(*guard, 200);
            println!("同一Mutex輪流更新，總和={}；worker已收尾", *guard);
        }
        Err(_) => println!("worker發生panic，已等待；不把部分資料當完整答案"),
    }
}
