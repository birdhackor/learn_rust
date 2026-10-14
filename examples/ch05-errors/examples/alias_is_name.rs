type CountNumber = i32;

fn main() {
    let unchecked: CountNumber = -1;
    let ordinary: i32 = unchecked;
    println!("alias沒有自動驗證，數字仍是={ordinary}");
    assert_eq!(ordinary, -1);
}
