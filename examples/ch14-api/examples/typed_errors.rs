use ch14_api::BatchLimit;
use ch14_api::LimitError;
use ch14_api::Options;
use ch14_api::SearchError;
use ch14_api::search;
use std::error::Error;
use std::io;

fn main() {
    for value in [0, 1001] {
        match BatchLimit::new(value) {
            Ok(limit) => println!("有效上限={}", limit.get()),
            Err(LimitError::Zero) => println!("caller分類：上限不能是零"),
            Err(LimitError::TooLarge(value)) => println!("caller分類：上限{value}太大"),
            Err(other) => println!("caller保留未來錯誤分支：{other}"),
        }
    }
    let options = Options::default();
    match search("", "Rust", &options) {
        Err(SearchError::EmptyQuery) => println!("caller分類：請提供非空query"),
        Err(other) => println!("其他失敗：{other}"),
        Ok(hits) => println!("成功筆數={}", hits.len()),
    }

    let error = SearchError::Read(io::Error::from(io::ErrorKind::PermissionDenied));
    println!("這層context={error}");
    match error.source() {
        Some(cause) => println!("保存的原原因={cause}"),
        None => println!("沒有原原因"),
    }
    match &error {
        SearchError::Read(cause) => println!("typed原因種類={:?}", cause.kind()),
        other => println!("其他失敗：{other}"),
    }
}
