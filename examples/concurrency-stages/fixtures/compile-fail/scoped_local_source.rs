use std::thread;

fn main() {
    thread::scope(|scope| {
        let values = vec![80, 120];
        let worker = scope.spawn(|| values.len());
        match worker.join() {
            Ok(length) => println!("長度={length}"),
            Err(_) => println!("工作panic"),
        }
    });
}
