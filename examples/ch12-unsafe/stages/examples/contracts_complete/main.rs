pub mod contracts;

use crate::contracts::ReadOnlyI32;
use std::thread;

fn main() {
    let amount = 80;
    let moved = ReadOnlyI32::new(&amount);
    let shared = ReadOnlyI32::new(&amount);
    println!("wrapper仍借原資料={}", *shared.original());

    let results = thread::scope(|scope| {
        let first = scope.spawn(move || moved.read());
        let second = scope.spawn(|| shared.read());
        let first_result = first.join();
        let second_result = second.join();
        (first_result, second_result)
    });
    match results {
        (Ok(first), Ok(second)) => {
            assert_eq!(first, 80);
            assert_eq!(second, 80);
            println!(
                "移交wrapper讀到{first}，共享wrapper讀到{second}，合計={}",
                first + second
            );
        }
        _ => println!("兩份工作均已等待，至少一份panic"),
    }
    println!("scope結束，原資料仍是={amount}");
}
