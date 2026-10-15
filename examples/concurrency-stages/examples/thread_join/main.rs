pub mod workers;

use crate::workers::sum;
use std::thread;

fn main() {
    let values = vec![80, 120];
    let handle = thread::spawn(move || sum(values.as_slice()));

    let local_values = [1, 2, 3];
    let local_total = sum(&local_values[..]);
    println!("主thread小計={local_total}");

    match handle.join() {
        Ok(worker_total) => {
            let total = local_total + worker_total;
            assert_eq!(worker_total, 200);
            assert_eq!(total, 206);
            println!("工作小計={worker_total}，總和={total}");
        }
        Err(_) => println!("工作panic，沒有完整總和"),
    }
}
