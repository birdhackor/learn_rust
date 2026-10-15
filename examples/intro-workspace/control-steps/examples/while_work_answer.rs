fn batch_count(work: i32, capacity: i32) -> i32 {
    let mut remaining = work;
    let mut batches = 0;
    while remaining > 0 {
        let processed = if remaining >= capacity {
            capacity
        } else {
            remaining
        };
        remaining -= processed;
        batches += 1;
    }
    batches
}

fn main() {
    let partial = batch_count(7, 3);
    let exact = batch_count(6, 3);
    let empty = batch_count(0, 3);
    println!("七項={partial}批，六項={exact}批，零項={empty}批");
}
