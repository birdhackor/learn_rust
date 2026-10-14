fn main() {
    let values = vec![1, 2];
    let owned: Vec<i32> = values.as_slice().iter().collect();
    println!("{owned:?}");
}
