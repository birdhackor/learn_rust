use std::cell::Cell;
use std::sync::Arc;
use std::thread;

fn main() {
    let value = Arc::new(Cell::new(80));
    let worker_value = Arc::clone(&value);
    let worker = thread::spawn(move || worker_value.get());
    match worker.join() {
        Ok(amount) => println!("值={amount}"),
        Err(_) => println!("工作panic"),
    }
}
