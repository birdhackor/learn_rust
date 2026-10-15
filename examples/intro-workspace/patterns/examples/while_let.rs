enum Batch {
    More(i32),
    Done,
}

// --8<-- [start:next]
fn next_batch(remaining: &mut i32) -> Batch {
    if *remaining == 0 {
        return Batch::Done;
    }
    let processed = if *remaining >= 3 { 3 } else { *remaining };
    *remaining -= processed;
    Batch::More(processed)
}
// --8<-- [end:next]

// --8<-- [start:loop]
fn batch_count(work: i32) -> i32 {
    let mut remaining = work;
    let mut batches = 0;
    while let Batch::More(processed) = next_batch(&mut remaining) {
        batches += 1;
        println!("本批處理{processed}，剩餘{remaining}");
    }
    batches
}
// --8<-- [end:loop]

fn main() {
    let seven = batch_count(7);
    let empty = batch_count(0);
    assert_eq!(seven, 3);
    assert_eq!(empty, 0);
    println!("七項共{seven}批，零項共{empty}批");
}
