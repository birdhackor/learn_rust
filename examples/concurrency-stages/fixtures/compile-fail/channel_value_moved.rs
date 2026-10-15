use std::sync::mpsc;

fn main() {
    let (sender, receiver) = mpsc::channel::<Vec<i32>>();
    let values = vec![80, 120];
    let _result = sender.send(values);
    println!("原數列長度={}", values.len());
    drop(receiver);
}
