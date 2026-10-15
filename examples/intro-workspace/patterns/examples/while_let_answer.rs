enum Batch {
    More(i32),
    Done,
}

fn next_batch(remaining: &mut i32, capacity: i32) -> Batch {
    if *remaining == 0 {
        return Batch::Done;
    }
    let processed = if *remaining >= capacity {
        capacity
    } else {
        *remaining
    };
    *remaining -= processed;
    Batch::More(processed)
}

fn batch_count(work: i32, capacity: i32) -> i32 {
    assert!(work >= 0);
    assert!(capacity > 0);
    let mut remaining = work;
    let mut batches = 0;
    while let Batch::More(processed) = next_batch(&mut remaining, capacity) {
        batches += 1;
        println!("這批處理{processed}，剩餘{remaining}");
    }
    batches
}

fn main() {
    let partial = batch_count(7, 3);
    let exact = batch_count(6, 3);
    let empty = batch_count(0, 3);
    assert_eq!(partial, 3);
    assert_eq!(exact, 2);
    assert_eq!(empty, 0);
    println!("七項={partial}批，六項={exact}批，零項={empty}批");
}
