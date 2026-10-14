use ch05_errors::conversions::Count;
use ch05_errors::parsing::ParseCountError;
use std::convert::AsRef;

fn parse_count<T: AsRef<str>>(input: &T) -> Result<Count, ParseCountError> {
    input.as_ref().parse::<Count>()
}

fn main() {
    let borrowed = "100";
    let mut owned = String::new();
    owned.push_str("100");
    let from_borrowed = parse_count(&borrowed).expect("固定文字表示有效件數");
    let from_owned = parse_count(&owned).expect("固定文字表示有效件數");
    assert_eq!(from_borrowed, from_owned);
    println!("兩種來源都取得件數 {}", from_owned.value());
    println!("原 String 仍在：{}", owned.as_str());

    let invalid = "101";
    match parse_count(&invalid) {
        Ok(count) => println!("件數 {}", count.value()),
        Err(error) => println!("失敗原因：{error}"),
    }
}
