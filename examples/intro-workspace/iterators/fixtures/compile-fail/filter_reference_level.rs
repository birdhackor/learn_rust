fn main() {
    let values = vec![1, 2];
    let filtered = values.as_slice().iter().filter(|item: &i32| *item > 0);
    for item in filtered {
        println!("{}", *item);
    }
}
