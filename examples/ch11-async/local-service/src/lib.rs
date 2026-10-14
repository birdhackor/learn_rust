// --8<-- [start:imports]
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::response::Response;
use axum::routing::get;
use ch11_http_single::build_client;
use ch11_http_single::inspect;
use ch11_http_single::parse_http_url;
use std::collections::HashMap;
use std::future::IntoFuture;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::task::JoinError;
use tokio::task::JoinHandle;
use tokio::time::error::Elapsed;
use tokio::time::timeout;
// --8<-- [end:imports]

// --8<-- [start:shared-state]
struct FixtureState {
    hits: AtomicUsize,
    gates: Mutex<HashMap<usize, mpsc::Receiver<()>>>,
    entered: mpsc::Sender<usize>,
}
// --8<-- [end:shared-state]

// --8<-- [start:fixture-owner]
pub struct Fixture {
    address: SocketAddr,
    state: Arc<FixtureState>,
    releases: Vec<Option<mpsc::Sender<()>>>,
    entered: mpsc::Receiver<usize>,
    stop: mpsc::Sender<()>,
    server: JoinHandle<io::Result<()>>,
}
// --8<-- [end:fixture-owner]

// --8<-- [start:responses]
async fn ready() -> Response {
    (StatusCode::OK, "ready").into_response()
}

async fn ok(state: State<Arc<FixtureState>>) -> Response {
    let _old = state.0.hits.fetch_add(1, Ordering::SeqCst);
    (StatusCode::OK, "Rust 本機服務").into_response()
}

async fn missing(state: State<Arc<FixtureState>>) -> Response {
    let _old = state.0.hits.fetch_add(1, Ordering::SeqCst);
    (StatusCode::NOT_FOUND, "missing").into_response()
}

