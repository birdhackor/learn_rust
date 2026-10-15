use std::cell::Cell;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

fn main() {
    let value = Arc::new(RwLock::new(Cell::new(80)));
    let reader_value = Arc::clone(&value);
    let reader = thread::spawn(move || {
        let view = reader_value.read().expect("新建RwLock尚未poison");
        view.get()
    });
    match reader.join() {
        Ok(amount) => println!("值={amount}"),
        Err(_) => println!("reader工作panic"),
    }
}
