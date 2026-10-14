fn main() {
    let values = vec![1, 2];
    let stream = values.into_iter();
    println!("原 Vec 長度={}", values.len());
    for value in stream {
        println!("{value}");
    }
}