async fn error(state: State<Arc<FixtureState>>) -> Response {
    let _old = state.0.hits.fetch_add(1, Ordering::SeqCst);
    (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
}

async fn redirect(state: State<Arc<FixtureState>>) -> Response {
    let _old = state.0.hits.fetch_add(1, Ordering::SeqCst);
    Redirect::temporary("/ok").into_response()
}

async fn redirect_loop(state: State<Arc<FixtureState>>) -> Response {
    let _old = state.0.hits.fetch_add(1, Ordering::SeqCst);
    Redirect::temporary("/loop").into_response()
}
// --8<-- [end:responses]

// --8<-- [start:held-handler]
async fn held(state: State<Arc<FixtureState>>, path: Path<usize>) -> Response {
    let input = path.0;
    let _old = state.0.hits.fetch_add(1, Ordering::SeqCst);
    let gate = {
        let mut gates = state.0.gates.lock().expect("fixture 不在持鎖時 panic");
        gates.remove(&input)
    };
    let mut gate = match gate {
        Some(gate) => gate,
        None => return (StatusCode::NOT_FOUND, "沒有未使用的 gate").into_response(),
    };
    match state.0.entered.send(input).await {
        Ok(()) => (),
        Err(_) => return (StatusCode::SERVICE_UNAVAILABLE, "controller 已結束").into_response(),
    }
    match gate.recv().await {
        Some(()) => (StatusCode::OK, "held 已放行").into_response(),
        None => (StatusCode::SERVICE_UNAVAILABLE, "gate 已關閉").into_response(),
    }
}
// --8<-- [end:held-handler]

// --8<-- [start:router]
fn router(state: Arc<FixtureState>) -> Router {
    Router::new()
        .route("/ready", get(ready))
        .route("/ok", get(ok))
        .route("/missing", get(missing))
        .route("/error", get(error))
        .route("/redirect", get(redirect))
        .route("/loop", get(redirect_loop))
        .route("/held/{input}", get(held))
        .with_state(state)
}
// --8<-- [end:router]

impl Fixture {
    // --8<-- [start:start]
    pub async fn start(held_count: usize) -> Result<Self, String> {
        let client = match build_client(true, Duration::from_secs(10)) {
            Ok(client) => client,
            Err(error) => return Err(error.to_string()),
        };
        let listener = match TcpListener::bind("127.0.0.1:0").await {
            Ok(listener) => listener,
            Err(error) => return Err(format!("本機 listener 建立失敗：{error}")),
        };
        let address = match listener.local_addr() {
            Ok(address) => address,
            Err(error) => return Err(format!("本機位址讀取失敗：{error}")),
        };
        let ready_address = format!("http://{address}/ready");
        let ready_url = match parse_http_url(ready_address.as_str()) {
            Ok(url) => url,
            Err(error) => return Err(error.to_string()),
        };
        let capacity = if held_count == 0 { 1 } else { held_count };
        let (entered_tx, entered_rx) = mpsc::channel::<usize>(capacity);
        let mut gates = HashMap::new();
        let mut releases = Vec::new();
        for input in 0..held_count {
            let (sender, receiver) = mpsc::channel::<()>(1);
            gates.insert(input, receiver);
            releases.push(Some(sender));
        }
        let state = Arc::new(FixtureState {
            hits: AtomicUsize::new(0),
            gates: Mutex::new(gates),
            entered: entered_tx,
        });
        let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
        let service =
            axum::serve(listener, router(Arc::clone(&state))).with_graceful_shutdown(async move {
                let _event = stop_rx.recv().await;
            });
        let server = tokio::spawn(service.into_future());
        let fixture = Self {
            address,
            state,
            releases,
            entered: entered_rx,
            stop: stop_tx,
            server,
        };
        let result = inspect(&client, ready_url, true).await;
        let failure = match result {
            Ok(report) => {
                if report.status == 200 && report.body == Some("ready".to_owned()) {
                    return Ok(fixture);
                }
                format!("ready 沒有得到約定回應：{report:?}")
            }
            Err(error) => format!("ready 請求失敗：{error}"),
        };
        let cleanup = fixture.shutdown().await;
        Err(format!("{failure}；收尾結果：{cleanup:?}"))
    }
    // --8<-- [end:start]

    // --8<-- [start:controls]
    pub fn url(&self, path: &str) -> String {
        format!("http://{}{path}", self.address)
    }

    pub fn hits(&self) -> usize {
        self.state.hits.load(Ordering::SeqCst)
    }

    pub async fn next_started(&mut self) -> Option<usize> {
        self.entered.recv().await
    }

    pub async fn next_started_within(&mut self, limit: Duration) -> Result<Option<usize>, Elapsed> {
        timeout(limit, self.next_started()).await
    }

    pub async fn release(&mut self, input: usize) -> bool {
        if input >= self.releases.len() {
            return false;
        }
        let sender = self.releases.as_mut_slice()[input].take();
        match sender {
            Some(sender) => match sender.send(()).await {
                Ok(()) => true,
                Err(_) => false,
            },
            None => false,
        }
    }
    // --8<-- [end:controls]

    // --8<-- [start:shutdown]
    pub async fn shutdown(self) -> Result<usize, String> {
        // 關閉仍持有的 gate，讓已進入 handler 的等待可以結束。
        drop(self.releases);
        let notification = self.stop.send(()).await;
        let finished = self.server.await;
        match finished {
            Ok(Ok(())) => match notification {
                Ok(()) => Ok(self.state.hits.load(Ordering::SeqCst)),
                Err(_) => Err("server 已失去停止收端".to_owned()),
            },
            Ok(Err(error)) => Err(format!("server 工作失敗：{error}")),
            Err(error) => Err(format!("server task 失敗：{error}")),
        }
    }
    // --8<-- [end:shutdown]
}

// --8<-- [start:finish-client]
pub async fn finish_client<T>(client: JoinHandle<T>) -> Result<T, JoinError> {
    client.abort();
    client.await
}
// --8<-- [end:finish-client]
