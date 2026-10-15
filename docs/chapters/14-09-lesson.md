# 14.9 受控本機服務：讓 HTTP 測試自己安排條件

上一節用人工報告驗資料政策，還沒證明 client 真的送出 HTTP。現在讓測試同時持有一個本機 server：固定回200／404／500，也能明確通知「請求已進入」，等我們放行才回應。這樣才能分清收到請求、client 取得結果，以及 server 完全結束。

## handler 把資料變成 Response

本節選 **axum** 建立 HTTP 服務。**handler（處理函式）** 接到對應請求後執行，交出回應；先用不需要輸入的 async welcome。

**`axum::response::IntoResponse`** 是把資料轉成 axum Response 的 trait，method **`into_response(self)`** 接手資料、交回 Response。這次 `(StatusCode::OK, "Rust 本機服務")` 是狀態與文字 body 的 tuple；這個 tuple 已提供 IntoResponse 實作，不用手組HTTP bytes。StatusCode::OK是200，字串成為文字內容。

```rust
--8<-- "examples/ch11-async/local-service/examples/basic.rs:handler"
```

**`Router`** 保存路徑對應。`Router::new()` 建空路由表，`.route("/ok", get(welcome))` 接手並交回 router；`axum::routing::get` 把 GET 請求交給 welcome。此處傳函式本身，不寫 welcome() 先建立一次工作。這些設定尚未開始接收網路。

## 持有同一份 listener，讓 OS 選 port

**loopback（回送）** 地址127.0.0.1讓這台機器連回自己，不向外部公開服務。**TCP** 是 Transmission Control Protocol（傳輸控制協定），提供傳輸 bytes 的連線；HTTP/1在本例使用它。**port（通訊埠）** 讓一台機器區分不同網路入口。

**`tokio::net::TcpListener::bind("127.0.0.1:0").await`** 要求綁定本機入口，0代表由OS選可用port，回傳 `io::Result<TcpListener>`。**`listener.local_addr()`** shared 借用listener，回傳 `io::Result<std::net::SocketAddr>`，可用Display組出這次的URL。

這份 listener 一直由程式持有，再移交 server。不是先找一個空port、drop listener、之後才重新bind；後者之間可能被別份工作占用。兩份同時存活的fixture也能各自持有OS選出的不同入口。

## IntoFuture：可 await 的值，如何交給 spawn？

**`axum::serve(listener, router)`** 接手兩者，建立服務設定；再用 **`.with_graceful_shutdown(stop_work)`** 接受一份 Output為unit的停止工作。停止工作完成時，server不再接受新連線，讓正在處理的請求完成，再結束；若handler仍一直等gate，就需要我們先處理那份等待。

這份serve設定可以await，卻**本身沒有直接實作Future**。這裡首次使用標準trait **`std::future::IntoFuture`**：它描述一個值如何轉成可執行的future，和第9.4節IntoIterator先轉成iterator再走訪的分工相近。

| IntoFuture 的項目 | 本次角色 |
| --- | --- |
| associated type Output | 轉出的future最後交出什麼；本例是 `io::Result<()>` |
| associated type IntoFuture | 真正實作Future的工作型別，Output要與前項相同 |
| `into_future(self)` | 接手設定，轉出那份工作 |

`.await` 會先做這項轉換。一般Future也有IntoFuture能力，所以14.2的await用法照常成立。**tokio::spawn要求的則是Future**，不替serve設定做這層轉換，因此明寫 **`tokio::spawn(service.into_future())`**；`use std::future::IntoFuture;` 讓method可用。這裡只使用已提供的實作，不需要先學服務內部怎樣保存future。

server handle的await又有兩層：外層 `Result<_, JoinError>` 是task，內層 `io::Result<()>` 是服務工作；不能看到外層Ok就略過內層。

## 最小完整程式：真正回200，再停止服務

早期 **`ch11-local-basic`** 是no-lib package，沒有Fixture或測試helper。完整manifest使用axum的http1／tokio，Tokio的rt／macros／net／time／sync，以及14.8已完成的http-single：

```toml
--8<-- "examples/ch11-async/stages/local-basic/Cargo.toml"
```

