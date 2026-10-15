pub mod workers;

use crate::workers::WorkError;
use crate::workers::check_amount;
use std::thread;

fn main() {
    for amount in [200, -1] {
        let handle = thread::spawn(move || check_amount(amount));
        match handle.join() {
            Ok(work_result) => match work_result {
                Ok(value) => println!("工作正常回傳，接受金額={value}"),
                Err(WorkError::Negative(value)) => {
                    println!("工作正常回傳，拒絕負金額={value}");
                }
            },
            Err(_) => println!("工作panic，沒有金額判定"),
        }
    }
}
