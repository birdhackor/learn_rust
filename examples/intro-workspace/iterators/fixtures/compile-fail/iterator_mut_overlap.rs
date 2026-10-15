fn main() {
    let mut values = vec![1, 2];
    let mut stream = values.as_mut_slice().iter_mut();
    values.push(3);
    if let Some(item) = stream.next() {
        println!("{}", *item);
    }
}
