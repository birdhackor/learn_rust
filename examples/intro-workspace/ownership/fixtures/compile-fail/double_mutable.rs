fn main() {
    let mut count: i32 = 3;
    let first = &mut count;
    let second = &mut count;
    *first += 1;
    *second += 1;
}
