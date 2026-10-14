use std::thread;

fn main() {
    let values = vec![80, 120];
    let handle = thread::spawn(move || values.len());
    println!("原數列長度={}", values.len());
    match handle.join() {
        Ok(length) => println!("工作回傳長度={length}"),
        Err(_) => println!("工作panic"),
    }
}
