use std::thread;

fn main() {
    let handle = thread::spawn(|| 200);
    match handle.join() {
        Ok(total) => println!("第一次小計={total}"),
        Err(_) => println!("工作panic"),
    }
    match handle.join() {
        Ok(total) => println!("第二次小計={total}"),
        Err(_) => println!("工作panic"),
    }
}
