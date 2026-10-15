use std::num::ParseIntError;

// --8<-- [start:parse-all]
fn parse_all(texts: &[String]) -> Result<Vec<i32>, ParseIntError> {
    texts
        .iter()
        .map(|text: &String| text.as_str().parse::<i32>())
        .collect()
}
// --8<-- [end:parse-all]

#[allow(clippy::unnecessary_fold)]
fn parse_total(texts: &[String]) -> Result<i32, ParseIntError> {
    let values = parse_all(texts)?;
    Ok(values
        .into_iter()
        .fold(0, |total: i32, value: i32| total + value))
}

fn main() {
    let texts = vec!["0".to_owned(), "3".to_owned(), "-2".to_owned()];
    assert_eq!(parse_total(texts.as_slice()), Ok(1));
    let invalid = vec!["3".to_owned(), "bad".to_owned()];
    match parse_total(invalid.as_slice()) {
        Ok(total) => println!("合計={total}"),
        Err(error) => println!("保留整數解析失敗：{error}"),
    }
    let empty: Vec<String> = Vec::new();
    assert_eq!(parse_total(empty.as_slice()), Ok(0));
}
