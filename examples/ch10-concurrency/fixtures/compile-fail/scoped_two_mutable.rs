use std::thread;

fn main() {
    let mut total = 0;
    thread::scope(|scope| {
        let first = scope.spawn(|| {
            total += 80;
        });
        let second = scope.spawn(|| {
            total += 120;
        });
        drop(first);
        drop(second);
    });
    println!("小計={total}");
}
