use std::cell::Cell;
use std::thread;

fn main() {
    let counter = Cell::new(0);
    thread::scope(|scope| {
        let worker = scope.spawn(|| counter.get());
        match worker.join() {
            Ok(value) => println!("值={value}"),
            Err(_) => println!("工作panic"),
        }
    });
}
