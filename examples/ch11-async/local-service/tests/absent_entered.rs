use ch11_http_single::build_client;
use ch11_http_single::error::AppError;
use ch11_http_single::inspect;
use ch11_http_single::parse_http_url;
use ch11_local_service::Fixture;
use ch11_local_service::finish_client;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;

#[tokio::test(flavor = "current_thread")]
async fn absent_entered_has_an_exit_and_keeps_the_request_error() {
    let client = build_client(true, Duration::from_secs(10)).expect("本機 client");
    let mut fixture = Fixture::start(1).await.expect("本機服務已回 ready");
    let address = fixture.url("/loop");
    let (finished_tx, mut finished_rx) = mpsc::channel::<()>(1);
    let request = tokio::spawn(async move {
        let result = match parse_http_url(address.as_str()) {
            Ok(url) => inspect(&client, url, false).await,
            Err(error) => Err(error),
        };
        let _notification = finished_tx.send(()).await;
        result
    });
    match timeout(Duration::from_secs(15), finished_rx.recv()).await {
        Ok(Some(())) => (),
        other => {
            let client_result = finish_client(request).await;
            let server_result = fixture.shutdown().await;
            panic!(
                "未收到 client 結果通知：{other:?}；client：{client_result:?}；server：{server_result:?}"
            );
        }
    }
    tokio::time::pause();
    let observed = fixture.next_started_within(Duration::from_secs(5)).await;
    tokio::time::resume();
    let client_result = finish_client(request).await;
    let server_result = fixture.shutdown().await;
    match observed {
        Err(_) => (),
        other => panic!("/loop 沒有 held entered，應由觀察期限結束：{other:?}"),
    }
    match client_result {
        Ok(Err(AppError::Request(error))) => {
            assert!(error.is_redirect());
            println!("已收回原 Request 錯誤：{error}");
        }
        other => panic!("應保留已完成 client 的 redirect 錯誤：{other:?}"),
    }
    assert_eq!(server_result, Ok(6));
    println!("entered 已由期限結束；client 已收回；server 已收尾，共 6 份工作請求");
}
