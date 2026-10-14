fn add_one(value: &mut i32) {
    *value += 1;
}

fn main() {
    let mut count: i32 = 3;
    let parent = &mut count;
    let child = &mut *parent;
    *child += 1;
    println!("短借用讀到={}", *child);
    *parent += 1;

    add_one(&mut *parent);
    add_one(parent);
    println!("parent 最後讀到={}", *parent);
}
