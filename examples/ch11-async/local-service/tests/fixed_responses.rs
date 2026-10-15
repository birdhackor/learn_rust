use ch11_http_single::build_client;
use ch11_http_single::error::AppError;
use ch11_http_single::inspect;
use ch11_http_single::parse_http_url;
use ch11_local_service::Fixture;
use std::time::Duration;

#[tokio::test(flavor = "current_thread")]
async fn real_http_keeps_status_body_and_final_redirect_url() {
    let client = build_client(true, Duration::from_secs(10)).expect("本機 client");
    let fixture = Fixture::start(0).await.expect("本機服務已回 ready");
    let mut observations = Vec::new();
    for spec in [
        ("/ok", 200, true, "Rust 本機服務"),
        ("/missing", 404, false, "missing"),
        ("/error", 500, false, "error"),
    ] {
        let address = fixture.url(spec.0);
        let result = match parse_http_url(address.as_str()) {
            Ok(url) => inspect(&client, url, true).await,
            Err(error) => Err(error),
        };
        observations.push((spec, address, result));
    }
    let address = fixture.url("/redirect");
    let result = match parse_http_url(address.as_str()) {
        Ok(url) => inspect(&client, url, false).await,
        Err(error) => Err(error),
    };
    let expected_final_url = fixture.url("/ok");
    let hits = fixture.hits();
    let final_count = fixture.shutdown().await;
    for (spec, address, result) in observations {
        let report = result.expect("有真 HTTP 回應");
        assert_eq!(report.final_url, address);
        assert_eq!(report.status, spec.1);
        assert_eq!(report.passed, spec.2);
        assert_eq!(report.body, Some(spec.3.to_owned()));
    }
    let report = result.expect("redirect 後有回應");
    assert_eq!(report.final_url, expected_final_url);
    assert_eq!(report.status, 200);
    assert!(report.passed);
    assert_eq!(report.body, None);
    assert_eq!(hits, 5);
    assert_eq!(final_count, Ok(5));
}
#[tokio::test(flavor = "current_thread")]
async fn endless_redirect_is_a_request_error_instead_of_http_failure() {
    let client = build_client(true, Duration::from_secs(10)).expect("本機 client");
    let fixture = Fixture::start(0).await.expect("本機服務已回 ready");
    let address = fixture.url("/loop");
    let result = match parse_http_url(address.as_str()) {
        Ok(url) => inspect(&client, url, false).await,
        Err(error) => Err(error),
    };
    let final_count = fixture.shutdown().await;
    match result {
        Err(AppError::Request(error)) => assert!(error.is_redirect()),
        other => panic!("redirect 政策應拒絕：{other:?}"),
    }
    assert_eq!(final_count, Ok(6));
}
#[tokio::test(flavor = "current_thread")]
async fn live_fixtures_keep_distinct_os_selected_ports() {
    let first = Fixture::start(0).await.expect("第一份服務已回 ready");
    let second = match Fixture::start(0).await {
        Ok(second) => second,
        Err(error) => {
            let first_result = first.shutdown().await;
            panic!("第二份服務啟動失敗：{error}；第一份收尾：{first_result:?}");
        }
    };
    let distinct = first.url("/ready") != second.url("/ready");
    let first_result = first.shutdown().await;
    let second_result = second.shutdown().await;
    assert!(distinct);
    assert_eq!(first_result, Ok(0));
    assert_eq!(second_result, Ok(0));
}
