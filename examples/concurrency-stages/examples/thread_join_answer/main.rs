pub mod workers;

use crate::workers::sum;
use std::thread;

fn main() {
    let first = vec![80, 120];
    let second = vec![1, 2, 3];
    let first_handle = thread::spawn(move || sum(first.as_slice()));
    let second_handle = thread::spawn(move || sum(second.as_slice()));

    let first_result = first_handle.join();
    let second_result = second_handle.join();

    match (first_result, second_result) {
        (Ok(first_total), Ok(second_total)) => {
            let total = first_total + second_total;
            assert_eq!(first_total, 200);
            assert_eq!(second_total, 6);
            assert_eq!(total, 206);
            println!("兩份工作都正常回傳，總和={total}");
        }
        _ => println!("兩份工作都已等待，至少一份panic，沒有完整總和"),
    }
}
