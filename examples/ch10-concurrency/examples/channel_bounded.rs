use ch10_concurrency::bounded_messages::send_bounded_values;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::sync_channel::<i32>(1);
    let worker = thread::spawn(move || send_bounded_values(sender, vec![80, 120, 30]));
    let mut total = 0;
    let mut received = 0;
    while let Ok(value) = receiver.recv() {
        total += value;
        received += 1;
    }
    match worker.join() {
        Ok(Ok(sent)) => {
            assert_eq!(sent, 3);
            assert_eq!(received, 3);
            assert_eq!(total, 230);
            println!("容量1，收完{received}項，總和={total}；worker已收尾");
        }
        _ => println!("已等待worker，工作失敗；不把部分總和當完整答案"),
    }
}
