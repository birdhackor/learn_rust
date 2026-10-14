use ch10_concurrency::messages::send_values;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel::<i32>();
    let first_sender = sender.clone();
    let second_sender = sender.clone();
    let first_worker = thread::spawn(move || send_values(first_sender, vec![80, 120]));
    let second_worker = thread::spawn(move || send_values(second_sender, vec![1, 2, 3]));
    drop(sender);

    let mut total = 0;
    let mut received = 0;
    while let Ok(value) = receiver.recv() {
        total += value;
        received += 1;
    }
    let first_result = first_worker.join();
    let second_result = second_worker.join();
    match (first_result, second_result) {
        (Ok(Ok(first_sent)), Ok(Ok(second_sent))) => {
            assert_eq!(first_sent, 2);
            assert_eq!(second_sent, 3);
            assert_eq!(received, 5);
            assert_eq!(total, 206);
            println!("收完{received}項，總和={total}；兩個worker已收尾");
        }
        _ => println!("已等待兩個worker，至少一項工作失敗；不把部分總和當完整答案"),
    }
}
