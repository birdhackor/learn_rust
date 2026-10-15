# 14.8 單一 async HTTP：等待回應，再取得 owned JSON 資料

上一節已分清 Response、HTTP通過與 body。本節保留相同規格，改用 async client：由一份工作依序等待回應、視選項等待 body，再輸出。這還不是同時檢查多個 URL 的工具。

## 保留 client，讓 inspect 只處理一次檢查

這次使用 **`reqwest::Client`**，不是 blocking module 的 Client。它保存連線設定與可重用的 **connection pool（連線池）**：同一 client 的多次請求有機會重用已建立的連線，不必每筆都重新建 client。這是資源重用的能力，不保證每次都能重用，也不是本節量出的效能結果。

把 builder 工作獨立成普通 **build_client(direct, limit)**；caller 擁有 Client，再用 shared reference 借給 async inspect。inspect 接手這次的 Url，read_body 是 bool，完成後交回 owned Inspection。這項分工讓後面的本地測試能重用同一 client 與明確的期限。

早期 `ch11-http-single-basic` 的完整 manifest：Tokio 仍用 rt／macros，增加本例連線所需 **`net`** 與已教的 time；尚未加入 JSON model：

```toml
--8<-- "examples/ch11-async/stages/http-single-basic/Cargo.toml"
```

## send 與 text 各取得哪一層結果？

普通 `client.get(url)` 建立 RequestBuilder。**`.send()`** 交回一份 Output 為 `Result<Response, reqwest::Error>` 的 future；`.send().await` 才取得這份結果。**`response.text()`** 接手 Response，交回一份 Output 為 `Result<String, reqwest::Error>` 的 future，再 await 取得完整文字。

固定 reqwest 0.13.5 的 send 是**普通 future 工廠**，呼叫時便做請求與期限的準備，隨即 await。沿用14.2的區分，不能因為它交回 future，就把呼叫時點說成完全不做事。

先讀 status、保存 final URL，再把 Response 交給 text，ownership 與上一節一樣。`StatusCode` 可 Copy；`response.url().as_str().to_owned()` 則保存一份獨立 String，不把仍借用 Response 的 reference 留到 text 之後。

這份 async Client 的 **`.timeout(limit)` 是同一請求的總期限**：涵蓋連線到完整 body，redirect 或拿到 headers 不會重新給一次預算。若收到 headers 前逾時是 Request；已得到 status、讀 body 才失敗則是 Body。這與上一節 blocking send／text 各自等待的介面契約不同。

## elapsed_ms 從哪裡開始量？

本次 Inspection 增加 **`elapsed_ms: u128`**，保存非負的總毫秒數。它用14.5的 std Instant，從準備 GET 前量到建立報告前；若要求 body，包含等待它的時間。不含 parse_http_url、build_client 或之後的 writer 輸出，也不是 Tokio 的虛擬測試時鐘。

```rust
--8<-- "examples/ch11-async/http-single/src/lib.rs:models"
```

這個數字是當次觀察，不能預先寫死。後面的離線測試會人工給12毫秒驗格式；人工值不代表它真的發過 HTTP。

## 完整基本版：三項 core tests、兩項 CLI tests

設定與階段錯誤仍沿用上一節政策。以下是本版實際載入的全部 module，先給完整 `src/config.rs`、`src/error.rs`：

```rust
--8<-- "examples/ch11-async/stages/http-single-basic/src/config.rs"
```

```rust
--8<-- "examples/ch11-async/stages/http-single-basic/src/error.rs"
```

完整 `src/lib.rs` 包含 parse_http_url、build_client、inspect、render、run 與全部三項 core tests。run 先解析與建 client，再 await inspect；render 仍以同步 Write 輸出並 flush。這份小報告沒有把 writer 改成另一套 async I/O：

```rust
--8<-- "examples/ch11-async/stages/http-single-basic/src/lib.rs"
```

完整 `src/main.rs` 使用 current_thread 入口，讓連線及 timer 的等待在正確 runtime 內執行，並保留 clap／退出政策。取得設定後才呼叫 run.await：

```rust
--8<-- "examples/ch11-async/stages/http-single-basic/src/main.rs"
```

從 repo 根目錄執行：

```bash
cargo run -p ch11-http-single-basic --locked -- --help
cargo test -p ch11-http-single-basic --lib --locked
```

三項測試驗輸入先拒絕、人工報告保留 status／time／None 與 Some(空body)、flush 失敗。這些結果沒有驗證真實網路。

完整 `tests/cli.rs` 保存 execute helper，實際啟動這個 package 的 binary，驗 help／version及無效參數／URL：