完整 **`src/main.rs`** 用channel提供停止工作，spawn服務後由client真正GET /ok。無論檢查結果成功與否，都先送stop、await server，再expect或assert：

```rust
--8<-- "examples/ch11-async/stages/local-basic/src/main.rs"
```

從repo根目錄執行：

```bash
cargo run -p ch11-local-basic --locked
```

兩行stdout確認真HTTP200／body，以及已等待server完成。這個程式有明確結尾，不是啟動後要靠外部殺掉的常駐服務。

## Fixture 自己持有控制與回收責任

接著把服務包成 **Fixture（測試專用環境）**。這份自訂struct擁有：address、shared state、每個gate的放行送端、entered收端、stop送端，以及server JoinHandle。caller取得Fixture，就也取得最後shutdown的責任。

**FixtureState** 由第13.6節的 `Arc<FixtureState>` 共同持有。AtomicUsize hits用第13.8節已教的fetch_add／load與SeqCst；它只計工作handler進入次數，不能代替join完成證據。gates是Mutex保護的 `HashMap<usize, mpsc::Receiver<()>>`，entered送端則回報哪個held請求已進入。

```rust
--8<-- "examples/ch11-async/local-service/src/lib.rs:shared-state"
```

axum的 **`State<T>`** 是取出route所提供shared state的 **extractor（資料擷取器）**；本例handler接收 `State<Arc<FixtureState>>`，`.0`取得裡面的Arc，沿用第11.10節tuple struct取欄位方式。Router的 **`.with_state(state)`** 提供這份state，後續handler取得的Arc共同指向同一資料，不複製整個FixtureState。

**`Path<usize>`** 則把路徑參數取成usize；`/held/{input}`的 `{input}` 對應 `path.0`。路徑無法解成要求的型別時，framework不會把一個無效usize交給handler。

## 固定回應與有gate的handler

本節路徑的規格如下，body是固定文字；ready探測不列入工作hits：

| 路徑 | 回應與計數 |
| --- | --- |
| /ready | 200、ready；不加hits |
| /ok、/missing、/error | 分別200／404／500；各加1 |
| /redirect | 暫時導向/ok；本身與後續/ok各加1 |
| /loop | 暫時導向自己；每次進入加1，最後由client的redirect上限拒絕 |
| /held/{input} | 取獨立gate，送entered，再等release；每次進入先加1 |

**`Redirect::temporary(path)`** 建立暫時重新導向的回應資料，本版轉成307回應，交給client依位置繼續。固定handlers的完整片段：

```rust
--8<-- "examples/ch11-async/local-service/src/lib.rs:responses"
```

held先在短guard範圍內 **remove gate**，把owned接收端取出，離開block便解鎖。之後await entered.send與gate.recv時沒有持有MutexGuard，沿用14.6的Send邊界。gate不存在回404；controller收端已結束回503；release送Some(())回200；所有放行送端結束、recv為None則回503與「gate 已關閉」。

```rust
--8<-- "examples/ch11-async/local-service/src/lib.rs:held-handler"
```

因此entered代表handler已取出gate、正要等待放行；不能把它誤認為client已收到headers。router完整設定：

```rust
--8<-- "examples/ch11-async/local-service/src/lib.rs:router"
```

## ready、entered、release 各負責一件事

**Fixture::start(held_count)** 準備指定數量的一次性gate及entered channel，再用同一份listener啟動服務。最後真的HTTP GET /ready，確認200與body=ready才交回Fixture；若ready檢查失敗，先shutdown已擁有的server，再回錯誤。held_count=0時仍使用容量1的通知channel，不建立非法零容量。

這只證明那次ready HTTP完成，不能保證下一個held請求一定會來。因此main與測試用 **`next_started_within(limit)`**：回傳 `Result<Option<usize>, Elapsed>`，Some帶input，None是來源結束，Err是觀察期限到。內部next_started直接recv，由外層timeout限制；等待被丟棄不會因此取消server handler。

