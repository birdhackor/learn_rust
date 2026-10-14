use std::rc::Rc;
use std::thread;

fn main() {
    let handle = thread::spawn(|| Rc::new(80));
    match handle.join() {
        Ok(value) => println!("值={}", *value),
        Err(_) => println!("工作panic"),
    }
}
