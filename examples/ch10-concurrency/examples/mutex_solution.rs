use ch10_concurrency::workers::sum;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let values = Arc::new(Mutex::new(Vec::new()));
    let first_values = Arc::clone(&values);
    let second_values = Arc::clone(&values);
    let first_worker = thread::spawn(move || {
        let mut added: usize = 0;
        for value in [80, 120] {
            let mut guard = first_values.lock().expect("正常小數列記錄不會poison");
            guard.push(value);
            added += 1;
        }
        added
    });
    let second_worker = thread::spawn(move || {
        let mut added: usize = 0;
        for value in [1, 2, 3] {
            let mut guard = second_values.lock().expect("正常小數列記錄不會poison");
            guard.push(value);
            added += 1;
        }
        added
    });
    let first_result = first_worker.join();
    let second_result = second_worker.join();
    match (first_result, second_result) {
        (Ok(first_added), Ok(second_added)) => {
            assert_eq!(first_added, 2);
            assert_eq!(second_added, 3);
            let guard = values.lock().expect("兩份記錄都正常完成");
            assert_eq!(guard.len(), 5);
            let total = sum(guard.as_slice());
            assert_eq!(total, 206);
            println!(
                "同一清單收到{}項，總和={total}；兩個worker已收尾",
                guard.len()
            );
        }
        _ => println!("已等待兩個worker，至少一份panic；清單可能只更新了一部分"),
    }
}
