# 18.12 完整 URL checker：保存全部來源，驗收真正工具

上一節完成報告映射、資料狀態與write／flush。這一節提供正式 `url-checker` 的整份可用答案：一份UTF-8網址清單，受jobs限制的HTTP工作，最後依原行排列文字或JSON。可以直接保存完整目錄建置，也可以與自己的最後工作副本逐檔比較。

本頁沒有新的實作題。先核對入口與每份來源的角色，再取得全檔、執行限定targets；同一份工具的早期快照不累加成額外作業。

## 完整版多帶了哪些驗收檔？

normal library仍是18.11的app／batch／config／error／input／report／single，root以pub use提供公開入口，main連接args_os、stdout／stderr與ExitCode。正式root不再保留起點的兩項dead_code allow。

`src/batch.rs`另外含七項本地HTTP／停止測試；`src/report.rs`有五項，`src/app.rs`有四項，共16項lib tests。`src/lib.rs`以 **`#[cfg(test)] mod test_fixture;`** 載入本地服務，因此只有lib test harness載入它；normal binary、integration test借用的normal library不會編譯這份fixture module。名字filter只決定執行哪些測試，不能把同一library其餘module藏掉。

這份 `src/test_fixture.rs`不再path依賴教材HTTP package；它從本package的single借build_client／parse_http_url，直接使用14.9已教的axum／IntoFuture／TcpListener／Arc／Mutex／AtomicUsize／mpsc。`ready_response`先保存status，再text().await讀約定body；外層10秒timeout涵蓋send和body。啟動失敗先shutdown，held handler短guard移出gate後才await；shutdown先關gate、通知停止、await server。fixture的body等待與產品single只等final headers仍分開。

七項batch tests沿18.10的bounded owner安排：每份觀察都有期限，先保存原tracked／Id登記數，再fallback drain與shutdown，最後才assert。保留提前Requested不提交、來源失敗不是取消、完成未收取保原Check／TaskFailure、真HTTP混合與Id→完整Input的驗收。合成task的panic hook可能顯示診斷，不等於整份test未處理失敗。

`tests/config.rs`是獨立integration target，只有兩項Parser測試：預設jobs8／timeout10秒／json=false，以及指定jobs2／timeout5秒／json=true、help與用法錯誤。它不啟動HTTP，也不能代替真binary的出口驗收。

## 依檔名保存完整答案

以下是全部檔案，包含library root、main、helpers、同檔tests、本機fixture、integration tests、manifest、lock、README與固定輸入。Rust片段直接引用實際可編譯來源；沒有省略imports或只顯示solution region。Cargo.lock由Cargo維護，隨整個目錄保存，不要求手抄解析項目。

??? example "展開完整設定、全部 Rust 來源與測試材料"

    `examples/ch15-projects/url-checker/Cargo.toml`：

    ```toml
    --8<-- "examples/ch15-projects/url-checker/Cargo.toml"
    ```

    `examples/ch15-projects/url-checker/Cargo.lock`：

    ```toml
    --8<-- "examples/ch15-projects/url-checker/Cargo.lock"
    ```

    `examples/ch15-projects/url-checker/src/lib.rs`：

    ```rust
    --8<-- "examples/ch15-projects/url-checker/src/lib.rs"
    ```

    `examples/ch15-projects/url-checker/src/config.rs`：

    ```rust
    --8<-- "examples/ch15-projects/url-checker/src/config.rs"
    ```

    `examples/ch15-projects/url-checker/src/error.rs`：

    ```rust
    --8<-- "examples/ch15-projects/url-checker/src/error.rs"
    ```

    `examples/ch15-projects/url-checker/src/input.rs`：

    ```rust
    --8<-- "examples/ch15-projects/url-checker/src/input.rs"
    ```

    `examples/ch15-projects/url-checker/src/single.rs`：

    ```rust
    --8<-- "examples/ch15-projects/url-checker/src/single.rs"
    ```

    `examples/ch15-projects/url-checker/src/batch.rs`：

    ```rust
    --8<-- "examples/ch15-projects/url-checker/src/batch.rs"
    ```

    `examples/ch15-projects/url-checker/src/report.rs`：

    ```rust
    --8<-- "examples/ch15-projects/url-checker/src/report.rs"
    ```

    `examples/ch15-projects/url-checker/src/app.rs`：

    ```rust
    --8<-- "examples/ch15-projects/url-checker/src/app.rs"
    ```

    `examples/ch15-projects/url-checker/src/main.rs`：

    ```rust
    --8<-- "examples/ch15-projects/url-checker/src/main.rs"
    ```

    `examples/ch15-projects/url-checker/src/test_fixture.rs`：

    ```rust
    --8<-- "examples/ch15-projects/url-checker/src/test_fixture.rs"
    ```

    `examples/ch15-projects/url-checker/tests/config.rs`：

    ```rust
    --8<-- "examples/ch15-projects/url-checker/tests/config.rs"
    ```

    `examples/ch15-projects/url-checker/fixtures/invalid-urls.txt`：

    ```text
    --8<-- "examples/ch15-projects/url-checker/fixtures/invalid-urls.txt"
    ```

