# 14.2 Future 與 async／await：先保存工作，再取得結果

上一節把「工作尚未完成」與「最後拿到答案」分開。現在先移除外部等待，只算一筆小計80加固定運費20，得到100。這個立即能完成的例子，用來看清 Rust 怎樣表示工作；它還不展示多項工作交錯。

## 呼叫後拿到答案，還是工作？

**future** 是代表一份可能稍後完成工作的值。它保存需要的資料與進度，完成時交出結果；工作也可能一直沒有完成。標準函式庫的 **`std::future::Future` trait** 描述這項能力，associated type **`Output`** 指定完成後交出的型別。這裡沿用第6.4節的 associated type：同一個實作把來源型別與結果型別連起來。

普通的 `fn total_now(...) -> i32` 呼叫時執行本體，交回 i32。在宣告前加 **`async`**，`async fn total_later(...) -> i32` 的呼叫則交回 future；箭頭的 i32 是那份工作的 **Output**，不是呼叫當下交回的型別。async 函式本體留到工作被執行時才進入，這是本例所說的 **lazy（延後執行）**。

在 async 函式內寫 **`work.await`**，會推進並等待 work，取得它的 Output。`.await` 是語法，不是 method，不加括號。若結果尚未就緒，周圍的 async 工作能保存進度並暫停；若已就緒就繼續，**每次 await 不一定真的暫停**。

下面先對照兩個計算函式。小值80加20不會溢位；本例沒有訂出所有 i32 輸入的溢位政策：

```rust
--8<-- "examples/ch11-async/foundations/src/lib.rs:totals"
```

「本體尚未執行」不代表呼叫時完全沒動作：引數先求值，owned 引數在呼叫時交進 future。等一下的 `drop` 會讓我們看見：保存過工作，也可以沒有執行本體就結束它。

## 讓 runtime 推進工作

future 需要被驅動才能執行。**runtime** 是執行 async 工作的基礎設施；其中的 **executor（執行器）** 推進可執行的工作，工作等待的條件有機會滿足時，再安排它繼續。Rust 提供語法與 Future trait，本章選用 **Tokio** 作 runtime。

先選 **current-thread runtime**：在呼叫者目前的 thread 上執行 async 工作，不增加 async worker threads。thread 是執行程式的地方，future 是被保存及推進的工作；建立 future 沒有因此建立 thread。

完整早期 package 設定如下，位於 `examples/ch11-async/stages/foundations-steps/`。共同欄位繼承第12.1節已展示的 repo workspace 設定；`rt` 提供 runtime，`macros` 提供入口 attributes，尚未要求 timer 或 channel。package 名稱保留範例原有的 `ch11` 系列，章節編號以本頁為準：

```toml
--8<-- "examples/ch11-async/stages/foundations-steps/Cargo.toml"
```

本章查核 Tokio 1.53.3，實際解析版本保存在根 Cargo.lock。這份 package 沒有 library；每份 example 都有完整的本地函式。預設 binary 只提示選例，完整 `src/main.rs`：

```rust
--8<-- "examples/ch11-async/stages/foundations-steps/src/main.rs"
```

普通 Rust 入口仍是同步 `fn main`。**`#[tokio::main(flavor = "current_thread")]`** 是放在函式前的 attribute macro：它把下面寫成 async 的 main 轉成普通入口，建立所選 runtime，推進 main 的 future 到完成。`flavor` 選執行方式；不指定時的預設是 multi-thread，需要另一項 feature，所以這裡明寫 current_thread。方便入口若無法初始化 runtime 會 panic，這與工作自己交回的 Result 是不同層次。

## 完整程式：建立、執行、丟棄

完整 `examples/lazy.rs` 把剛才的函式與 main 放在同一檔。普通函式先算完；`work` 建立後才 await；另一份小計120的工作則直接交給第11.3節已教的 `drop` 結束：

```rust
--8<-- "examples/ch11-async/stages/foundations-steps/examples/lazy.rs"
```

以下命令全部從 repo 根目錄執行，並且只選當下已提供的 target：

```bash
cargo run -p ch11-foundations-steps --example lazy --locked
```

stdout 的固定順序是：

```text
普通函式已交回：100
future 已建立，async 本體尚未執行
async 本體：處理小計 80
await 已取得結果：100
另一份 future 未執行就已丟棄
```

本體標記在「future 已建立」之後。小計120沒有本體標記：丟棄這份尚未執行的工作，不會為了丟棄而執行它。這些輸出支持建立、本體執行與丟棄的區分；因為運算立即能完成，尚未展示真正等待與交錯，也不是效能比較。

## async block 保存哪些資料？

不必每次替工作命名函式。**`async { ... }`** 建立一份 future，block 的末尾值成為它的 Output；本體同樣延後執行。它會依使用方式捕捉外部資料，和第9.1節 closure 的捕捉問題相連。

第一份工作只讀 heading，保存借用，Output 是文字的 byte 數4。建立後，main 仍可用 shared reference 讀 heading。第二份用 **`async move`**，把 note 這個 String 的 ownership 交給工作；本體末尾再把 String 交作 Output，所以 await 取得的 saved 擁有文字。move 捕捉若原本拿到的是 reference，移入的仍是 reference，不會把它指向的值自動變成 owned 資料。

完整 `examples/block_capture.rs`：

```rust
--8<-- "examples/ch11-async/stages/foundations-steps/examples/block_capture.rs"
```

```bash
cargo run -p ch11-foundations-steps --example block_capture --locked
```

