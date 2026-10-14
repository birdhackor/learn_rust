use ch10_concurrency::workers::sum;
use std::thread;

fn main() {
    let mut first = vec![80, 120];
    let mut second = vec![1, 2, 3];
    let results = thread::scope(|scope| {
        let first_worker = scope.spawn(|| {
            for value in first.as_mut_slice() {
                *value += 1;
            }
            first.len()
        });
        let second_worker = scope.spawn(|| {
            for value in second.as_mut_slice() {
                *value += 1;
            }
            second.len()
        });
        let first_result = first_worker.join();
        let second_result = second_worker.join();
        (first_result, second_result)
    });

    match results {
        (Ok(first_len), Ok(second_len)) => {
            assert_eq!(first_len, 2);
            assert_eq!(second_len, 3);
            assert_eq!(sum(first.as_slice()), 202);
            assert_eq!(sum(second.as_slice()), 9);
            let total = sum(first.as_slice()) + sum(second.as_slice());
            assert_eq!(total, 211);
            println!("兩份獨占借用工作完成，長度={first_len}、{second_len}，總和={total}");
        }
        _ => println!("兩份工作都已等待，至少一份panic，資料可能已部分修改"),
    }
}