??? example "展開隨工具保存的完整 README"

    ````text
    --8<-- "examples/ch15-projects/url-checker/README.md"
    ````

## 使用完整答案，核對真實可用的範圍

在教材repo根目錄先進入`examples/ch15-projects/url-checker`，或把**整個目錄**保存到repo外。以下命令的相對檔名以該目錄為起點，`--manifest-path`不會替程式換工作目錄。

隨附fixtures/invalid-urls.txt是有空行、兩份相同ftp網址與一份`/notes`的真材料。下表明示原檔每行的內容；執行時使用隨完整目錄提供的檔案，最後LF不增加一份輸入。

| 原index | 該行完整內容 | 處理方式 |
| --- | --- | --- |
| 0 | 空行 | 略過，仍占原行索引 |
| 1 | `ftp://host/item` | 保留為輸入 |
| 2 | 一個空白字元 | 略過，仍占原行索引 |
| 3 | `/notes` | 保留為輸入 |
| 4 | `ftp://host/item` | 保留為另一份輸入 |

```bash
cargo +1.99.0 build --release --locked
cargo +1.99.0 run --quiet --locked -- fixtures/invalid-urls.txt --json
```

run的退出狀態不是只看終端機有沒有報告。沿10.5在上一條命令後緊接 `echo $?`（PowerShell使用 `echo $LASTEXITCODE`），應觀察1；不要因非零就把這份正確保存Input問題的報告改成成功0。

這份材料不送HTTP；報告原index為1／3／4，三筆kind=input，各自保留original，時間與HTTP欄位null，退出1。換成自己的http／https清單才是真網路檢查，結果依環境與當時服務而定；可用`--jobs 2 --timeout-secs 5`選兩個正數設定，或省略--json取得同份資料的文字表示。一般取消操作是按Ctrl+C，等報告與130；不要用強制終止代替這份收尾。

這次讓 Clippy 回報的警告也使檢查失敗：`--` 將後面的選項交給 Clippy，`-D warnings` 把警告提升為錯誤。沿2.7，沒有警告仍不代表功能已正確。

```bash
cargo +1.99.0 test --lib --locked
cargo +1.99.0 test --test config --locked
cargo +1.99.0 clippy --lib --bin url-checker --test config --locked -- -D warnings
```

完整答案有16項library tests及2項config integration tests；保留B4的有限本機HTTP／停止驗收，加本節報告與writer checks；各觀察期限只守測試，不變成產品整批期限。真正binary另以本機受控服務與明確請求進入通知驗CLI及SIGINT；這些與lib tests分開記錄。手動使用時，Ctrl+C須讓程式自己收尾並交出130；強制終止不提供這份完成證據。

若要安裝成平常可呼叫的命令，新增Cargo操作 **`cargo install --path . --locked`**：`--path .`選目前本機package，建置其binary並安裝到Cargo的binary目錄，不上傳registry；--locked仍依本次lock。toolchain選擇沿前面寫在cargo後：

```bash
cargo +1.99.0 install --path . --locked
```

shell能找到Cargo binary目錄後，便可用`url-checker 自己的清單.txt --json`。README也列建置、執行、測試、輸出政策與限制。本參考版按實際檔案行數計核心825、測試898、本機Fixture198，共1921行Rust（含空白與snippet標記，不含lock）；新練習只補前述三個動作，不要求一次手寫全部參考tests，也不累加階段副本。這份工具仍讀完整小清單、保存所有結果；jobs限制已提交tasks，不限制整張輸入與報告的記憶體。

延伸閱讀：[English Book：TDD](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html)、[繁中對應](https://rust-lang.tw/book-tw/ch12-04-testing-the-librarys-functionality.html)、[English Book：stdout／stderr](https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html)、[繁中對應](https://rust-lang.tw/book-tw/ch12-06-writing-to-stderr-instead-of-stdout.html)。本節採其小步驗收與出口分工，實作依本書已教的typed錯誤、owned資源及flush契約。

## 交回可使用的答案，也保留界線

本工具已完成單筆分類、原行身分、有限提交、每筆期限、停止後回收、完整報告與CLI。jobs數的是已提交尚未收回的task，完成未收也在其中；它沒有限制所有輸入／結果／格式化String的總記憶體。輸入仍須是完整可讀的UTF-8小清單。

每份GET可能已對server產生影響，取消不會回復它。HTTP2xx只滿足本題政策，不驗網站業務內容；不主動讀body也不保證底層完全沒收到body bytes。正常完成／使用者停止均收回client與listener，報告write及flush成功才交資料狀態，fatal2始終優先。

把每份輸入的去向、每個owner的收尾，以及每個出口的完成條件保存清楚，才得到可持續修改的工具。若要擴大為大型清單或常駐服務，先另訂串流、記憶體與停止政策；本題的完整答案保留目前明定且已驗收的範圍。
