fn main() {
    let mut values = vec![80, 120];
    let first = values.as_slice().get(0);
    values.push(200);
    if let Some(value) = first {
        println!("{}", *value);
    }
}
