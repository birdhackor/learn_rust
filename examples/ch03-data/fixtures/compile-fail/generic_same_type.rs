struct Pair<T> {
    first: T,
    second: T,
}

fn main() {
    let pair = Pair {
        first: 120,
        second: true,
    };
    println!("{}", pair.first);
}
