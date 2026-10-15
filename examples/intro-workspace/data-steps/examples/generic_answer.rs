struct Pair<T, U> {
    first: T,
    second: U,
}

fn pair<T, U>(first: T, second: U) -> Pair<T, U> {
    Pair { first, second }
}

fn main() {
    let status: Pair<i32, bool> = pair(120, true);
    assert_eq!(status.first, 120);
    assert!(status.second);
    println!("金額={}，是否可用={}", status.first, status.second);
}
