use ch11_http_single::build_client;
use ch11_http_single::inspect;
use ch11_http_single::parse_http_url;
use ch11_local_service::Fixture;
use ch11_local_service::finish_client;
use std::time::Duration;

#[tokio::test(flavor = "current_thread")]
async fn shutdown_closes_unreleased_gates_and_waits_for_the_response() {
    let client = build_client(true, Duration::from_secs(10)).expect("本機 client");
    let mut fixture = Fixture::start(1).await.expect("本機服務已回 ready");
    let address = fixture.url("/held/0");
    let request = tokio::spawn(async move {
        let url = parse_http_url(address.as_str())?;
        inspect(&client, url, true).await
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
    let closing = tokio::spawn(fixture.shutdown());
    let client_result = request.await;
    let server_result = closing.await;
    let report = client_result
        .expect("client task 正常")
        .expect("graceful 回應");
    let final_count = server_result.expect("shutdown task 正常");
    assert_eq!(report.status, 503);
    assert!(!report.passed);
    assert_eq!(report.body, Some("gate 已關閉".to_owned()));
    assert_eq!(final_count, Ok(1));
}
