pub mod workers;

use crate::workers::sum;
use std::thread;

fn main() {
    let mut values = vec![80, 120];
    let result = thread::scope(|scope| {
        let worker = scope.spawn(|| sum(values.as_slice()));
        worker.join()
    });

    match result {
        Ok(total) => {
            assert_eq!(total, 200);
            println!("借讀的工作小計={total}");
        }
        Err(_) => println!("借讀工作panic"),
    }
    values.push(30);
    assert_eq!(values.len(), 3);
    println!("scope返回後，原owner仍可更新，長度={}", values.len());
}
