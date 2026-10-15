use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let value = Rc::new(Mutex::new(0));
    let worker_value = Rc::clone(&value);
    let worker = thread::spawn(move || {
        let mut guard = worker_value.lock().expect("新建Mutex尚未poison");
        *guard += 80;
    });
    match worker.join() {
        Ok(_) => println!("工作完成"),
        Err(_) => println!("工作panic"),
    }
}
