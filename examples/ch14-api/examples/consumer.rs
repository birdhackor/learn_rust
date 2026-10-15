use ch14_api::BatchLimit;
use ch14_api::Options;
use ch14_api::search;

fn main() {
    let default = Options::default();
    println!(
        "預設：ASCII忽略大小寫={}，最多{}筆",
        default.ignore_case(),
        default.limit().get()
    );
    let limit = BatchLimit::new(2).expect("2在有效範圍內");
    let options = default.with_limit(limit).with_ignore_case(true);
    let hits = search(
        "RUST",
        "Rust第一行\nPython\nrust第三行\nRust第四行\n",
        &options,
    )
    .expect("query非空");
    for hit in hits.as_slice() {
        println!("{}:{}", hit.line_number, hit.text.as_str());
    }
    println!("借完設定，上限仍是={}", options.limit().get());
    #[cfg(feature = "text")]
    {
        let text = ch14_api::render_text(hits.as_slice());
        println!("text feature報告：\n{}", text.as_str());
    }
}
