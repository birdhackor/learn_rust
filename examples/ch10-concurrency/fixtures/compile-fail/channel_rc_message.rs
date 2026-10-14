use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel::<Rc<i32>>();
    let worker = thread::spawn(move || {
        let _result = sender.send(Rc::new(200));
    });
    drop(receiver);
    match worker.join() {
        Ok(_) => println!("工作完成"),
        Err(_) => println!("工作panic"),
    }
}