**`release(input)`** mutable借用Fixture，只允許每個送端使用一次。這裡新增 **`Option::take()`**：mutable操作Option，用None取代原值，再把原本的Some或None交回。從Vec位置取出的因此是owned Sender，可在await前結束對該位置的借用；不保留另一份送端使gate無法關閉。越界、重用或收端已結束，都回false。

```rust
--8<-- "examples/ch11-async/local-service/src/lib.rs:controls"
```

## shutdown 先關gate，再通知及等待server

**`fixture.shutdown()`** 接手整份Fixture，依序drop尚持有的放行送端、送停止訊息、await server，最後回傳hits或錯誤。已進入held的handler因此可收到None、回503，graceful shutdown才有辦法收妥它；只通知server、卻留下永不放行的gate，不能保證能結束。

**finish_client(handle)** 則只處理client task：abort後await，交回實際結果。已完成的client仍可能保留正常Output；尚未完成才取消。它不持有Fixture，不能替caller關閉server或回復hits。

```rust
--8<-- "examples/ch11-async/local-service/src/lib.rs:shutdown"
```

```rust
--8<-- "examples/ch11-async/local-service/src/lib.rs:finish-client"
```

## 完整Fixture、有限main與失敗收尾

完成版 **`ch11-local-service`** 的完整manifest此時才帶入測試用test-util，供後段pause／resume。以下完整library包含剛才全部handler、router、Fixture methods與helper，沒有另一份隱藏test support：

```toml
--8<-- "examples/ch11-async/local-service/Cargo.toml"
```

```rust
--8<-- "examples/ch11-async/local-service/src/lib.rs"
```

Client 支援 Send／Sync，owned client 能移進 spawn 工作，對應第14.3節的 Send 要求。

完整 **`src/main.rs`** 先spawn client，等entered，再放行並取得結果；每個失敗分支都先收client、shutdown server，才panic。成功路徑也先保存server結果，再assert，避免assert提前離開使資源責任不明：

```rust
--8<-- "examples/ch11-async/local-service/src/main.rs"
```

```bash
cargo run -p ch11-local-service --locked
```

stdout為：

```text
服務已收到 input 0；尚未交回 headers
放行後：HTTP 200，body=Some("held 已放行")
已等待 server；共處理 1 份工作請求
```

第一行是最後回報已建立的進入證據，並非程式正在那一行等候輸入。這個binary自己收妥client與server，CLI test才能用Command::output等有限結束。

本package也保留最小/ok選例，現在完整Fixture已教，才從這個完成package編譯它。完整 **`examples/basic.rs`**：

```rust
--8<-- "examples/ch11-async/local-service/examples/basic.rs"
```

## 固定回應：驗真實HTTP與最終URL

先提供完整 **`tests/fixed_responses.rs`**，內有三項測試：200／404／500與body、單次redirect的final URL、redirect loop拒絕，以及兩份同時存活fixture使用不同入口。

新用的 **`reqwest::Error::is_redirect()`** 回bool，辨認redirect政策失敗；/loop沒有得到最終Inspection，應是AppError::Request。最多跟隨5次redirect，加上首次請求共6次handler進入，因此hits=6。一般三筆加一次redirect的兩次工作請求，則共5；ready不計。

第二份fixture啟動失敗的分支也會先shutdown第一份。所有報告或錯誤保存到server收尾後才斷言：

```rust
--8<-- "examples/ch11-async/local-service/tests/fixed_responses.rs"
```

```bash
cargo test -p ch11-local-service --test fixed_responses --locked
```

## 獨立gate指定完成順序

這次提交/held/0與/held/1。**`client.clone()`** 交出另一個Client值，共用內部連線池與設定，不是重新建另一份獨立pool。

先以有限期限收到兩份entered，順序可0／1或1／0；兩份client都尚未完成。只放行1並收回，0仍在等待，再放行0。這證明完成順序來自獨立gate控制，不靠固定sleep。

完整 **`tests/gates.rs`** 包含兩個client把手的所有失敗收尾，也確認gate不能重用與索引越界回false：

```rust
--8<-- "examples/ch11-async/local-service/tests/gates.rs"
```

```bash
cargo test -p ch11-local-service --test gates --locked
```

## 先完成真網路前置，再暫停timer時鐘

