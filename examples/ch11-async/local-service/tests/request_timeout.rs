use ch11_http_single::build_client;
use ch11_http_single::error::AppError;
use ch11_http_single::inspect;
use ch11_http_single::parse_http_url;
use ch11_local_service::Fixture;
use ch11_local_service::finish_client;
use std::time::Duration;

#[tokio::test(flavor = "current_thread")]
async fn a_request_reaches_its_deadline_while_headers_are_held() {
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
    let waiting = !request.is_finished();
    tokio::time::pause();
    let result = request.await;
    tokio::time::resume();
    let final_count = fixture.shutdown().await;
    assert!(waiting);
    match result {
        Ok(Err(AppError::Request(error))) => assert!(error.is_timeout()),
        other => panic!("gate 未放行，尚未得到 Response 就應逾時：{other:?}"),
    }
    assert_eq!(final_count, Ok(1));
}
