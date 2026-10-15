# 14.7 HTTP 基礎：有收到回應，和檢查通過，是兩件事

現在要檢查一個網址：收到最終回應後，交代 final URL、HTTP status，以及是否符合「2xx才通過」的規格。使用者還能選擇讀不讀 body。先用同步介面把資料與失敗階段看清楚，下一節保留相同政策，再換成 async 等待。

## URL 指出位置，HTTP 交接請求與回應

**URL** 是 Uniform Resource Locator（統一資源定位符），例如 `https://example.org/notes?lang=rust#intro`。其中 https 是 **scheme**，example.org 是主機，/notes 是 path，?lang=rust 是 query，#intro 是 fragment。它們有不同用途；fragment 不作為 HTTP 請求內容送給 server。

**HTTP** 是 Hypertext Transfer Protocol（超文字傳輸協定）。**client** 發出 request（請求），**server** 依協定交回 response（回應）。本例使用 **GET**，表示取得指定資源；**HTTPS** 是透過 TLS（Transport Layer Security，傳輸層安全協定）保護的 HTTP，涉及連線加密及對方身分驗證。

response 包含 **status code（狀態碼）**、**headers（標頭）** 與可能有的 **body（內容）**。例如200通常表示成功，404表示找不到資源；HTTP 的404回應本身仍是收到的一份回應。連線失敗、在期限內沒收到最終回應，則沒有這份可供檢查的結果。

本工具訂出政策：200到299這個 **2xx** 範圍為 passed=true；其他 HTTP status 仍交出報告，passed=false。若回應要求 **redirect（重新導向）**，client 可依新位置再請求；所以輸入 URL 與最終 URL 可能不同。這次最多允許5次 redirect，超過則是取得最終回應失敗。

## 先把 CLI 參數保存成 owned 設定

package 在 `examples/ch11-http/`，名稱為 `ch11-http`。它使用 **reqwest** 的 blocking（阻塞）client：呼叫 send／text 時，目前 thread 等它們完成或失敗。這份 binary 使用普通 main，不在本節的 async runtime 內呼叫 blocking client。

完整 manifest 使用 clap derive、reqwest blocking／rustls、url。rustls 是選用的 TLS 實作；`default-features = false` 關閉 reqwest 預設 features，再明列本例需求：

```toml
--8<-- "examples/ch11-http/Cargo.toml"
```

第10.6節的 clap Parser 產生參數解析，這次 Settings 保存 owned URL 文字，以及兩個 bool 開關：`--body` 要讀完整內容；`--direct` 要直接連線、關閉 proxy。**proxy（代理伺服器）** 可代為轉交請求，client 預設可能採系統／環境設定；direct 讓本次明確選擇不使用它。

完整 `src/config.rs`：

```rust
--8<-- "examples/ch11-http/src/config.rs"
```

## 解析成功，還要符合本工具的 scheme

`url::Url` 是解析後的 owned URL 型別，提供 Debug；**`Url::parse(input)`** 回傳 `Result<Url, url::ParseError>`，解析錯誤有 Debug／Display。**`url.scheme()`** 借出 scheme 文字，**`url.as_str()`** 借出正規化後的完整文字；例如大寫 HTTPS 會規範成 https。

只接受 http／https，是本工具額外訂的規則。`file:///tmp/notes.txt` 可以是合法 URL，卻不是本工具支援的 scheme；`/notes` 則缺少這裡要求的完整位置，解析就失敗。

```rust
--8<-- "examples/ch11-http/src/lib.rs:parse-url"
```

**Inspection** 保存最終地址、數字 status、passed 與 `Option<String>` 的 body。**None 是沒有要求讀 body**；Some(空字串) 是已讀而內容為空。不能把兩者都顯示成「內容不存在」：

```rust
--8<-- "examples/ch11-http/src/lib.rs:model"
```

## builder 準備規則；send 才取得回應

這次選用的 API 依操作先後如下。它們是 reqwest blocking module 的介面，不回傳讓我們 await 的工作：

| 操作 | 取得或改變什麼 |
| --- | --- |
| `Client::builder()` | 建立 ClientBuilder，先保存 client 設定 |
| `.redirect(Policy::limited(5))` | 接手並交回 builder，限制最多5次重新導向 |
| `.timeout(Duration::from_secs(10))` | 設定此次使用的10秒等待期限 |
| `.no_proxy()` | 接手並交回 builder，關閉所有 proxy，包括自動系統設定 |
| `.build()` | 接手 builder，交回 `Result<Client, reqwest::Error>` |
| `client.get(url)` | shared 借用 client、接手 Url，建立 GET 的 RequestBuilder，尚未送請求 |
| `.send()` | 接手 RequestBuilder，等待最終回應，交回 `Result<Response, reqwest::Error>` |

