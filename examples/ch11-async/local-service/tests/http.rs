use ch11_http_single::build_client;
use ch11_http_single::error::AppError;
use ch11_http_single::inspect;
use ch11_http_single::parse_http_url;
use ch11_local_service::Fixture;
use ch11_local_service::finish_client;
use std::time::Duration;

// --8<-- [start:fixed-responses-test]
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
// --8<-- [end:fixed-responses-test]

// --8<-- [start:redirect-limit-test]
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
// --8<-- [end:redirect-limit-test]

// --8<-- [start:gates-test]
#[tokio::test(flavor = "current_thread")]
async fn separate_gates_choose_completion_without_fixed_sleep() {
    let client = build_client(true, Duration::from_secs(10)).expect("本機 client");
    let mut fixture = Fixture::start(2).await.expect("本機服務已回 ready");
    let first_address = fixture.url("/held/0");
    let first_client = client.clone();
    let first = tokio::spawn(async move {
        let url = parse_http_url(first_address.as_str())?;
        inspect(&first_client, url, true).await
    });
    let second_address = fixture.url("/held/1");
    let second = tokio::spawn(async move {
        let url = parse_http_url(second_address.as_str())?;
        inspect(&client, url, true).await
    });
    let first_seen = match fixture.next_started_within(Duration::from_secs(5)).await {
        Ok(Some(input)) => input,
        other => {
            let first_result = finish_client(first).await;
            let second_result = finish_client(second).await;
            let server_result = fixture.shutdown().await;
            panic!(
                "第一份 entered 未確認：{other:?}；client 0：{first_result:?}；client 1：{second_result:?}；server：{server_result:?}"
            );
        }
    };
    let second_seen = match fixture.next_started_within(Duration::from_secs(5)).await {
        Ok(Some(input)) => input,
        other => {
            let first_result = finish_client(first).await;
            let second_result = finish_client(second).await;
            let server_result = fixture.shutdown().await;
            panic!(
                "第二份 entered 未確認：{other:?}；client 0：{first_result:?}；client 1：{second_result:?}；server：{server_result:?}"
            );
        }
    };
    let hits = fixture.hits();
    let first_waiting = !first.is_finished();
    let second_waiting = !second.is_finished();
    let released_second = fixture.release(1).await;
    let second_result = if released_second {
        second.await
    } else {
        finish_client(second).await
    };
    let first_still_waiting = !first.is_finished();
    let released_first = fixture.release(0).await;
    let first_result = if released_first {
        first.await
    } else {
        finish_client(first).await
    };
    let reused = fixture.release(0).await;
    let outside = fixture.release(2).await;
    let final_count = fixture.shutdown().await;
    assert!((first_seen == 0 && second_seen == 1) || (first_seen == 1 && second_seen == 0));
    assert_eq!(hits, 2);
    assert!(first_waiting && second_waiting && first_still_waiting);
    if !(released_first && released_second) {
        panic!("client 0：{first_result:?}；client 1：{second_result:?}");
    }
    let second_report = second_result
        .expect("client task 正常")
        .expect("回應已放行");
    let first_report = first_result.expect("client task 正常").expect("回應已放行");
    assert_eq!(second_report.status, 200);
    assert_eq!(second_report.body, Some("held 已放行".to_owned()));
    assert_eq!(first_report.status, 200);
    assert!(!reused && !outside);
    assert_eq!(final_count, Ok(2));
}
// --8<-- [end:gates-test]

// --8<-- [start:timeout-test]
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
// --8<-- [end:timeout-test]

// --8<-- [start:cancel-test]
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
// --8<-- [end:cancel-test]

// --8<-- [start:shutdown-test]
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
// --8<-- [end:shutdown-test]

// --8<-- [start:dynamic-port-test]
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
// --8<-- [end:dynamic-port-test]