14.5從測試入口start_paused；現在需要先讓真正網路完成ready與handler entered，不能一開始就把它們當成虛擬timer。

**`tokio::time::pause()`** 在current_thread測試中暫停Tokio時鐘；**`resume()`** 恢復它跟隨真實時間。兩者不帶參數，回unit；pause要求當時尚未暫停，resume要求已暫停。先收到held entered，確定尚無headers，再pause、等待client自己的10秒總期限；得到結果後先resume，最後shutdown，再斷言。

完整 **`tests/request_timeout.rs`**：

```rust
--8<-- "examples/ch11-async/local-service/tests/request_timeout.rs"
```

```bash
cargo test -p ch11-local-service --test request_timeout --locked
```

它驗的是headers還被gate扣住時取得Request逾時，沒有驗收到headers後的Body逾時，也沒有用std elapsed_ms當成虛擬10秒的證據。

## 沒有entered，也要有觀察出口

若URL誤指向/loop，它不會送held的entered。Fixture仍持有通知送端，直接recv可能一直等。先讓client算出實際Result並送finished通知，給這份通知15秒真實期限；確認後才pause，用5秒Tokio期限觀察「沒有entered」。

接著resume、收client、shutdown server，最後檢查原本的Request redirect錯誤與hits=6。不要只因entered逾時，就丟掉真正請求已失敗的原因。完整 **`tests/absent_entered.rs`**：

```rust
--8<-- "examples/ch11-async/local-service/tests/absent_entered.rs"
```

```bash
cargo test -p ch11-local-service --test absent_entered --locked
```

finish_client對這份已完成工作仍可取得Ok(Err(Request(...)))；abort請求不會必然把既有結果改成Cancelled。

## client取消後，server仍由Fixture收尾

完整 **`tests/client_cancel.rs`** 先確認entered，再abort並await client，檢查is_cancelled。handler已經進入、hits已加1；client取消不會替我們回復這項修改，也沒有拿走Fixture的server把手。仍須shutdown並await server：

```rust
--8<-- "examples/ch11-async/local-service/tests/client_cancel.rs"
```

```bash
cargo test -p ch11-local-service --test client_cancel --locked
```

## 練習與完整答案：不放行，直接關gate

沿用input0與entered，這次不呼叫release，而把整份Fixture交給shutdown task。同時收回原client與shutdown兩份把手。預測：是HTTP200、Request錯誤，還是正常收到另一個status？哪個動作讓handler能離開gate.recv？

完整 **`examples/closed_gate_solution.rs`**：

```rust
--8<-- "examples/ch11-async/local-service/examples/closed_gate_solution.rs"
```

```bash
cargo run -p ch11-local-service --example closed_gate_solution --locked
```

shutdown先drop放行送端，handler收到None並回HTTP503、body「gate 已關閉」，所以client正常取得Inspection、passed=false；不是完全沒有Response的Request錯誤。兩份把手都已收回、hits=1。

完整 **`tests/shutdown_gate.rs`** 保存這項契約；完整 **`tests/cli.rs`** 驗有限main的三行精確stdout與正常退出：

```rust
--8<-- "examples/ch11-async/local-service/tests/shutdown_gate.rs"
```

```rust
--8<-- "examples/ch11-async/local-service/tests/cli.rs"
```

```bash
cargo test -p ch11-local-service --test shutdown_gate --locked
cargo test -p ch11-local-service --test cli --locked
```

測試依用途分成不同檔案；每次明選一個test target。名字filter只篩執行，不能讓同一檔的其餘程式消失。現在全部source與答案都已提供，再檢查所有target：

```bash
cargo test -p ch11-local-basic -p ch11-local-service --all-targets --locked
cargo clippy -p ch11-local-basic -p ch11-local-service --all-targets --locked -- -D warnings
```

本章從保存工作開始，現在能用真HTTP的進入、放行、期限與join分開證明進度。下一章處理另一組有效性問題：借用結果的型別、可替換範圍，以及需要額外契約才能使用的底層介面。

[下一章：進階 lifetime、unsafe 與 FFI](15-01-lesson.md)
