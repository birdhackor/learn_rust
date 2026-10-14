use ch11_http_single::build_client;
use ch11_http_single::inspect;
use ch11_http_single::parse_http_url;
use ch11_local_service::Fixture;
use ch11_local_service::finish_client;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = build_client(true, Duration::from_secs(10)).expect("能準備本機 client");
    let mut fixture = Fixture::start(1).await.expect("能建立並驗收本機服務");
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
    let released = fixture.release(0).await;
    let client_result = if released {
        request.await
    } else {
        finish_client(request).await
    };
    let server_result = fixture.shutdown().await;
    if !released {
        panic!("gate 未放行；client：{client_result:?}；server：{server_result:?}");
    }
    let report = client_result
        .expect("client task 正常完成")
        .expect("放行後得到回應");
    let hits = server_result.expect("server 已收尾");
    assert_eq!(report.status, 200);
    assert_eq!(report.body, Some("held 已放行".to_owned()));
    assert_eq!(hits, 1);
    println!("服務已收到 input 0；尚未交回 headers");
    println!("放行後：HTTP {}，body={:?}", report.status, report.body);
    println!("已等待 server；共處理 {hits} 份工作請求");
}
