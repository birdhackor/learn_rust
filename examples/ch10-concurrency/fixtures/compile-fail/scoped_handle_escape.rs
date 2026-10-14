use std::thread;

fn main() {
    let worker = thread::scope(|scope| scope.spawn(|| 200));
    match worker.join() {
        Ok(total) => println!("小計={total}"),
        Err(_) => println!("工作panic"),
    }
}
