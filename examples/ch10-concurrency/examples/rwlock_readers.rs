use ch10_concurrency::workers::sum;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

fn main() {
    let values = Arc::new(RwLock::new(vec![80, 120]));
    let first_values = Arc::clone(&values);
    let second_values = Arc::clone(&values);
    let first_reader = thread::spawn(move || {
        let view = first_values.read().expect("本例沒有writer或poison");
        view.len()
    });
    let second_reader = thread::spawn(move || {
        let view = second_values.read().expect("本例沒有writer或poison");
        sum(view.as_slice())
    });
    let first_result = first_reader.join();
    let second_result = second_reader.join();
    match (first_result, second_result) {
        (Ok(length), Ok(total)) => {
            assert_eq!(length, 2);
            assert_eq!(total, 200);
            println!("兩份reader工作已收尾，長度={length}，總和={total}");
        }
        _ => println!("已等待兩份reader，至少一份panic"),
    }
}
