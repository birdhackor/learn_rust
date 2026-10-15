use ch11_http_single::build_client;
use ch11_http_single::inspect;
use ch11_http_single::parse_http_url;
use ch11_local_service::Fixture;
use ch11_local_service::finish_client;
use std::time::Duration;

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
