use ch11_local_service::Fixture;
use url_b1::Outcome;
use url_b1::build_client;
use url_b1::check;
use url_b1::default_limit;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let fixture = Fixture::start(0).await.expect("真本機服務已回 ready");
    let client = build_client(true).expect("能建立本機 client");
    let original = fixture.url("/redirect");
    let record = check(&client, original, default_limit()).await;
    let final_hits = fixture.shutdown().await;
    assert_eq!(final_hits, Ok(2));
    println!("原輸入：{}", record.original.as_str());
    match &record.outcome {
        Outcome::Http {
            final_url,
            status,
            passed,
        } => {
            assert_eq!(*status, 200);
            assert!(*passed);
            println!("最後 URL：{final_url}");
            println!("HTTP {status}，passed={passed}");
        }
        other => panic!("固定 redirect 應得到HTTP：{other:?}"),
    }
    println!("本次request耗時：{:?} ms", record.elapsed_ms);
    println!("server 已收尾，處理2份工作請求");
}
