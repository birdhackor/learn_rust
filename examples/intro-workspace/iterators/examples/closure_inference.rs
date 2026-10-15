fn main() {
    let double = |number| number * 2;
    let result: i32 = double(4);
    println!("推導後的數字結果={result}");
    assert_eq!(double(6), 12);
}
