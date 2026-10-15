// --8<-- [start:helper]
fn add_one(value: &mut i32) {
    *value += 1;
}
// --8<-- [end:helper]

fn main() {
    let mut count: i32 = 3;
    // --8<-- [start:short-borrow]
    let parent = &mut count;
    let child = &mut *parent;
    *child += 1;
    println!("短借用讀到={}", *child);
    *parent += 1;
    // --8<-- [end:short-borrow]

    // --8<-- [start:calls]
    add_one(&mut *parent);
    add_one(parent);
    println!("parent 最後讀到={}", *parent);
    // --8<-- [end:calls]
}
