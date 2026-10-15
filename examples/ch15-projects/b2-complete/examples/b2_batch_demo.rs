use ch11_local_service::Fixture;
use url_b2::prepare_inputs;
use url_b2::run_sequential;
use url_b2::single::build_client;
use url_b2::single::default_limit;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let fixture = Fixture::start(0).await.expect("ready");
    let client = build_client(true).expect("本機client");
    let ok = fixture.url("/ok");
    let missing = fixture.url("/missing");
    let text = format!("\n{ok}\n/notes\n{ok}\n{missing}\n");
    let inputs = prepare_inputs(text.as_str()).expect("四份非空白行");
    let result = run_sequential(&client, inputs, default_limit()).await;
    let hits = fixture.shutdown().await.expect("server完成");
    assert_eq!(hits, 3);
    let entries = result.expect("非空批次");
    assert_eq!(entries.len(), 4);
    for entry in entries {
        println!(
            "index={} 原文={} 結果={:?}",
            entry.input_index,
            entry.check.original.as_str(),
            entry.check.outcome
        );
    }
    println!("四份結果，server已收尾；三份工作請求");
}
