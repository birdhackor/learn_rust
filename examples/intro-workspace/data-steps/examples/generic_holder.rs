// --8<-- [start:holder]
struct Holder<T> {
    value: T,
}
// --8<-- [end:holder]

// --8<-- [start:hold]
fn hold<T>(value: T) -> Holder<T> {
    Holder { value }
}
// --8<-- [end:hold]

fn main() {
    // --8<-- [start:calls]
    let number: Holder<i32> = hold(120);
    let ready: Holder<bool> = hold(true);
    // --8<-- [end:calls]
    // --8<-- [start:explicit]
    let explicit = hold::<i32>(80);
    // --8<-- [end:explicit]
    println!("數字盒={}，布林盒={}", number.value, ready.value);
    println!("指定型別的數字盒={}", explicit.value);
    assert_eq!(number.value, 120);
    assert!(ready.value);
    assert_eq!(explicit.value, 80);
}