先顯示「block 已建立」，之後 await 才印借用本體；兩個 assertion 分別確認4與配送。若在 note 已移入後又使用 note，就會違反前文已教的 ownership 規則；async 沒有取消那項規則。

## 普通工廠可以先做準備

第9.3節的 return-position `impl Trait` 讓函式交出一個實作指定 trait 的具體型別。現在用 **`impl Future<Output = i32>`** 表示「交回一份完成後產生 i32 的工作」。尖括號裡的等號指定 associated type，不是替 Output 傳入一個值。

下面的 prepared_total 是**普通函式**。呼叫立即印出準備標記，再建立並交回 async move block；只有 block 內的加總延後。這是為什麼不能把「凡是回傳 future 的函式」都說成呼叫時完全不做事。

完整 `examples/explicit_future.rs`，包含它使用的 total_now：

```rust
--8<-- "examples/ch11-async/stages/foundations-steps/examples/explicit_future.rs"
```

```bash
cargo run -p ch11-foundations-steps --example explicit_future --locked
```

準備標記在取得結果100之前。之後使用 library 的普通 request 工廠時，也要查清呼叫與等待各做什麼。

## 完成也可能交出 Err

把工作規格改成「小計不能負數」，沿用第8章的 Result。**`async fn ... -> Result<i32, String>`** 的 Output 整個就是 Result。await 只取得 Output，沒有代為解開 Ok，也沒有替我們選錯誤政策。

完整 `examples/fallible_output.rs` 用 −10、80兩筆小值：

```rust
--8<-- "examples/ch11-async/stages/foundations-steps/examples/fallible_output.rs"
```

```bash
cargo run -p ch11-foundations-steps --example fallible_output --locked
```

第一份工作完成後得到 Err，第二份得到 Ok(100)。for 中每次都 await 到結果才進下一筆，這裡是依序執行，沒有因為寫成 async 就同時推進兩份工作。

## 把本節功能收進完整 library

以下完成版 `ch11-foundations` 將計算集中成 library，方便下一節引用。先介紹新增的測試入口：**`#[tokio::test(flavor = "current_thread")]`** 把 async 測試接到每個測試各自的 current-thread runtime，測試本體可以 await；普通 `#[test]` 本身不會建立這種 async 執行環境。

library 中的測試用 mutable visits 追蹤本體副作用。第一份 async block 借用 visits，建立時還沒加1；drop 結束這份借用，檢查仍是0。第二份 await 完成，才確認結果和 visits 都是1。這同時說明「工作被丟棄」與「工作執行完」對同一份資料的差異。

完整 manifest 與 `src/lib.rs`（含整個測試 module）：

```toml
--8<-- "examples/ch11-async/foundations/Cargo.toml"
```

```rust
--8<-- "examples/ch11-async/foundations/src/lib.rs"
```

完整 `src/main.rs` 使用這份 library，行為與前面的 lazy 一樣：

```rust
--8<-- "examples/ch11-async/foundations/src/main.rs"
```

```bash
cargo test -p ch11-foundations --lib --locked
cargo run -p ch11-foundations --locked
```

integration test 沿用第10.8節的 Command／Output 與第12.2節的 env!。Cargo 為 integration test 提供 `CARGO_BIN_EXE_` 加 binary 名稱的編譯時環境值，內容是已建好 binary 的路徑。此處 `Command::output` 等待這支有限程式結束，取得 status/stdout/stderr；`str::from_utf8` 檢查輸出 bytes 後借作文字。

完整 `tests/cli.rs` 把前述五行 stdout 當精確規格：

```rust
--8<-- "examples/ch11-async/foundations/tests/cli.rs"
```

```bash
cargo test -p ch11-foundations --test cli --locked
```

完成版也保留三份具名 example。它們的 block capture 與早期版相同；另外兩份改從這份已完成的 library 匯入計算。以下各是完整檔案，沒有額外 helper：

```rust
--8<-- "examples/ch11-async/foundations/examples/block_capture.rs"
```

```rust
--8<-- "examples/ch11-async/foundations/examples/explicit_future.rs"
```

```rust
--8<-- "examples/ch11-async/foundations/examples/fallible_output.rs"
```

## 練習與完整答案：保存一份收據工作

寫 async receipt，接收小計80與 owned label「訂單 A」，await 本頁的 total_later，再用第8.3節的 format! 交回 owned String「訂單 A：總額 100」。main 先保存 future，再 await，確認結果。預測：建立 future 時，是否已印出 total_later 本體標記？

完整 `examples/receipt_solution.rs`：

```rust
--8<-- "examples/ch11-async/foundations/examples/receipt_solution.rs"
```

```bash
cargo run -p ch11-foundations --example receipt_solution --locked
```

先印「收據 future 已保存小計與標籤」，await 推進 receipt 之後才進入 total_later 本體，最後取得收據。label 在呼叫 receipt 時已移進 future，不必等到本體執行才交 ownership。

本節全部 target 已提供，可以一起驗證。`--all-targets` 沿用第12.1節，會包含 package 的 library、binary、examples 與測試；因此放在答案完整出現後：

```bash
cargo test -p ch11-foundations-steps --all-targets --locked
cargo test -p ch11-foundations --all-targets --locked
cargo clippy -p ch11-foundations-steps -p ch11-foundations --all-targets --locked -- -D warnings
```

現在能分清「有一份 future」與「本體已完成」。下一節用真正的 started 與 release 訊息建立等待條件，並把工作交給 runtime 作為 task 管理。

[下一節：spawn、JoinHandle 與 JoinSet](14-03-lesson.md)
