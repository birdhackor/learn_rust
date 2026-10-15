// --8<-- [start:batch-function]
fn batch_count(work: i32) -> i32 {
    let mut remaining = work;
    let mut batches = 0;
    while remaining > 0 {
        let processed = if remaining >= 3 { 3 } else { remaining };
        remaining -= processed;
        batches += 1;
        println!("完成第{batches}批，這批處理{processed}，剩餘{remaining}");
    }
    batches
}
// --8<-- [end:batch-function]

fn main() {
    let first = batch_count(7);
    println!("七項工作共{first}批");
    let empty = batch_count(0);
    println!("零項工作共{empty}批");
}
