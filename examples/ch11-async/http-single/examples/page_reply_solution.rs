use ch11_http_single::Inspection;
use ch11_http_single::reply::decode_page;

fn main() {
    // 人工報告只供格式／模型練習，不代表本程式送過 HTTP。
    let inspection = Inspection {
        final_url: "https://example.org/notes".to_owned(),
        status: 200,
        passed: true,
        body: Some("{\"title\":\"筆記\",\"revision\":3}".to_owned()),
        elapsed_ms: 0,
    };
    let page = decode_page(&inspection).expect("人工 body 符合 PageRecord");
    assert_eq!(page.title.as_str(), "筆記");
    assert_eq!(page.revision, 3);
    println!("{}：revision={}", page.title.as_str(), page.revision);
}
