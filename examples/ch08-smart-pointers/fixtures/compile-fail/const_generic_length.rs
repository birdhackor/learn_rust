fn fixed_len<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

fn main() {
    let counts: [i32; 2] = [10, 20];
    println!("N={}", fixed_len::<i32, 3>(&counts));
}
