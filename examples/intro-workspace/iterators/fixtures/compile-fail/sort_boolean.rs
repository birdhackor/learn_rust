fn main() {
    let mut numbers = vec![3, 1, 2];
    numbers.as_mut_slice().sort_by(|left: &i32, right: &i32| *left < *right);
    println!("{numbers:?}");
}
