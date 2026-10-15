use std::str::FromStr;

fn main() {
    let direct = i32::from_str("3");
    let through_parse = "3".parse::<i32>();
    assert_eq!(direct, Ok(3));
    assert_eq!(through_parse, Ok(3));
    println!("兩種寫法都取得整數 3");

    for text in ["abc", "3 ", "2147483648"] {
        match text.parse::<i32>() {
            Ok(value) => println!("成功：{value}"),
            Err(error) => println!("解析 {text:?} 失敗：{error}"),
        }
    }
}
