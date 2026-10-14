use ch10_concurrency::bounded_messages::send_bounded_values;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::sync_channel::<i32>(0);
    let worker = thread::spawn(move || send_bounded_values(sender, vec![80, 120, 30]));
    let first = receiver.recv().expect("worker應送出第一項");
    assert_eq!(first, 80);
    println!("收到第一項={first}");
    drop(receiver);

    match worker.join() {
        Ok(Err(error)) => {
            let unsent = error.0;
            assert_eq!(unsent, 120);
            println!("停止後取回未送值={unsent}；worker已收尾");
        }
        Ok(Ok(_)) => panic!("容量0只接收一項，不應成功送完三項"),
        Err(_) => println!("worker發生panic，已等待其結束"),
    }
}