reqwest::Error 有 Debug／Display，**`is_timeout()`** 回答是否屬於逾時失敗。`.build()` 也可能失敗，與送出請求後才發生的 Request 錯誤分開記錄。

拿到 Response 後，**`response.status()`** 交回可 Copy 的 StatusCode；`is_success()` 判2xx，`as_u16()` 取得數字。**`response.url()`** 借出最終 Url。先把這兩份資料保存，再決定讀不讀 body，因為 **`response.text()`** 會接手 Response，讀完整內容並交回 `Result<String, reqwest::Error>`。

本次不啟用 charset feature，text 依 UTF-8 解碼，無效序列用替代字元取代；它不同於第10.4節要求合法 UTF-8、否則回錯誤的 String::from_utf8。這個範例也沒有設定 body 大小上限，--body 會收完整文字，不能將它當成只預覽固定幾個 bytes。

**固定 reqwest 0.13.5 的 blocking send 與 text 各自套用等待期限**，不是兩階段共用同一份10秒預算。因此不能把這份工具宣稱為整個檢查最多10秒。下一節 async client 的總期限契約另行說明。

```rust
--8<-- "examples/ch11-http/src/lib.rs:inspect"
```

404在這段仍是正常 Response，會產生 passed=false 的 Inspection。若已收到status、後續 text 卻失敗，則保留該數字進 Body 錯誤，不把它誤報成完全沒收到 response。

## 用錯誤型別保留階段，用 writer 檢查輸出

完整 **`src/error.rs`** 將 Url／Scheme／Client／Request／Body／Output 分開，Display 產生可閱讀診斷。status() 是本工具自訂的 method：輸入問題回退出數字2，其餘操作失敗回1，和 Response 的 HTTP status 是不同欄位與用途。

```rust
--8<-- "examples/ch11-http/src/error.rs"
```

render 沿用第10.4節的泛型 Write，輸出報告後也檢查 flush；run 在取得報告、完整輸出成功後才交回 passed。HTTP404的流程是 **Ok(false)**；連線、讀 body 或輸出失敗才是 AppError：

```rust
--8<-- "examples/ch11-http/src/lib.rs:render-run"
```

## 完整程式與離線驗證

完整 **`src/lib.rs`** 包含兩個 module 宣告、所有函式與四項 unit tests。RefuseFlush 是同檔完整測試 writer：write 先保存 bytes，flush 則交出 PermissionDenied。它用第10.4節的 Write 實作及 io::Error／ErrorKind，驗證「寫入已接受」仍不能掩蓋 flush 失敗。

```rust
--8<-- "examples/ch11-http/src/lib.rs"
```

完整 **`src/main.rs`** 用 args_os、try_parse_from 與 error.print 接上第10.6節的 clap 流程；錯誤參數在送網路請求前拒絕。檢查未通過也是退出1，但有完整報告；AppError 則將診斷寫 stderr：

```rust
--8<-- "examples/ch11-http/src/main.rs"
```

從 repo 根目錄執行以下離線命令：

```bash
cargo run -p ch11-http --locked -- --help
cargo test -p ch11-http --lib --locked
```

四項 unit tests 驗解析與 scheme、人工404報告與 body 選擇，以及拒絕 flush。這些測試沒有發出 HTTP 請求。

完整 **`tests/cli.rs`** 包含 execute helper，以真正 binary 驗 help 與三種輸入錯誤；它們也都在網路前結束：

```rust
--8<-- "examples/ch11-http/tests/cli.rs"
```

```bash
cargo test -p ch11-http --test cli --locked
```

若要觀察一次真實公開網站，可另執行以下選用命令；外部回應、proxy 與連線條件都可能變動，這不是固定測試答案：

```bash
cargo run -p ch11-http --locked -- https://example.org
cargo run -p ch11-http --locked -- https://example.org --body
```

需要直接連線時再加入 --direct。這兩次分別選擇不讀與讀完整 body；不要只比輸出行數，就宣稱量出了兩種方式的效能差異。

## 練習與完整答案：同一份報告的摘要

寫 status_line，借用 Inspection，產生 owned String：passed=true 使用「通過」，false 使用「HTTP 未通過」，並保留 final URL 與 status。只用人工200／404報告驗證，不為格式化練習發出請求。

完整 `examples/status_line.rs`：

```rust
--8<-- "examples/ch11-http/examples/status_line.rs"
```

```bash
cargo run -p ch11-http --example status_line --locked
cargo test -p ch11-http --all-targets --locked
cargo clippy -p ch11-http --all-targets --locked -- -D warnings
```

最後印 `HTTP 未通過：https://example.org/notes（HTTP 404）`。這仍是一份可用的檢查報告；它與完全拿不到 Response 的 Request 失敗不同。

下一節保留這些資料與失敗政策，改用 async client，讓等待回應及 body 的進度能由 runtime 推進。

[下一節：單一 async HTTP 與 owned JSON 回覆](14-08-lesson.md)
