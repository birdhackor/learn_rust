pub mod workers;

use crate::workers::sum;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

fn main() {
    let values = Arc::new(RwLock::new(vec![80, 120]));
    let writer_values = Arc::clone(&values);
    let writer = thread::spawn(move || {
        let mut guard = writer_values.write().expect("正常追加不會poison");
        guard.push(30);
    });
    match writer.join() {
        Ok(_) => {
            let first_values = Arc::clone(&values);
            let second_values = Arc::clone(&values);
            let first_reader = thread::spawn(move || {
                let view = first_values.read().expect("writer已正常完成");
                view.len()
            });
            let second_reader = thread::spawn(move || {
                let view = second_values.read().expect("writer已正常完成");
                sum(view.as_slice())
            });
            let first_result = first_reader.join();
            let second_result = second_reader.join();
            match (first_result, second_result) {
                (Ok(length), Ok(total)) => {
                    assert_eq!(length, 3);
                    assert_eq!(total, 230);
                    println!("writer先完成，兩reader也已收尾，長度={length}，總和={total}");
                }
                _ => println!("已等待兩reader，至少一份panic"),
            }
        }
        Err(_) => println!("已等待writer，其工作panic；不建立依賴完整更新的reader"),
    }
}
