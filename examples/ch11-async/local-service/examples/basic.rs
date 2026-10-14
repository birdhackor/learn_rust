use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use ch11_http_single::build_client;
use ch11_http_single::inspect;
use ch11_http_single::parse_http_url;
use std::future::IntoFuture;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

// --8<-- [start:handler]
async fn welcome() -> Response {
    (StatusCode::OK, "Rust 本機服務").into_response()
}
// --8<-- [end:handler]

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = build_client(true, Duration::from_secs(10)).expect("能準備 client");
    // --8<-- [start:route]
    let router: Router = Router::new().route("/ok", get(welcome));
    // --8<-- [end:route]
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("本機可 bind");
    let address = listener.local_addr().expect("已綁定 listener 有本機位址");
    let address = format!("http://{address}/ok");
    let url = parse_http_url(address.as_str()).expect("固定本機 HTTP URL");
    let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
    let service = axum::serve(listener, router).with_graceful_shutdown(async move {
        let _event = stop_rx.recv().await;
    });
    let server = tokio::spawn(service.into_future());
    let result = inspect(&client, url, true).await;
    let notification = stop_tx.send(()).await;
    let finished = server.await;
    let report = result.expect("本機服務能回應");
    assert_eq!(report.status, 200);
    assert_eq!(report.body, Some("Rust 本機服務".to_owned()));
    notification.expect("server 仍等待停止");
    match finished {
        Ok(Ok(())) => (),
        Ok(Err(error)) => panic!("server 工作失敗：{error}"),
        Err(error) => panic!("server task 失敗：{error}"),
    }
    println!("真 HTTP 回應：200，Rust 本機服務");
    println!("已通知停止，並等待 server 完成");
}
