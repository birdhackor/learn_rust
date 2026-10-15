fn main() {
    let first = Box::new(120);
    let second = first;
    println!("{}", *second);
    println!("{}", *first);
}
