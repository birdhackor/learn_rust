use std::sync::mpsc;

fn main() {
    let (sender, receiver) = mpsc::channel::<i32>();
    sender.send(80).expect("receiver仍存在");
    sender.send(120).expect("receiver仍存在");
    drop(sender);

    let first = receiver.recv().expect("80已送入channel");
    let second = receiver.recv().expect("120已送入channel");
    assert_eq!(first, 80);
    assert_eq!(second, 120);
    println!("Sender已關閉，仍收到={first}、{second}");
    match receiver.recv() {
        Ok(_) => panic!("只送了兩項，不應有第三項"),
        Err(_) => println!("待收訊息已空，recv回傳Err"),
    }
}
