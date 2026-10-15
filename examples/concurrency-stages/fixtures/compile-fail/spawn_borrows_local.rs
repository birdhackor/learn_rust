use std::thread;

fn main() {
    let values = vec![80, 120];
    let handle = thread::spawn(|| values.len());
    match handle.join() {
        Ok(length) => println!("長度={length}"),
        Err(_) => println!("工作panic"),
    }
}