```rust
--8<-- "examples/ch11-async/stages/http-single-basic/tests/cli.rs"
```

```bash
cargo test -p ch11-http-single-basic --test cli --locked
```

保留與上一節相同的選用觀察方式，這次執行 async binary：

```bash
cargo run -p ch11-http-single-basic --locked -- https://example.org
cargo run -p ch11-http-single-basic --locked -- https://example.org --body
```

網路條件與回應會變，elapsed 是這次觀察值。text 同樣依本例 features 採 UTF-8 替代解碼，且未限制完整 body 大小；async 等待沒有改變這份內容政策。

## HTTP成功之後，JSON仍要符合模型

接著增加不同的資料需求：body 應是 `{"title":"筆記","revision":3}`，要取得能獨立保存的 PageRecord。第10.7節已教 Serde／Deserialize／from_str，現在把它們接到已取得的 HTTP 報告；不因 HTTP200就假定 revision 必定是 i32。

**PageRecord** 的 title 是 owned String，revision 是 i32。`#[derive(Deserialize)]` 依欄位解碼，沿用 `#[serde(deny_unknown_fields)]` 拒絕多出的欄位；缺欄位、JSON語法錯誤、型別不合也會失敗。文字 `"3"` 不會自動當成數字3。

**ReplyError** 保留三個階段：

| 分類 | 判斷時點 |
| --- | --- |
| Http(status) | Inspection未通過，先拒絕，不解碼 body |
| BodyNotRequested | 通過，但 body 是None，沒有完整文字可解碼 |
| Json(error) | 已取得文字，卻不能解碼成指定模型 |

**decode_page(&inspection)** 先按這個順序檢查。`serde_json::from_str::<PageRecord>(text.as_str())` 借用文字並回傳 `Result<PageRecord, serde_json::Error>`；錯誤有 Debug／Display。成功結果的欄位全 owned，沒有把 title 借在 Inspection 裡，原報告結束後 PageRecord 仍可使用。

## 完整 JSON 版與完整答案

完成版改用 `ch11-http-single`，manifest 此時加入實際使用的 Serde／serde_json：

```toml
--8<-- "examples/ch11-async/http-single/Cargo.toml"
```

完整 `src/reply.rs` 包含模型、錯誤Display、decode_page與測試。測試先讓人工 Inspection 在內層 scope 結束，外層才核對 PageRecord；另驗未讀body、revision誤用字串，以及HTTP404優先拒絕：

```rust
--8<-- "examples/ch11-async/http-single/src/reply.rs"
```

本版完整 `src/config.rs`、`src/error.rs` 與 `src/lib.rs` 如下。library 新增 `pub mod reply;`，依第2.6節規則載入同層 reply.rs；其餘 core 功能與前面的基本版相同：

```rust
--8<-- "examples/ch11-async/http-single/src/config.rs"
```

```rust
--8<-- "examples/ch11-async/http-single/src/error.rs"
```

```rust
--8<-- "examples/ch11-async/http-single/src/lib.rs"
```

完整 `src/main.rs` 與 `tests/cli.rs`，使用本完成版的 library／binary：

```rust
--8<-- "examples/ch11-async/http-single/src/main.rs"
```

```rust
--8<-- "examples/ch11-async/http-single/tests/cli.rs"
```

```bash
cargo test -p ch11-http-single --lib --locked
cargo test -p ch11-http-single --test cli --locked
```

現在有四項 library tests（core三項、reply一項）及兩項 CLI tests。計數反映本頁完整source，不表示已測過真正HTTP。

### 練習：由人工成功報告取回頁面資料

建立人工 Inspection，status200、passed=true、body為上述JSON。呼叫decode_page，確認title與revision，再印 `筆記：revision=3`。另外說明：若body為None，或revision改成字串，錯誤應在哪一層？

完整 `examples/page_reply_solution.rs`：

```rust
--8<-- "examples/ch11-async/http-single/examples/page_reply_solution.rs"
```

```bash
cargo run -p ch11-http-single --example page_reply_solution --locked
```

None是BodyNotRequested；有文字但revision為字串則是Json，兩者都不是Request逾時。此例只驗模型與政策，沒有送HTTP。

全頁來源已提供，可一起檢查兩版：

```bash
cargo test -p ch11-http-single-basic -p ch11-http-single --all-targets --locked
cargo clippy -p ch11-http-single-basic -p ch11-http-single --all-targets --locked -- -D warnings
```

單一工作已能交代HTTP與owned模型，但測試仍只能人工填報告。下一節建立真正的loopback服務，讓測試自己安排回應、進入證據與放行，再驗真正的等待及清理。

[下一節：受控本機服務與測試](14-09-lesson.md)
