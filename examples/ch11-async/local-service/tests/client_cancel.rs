use ch11_http_single::build_client;
use ch11_http_single::inspect;
use ch11_http_single::parse_http_url;
use ch11_local_service::Fixture;
use ch11_local_service::finish_client;
use std::time::Duration;

#[tokio::test(flavor = "current_thread")]
async fn aborting_the_client_still_requires_server_shutdown() {
    let client = build_client(true, Duration::from_secs(10)).expect("本機 client");
    let mut fixture = Fixture::start(1).await.expect("本機服務已回 ready");
    let address = fixture.url("/held/0");
    let request = tokio::spawn(async move {
        let url = parse_http_url(address.as_str())?;
        inspect(&client, url, false).await
    });
    match fixture.next_started_within(Duration::from_secs(5)).await {
        Ok(Some(0)) => (),
        other => {
            let client_result = finish_client(request).await;
            let server_result = fixture.shutdown().await;
            panic!(
                "input 0 未確認進入：{other:?}；client：{client_result:?}；server：{server_result:?}"
            );
        }
    }
    let result = finish_client(request).await;
    let final_count = fixture.shutdown().await;
    match result {
        Err(error) => assert!(error.is_cancelled()),
        Ok(_) => panic!("尚未放行的 client task 應被取消"),
    }
    assert_eq!(final_count, Ok(1));
}
