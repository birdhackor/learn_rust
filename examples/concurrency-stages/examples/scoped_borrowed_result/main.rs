pub mod workers;

use crate::workers::sum;
use std::thread;

fn main() {
    let mut values = vec![80, 120];
    let result = thread::scope(|scope| {
        let worker = scope.spawn(|| values.as_slice());
        worker.join()
    });

    match result {
        Ok(view) => {
            assert_eq!(view.len(), 2);
            assert_eq!(sum(view), 200);
            println!(
                "結果仍借自外部數列，長度={}，小計={}",
                view.len(),
                sum(view)
            );
        }
        Err(_) => println!("借出view的工作panic"),
    }
    values.push(30);
    println!("最後一次借讀後，原數列長度={}", values.len());
}
