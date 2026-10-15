use ch11_local_service::Fixture;
use url_b3::default_jobs;
use url_b3::input::prepare_inputs;
use url_b3::run_bounded;
use url_b3::single::build_client;
use url_b3::single::default_limit;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = build_client(true).expect("本機 client");
    let fixture = Fixture::start(0).await.expect("ready");
    let ok = fixture.url("/ok");
    let missing = fixture.url("/missing");
    let text = format!("\n{ok}\n/notes\n{ok}\n{missing}\n");
    let inputs = match prepare_inputs(text.as_str()) {
        Ok(inputs) => inputs,
        Err(error) => {
            let stopped = fixture.shutdown().await;
            panic!("材料應有四筆：{error:?}；收尾={stopped:?}");
        }
    };
    let result = run_bounded(&client, inputs, default_jobs(), default_limit()).await;
    let stopped = fixture.shutdown().await;
    assert_eq!(stopped, Ok(3));
    let entries = result.expect("非空且 jobs 為正");
    for entry in &entries {
        println!(
            "index={} original={:?} completion={:?}",
            entry.input_index,
            entry.original(),
            entry.completion
        );
    }
}
