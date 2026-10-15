pub mod workers;

use crate::workers::sum;
use std::thread;

fn main() {
    let values = vec![80, 120];
    let mut total = 0;
    thread::scope(|scope| {
        let worker = scope.spawn(|| {
            total = sum(values.as_slice());
        });
        drop(worker);
    });

    assert_eq!(total, 200);
    assert_eq!(values.len(), 2);
    println!(
        "scope已等待工作body，小計={total}，原數列長度={}",
        values.len()
    );
}
