struct Holder<T> {
    value: T,
}

fn hold<T>(value: T) -> Holder<T> {
    Holder { value }
}

fn main() {
    let number: Holder<i32> = hold(120);
    let ready: Holder<bool> = hold(true);
    let explicit = hold::<i32>(80);
    println!("數字盒={}，布林盒={}", number.value, ready.value);
    println!("指定型別的數字盒={}", explicit.value);
    assert_eq!(number.value, 120);
    assert!(ready.value);
    assert_eq!(explicit.value, 80);
}
