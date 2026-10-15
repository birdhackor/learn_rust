fn main() {
    let mut count: i32 = 3;
    let parent = &mut count;
    let next = parent;
    *next += 1;
    *parent += 1;
}
