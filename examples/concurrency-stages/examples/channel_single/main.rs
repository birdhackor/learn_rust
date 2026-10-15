pub mod workers;

use crate::workers::sum;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel::<Vec<i32>>();
    let worker = thread::spawn(move || sender.send(vec![80, 120]));
    let received = receiver.recv();
    let finished = worker.join();

    match (received, finished) {
        (Ok(values), Ok(Ok(_))) => {
            assert_eq!(values.len(), 2);
            let total = sum(values.as_slice());
            assert_eq!(total, 200);
            println!(
                "收到owned數列，長度={}，小計={total}；worker已收尾",
                values.len()
            );
        }
        _ => println!("已等待worker，訊息或工作有失敗"),
    }
}
