use ch14_api::BatchLimit;
use ch14_api::Options;
use ch14_api::search;

fn main() {
    let limit = BatchLimit::new(1).expect("1在有效範圍內");
    let options = Options::default().with_limit(limit);
    let hits = search("Rust", "Python\nRust第一行\nRust第二行\n", &options).expect("query非空");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits.as_slice()[0].line_number, 2);
    assert_eq!(hits.as_slice()[0].text.as_str(), "Rust第一行");
    println!("上限1只保留原第{}行", hits.as_slice()[0].line_number);
}
