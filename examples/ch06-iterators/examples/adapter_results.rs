use std::num::ParseIntError;

fn main() {
    let texts = vec!["10".to_owned(), "bad".to_owned(), "30".to_owned()];
    let mut visits = 0;
    let result: Result<Vec<i32>, ParseIntError> = texts
        .as_slice()
        .iter()
        .map(|text: &String| {
            visits += 1;
            text.as_str().parse::<i32>()
        })
        .collect();
    match result {
        Ok(values) => println!("整批成功={values:?}"),
        Err(error) => println!("整批解析失敗：{error}"),
    }
    println!("實際解析次數={visits}，原清單筆數={}", texts.len());
    assert_eq!(visits, 2);
}
