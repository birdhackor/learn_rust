use ch10_concurrency::workers::sum;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) = mpsc::channel::<Vec<i32>>();
    drop(receiver);
    let values = vec![80, 120];
    match sender.send(values) {
        Ok(_) => panic!("receiver已不存在，傳送應失敗"),
        Err(error) => {
            let recovered = error.0;
            assert_eq!(recovered.len(), 2);
            let total = sum(recovered.as_slice());
            assert_eq!(total, 200);
            println!(
                "傳送失敗，取回原數列，長度={}，小計={total}",
                recovered.len()
            );
        }
    }
}
