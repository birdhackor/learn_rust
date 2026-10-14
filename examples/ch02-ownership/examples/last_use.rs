fn main() {
    let mut count: i32 = 3;
    let first = &count;
    let second = &count;
    println!("兩次 shared 讀取={}、{}", *first, *second);

    let writable = &mut count;
    *writable += 1;
    println!("mutable 借用讀到={}", *writable);

    count += 1;
    println!("借用結束後，owner 改成={count}");
}
