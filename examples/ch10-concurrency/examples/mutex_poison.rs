use ch10_concurrency::workers::panic_message;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

struct Totals {
    total: i32,
    count: usize,
}

fn main() {
    let shared = Arc::new(Mutex::new(Totals { total: 0, count: 0 }));
    let worker_shared = Arc::clone(&shared);
    let worker = thread::spawn(move || -> () {
        let mut guard = worker_shared.lock().expect("新建Mutex尚未poison");
        guard.total += 80;
        panic!("尚未更新完成筆數就panic");
    });
    match worker.join() {
        Ok(_) => panic!("此受控範例的worker應發生panic"),
        Err(payload) => {
            let message = panic_message(payload);
            assert_eq!(message.as_str(), "尚未更新完成筆數就panic");
            println!("已等待worker，收到panic：{message}");
        }
    }

    match shared.lock() {
        Ok(_) => panic!("持鎖panic後，此範例應收到poison警告"),
        Err(error) => {
            let mut guard = error.into_inner();
            assert_eq!(guard.total, 80);
            assert_eq!(guard.count, 0);
            println!(
                "取回guard仍見部分修改：總額={}，完成筆數={}",
                guard.total, guard.count
            );
            guard.total = 0;
            guard.count = 0;
            shared.clear_poison();
        }
    }

    let guard = shared.lock().expect("已恢復已知狀態並清除poison");
    assert_eq!(guard.total, 0);
    assert_eq!(guard.count, 0);
    println!(
        "重設後再次lock成功：總額={}，完成筆數={}",
        guard.total, guard.count
    );
}
