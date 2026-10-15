# 14.5 timer 與 timeout：設定等待範圍，再分辨完成和逾時

上一節限制在途 task 數。現在單看一份工作：若它一直沒有結果，呼叫者要等到什麼時候？我們要替一段明確的工作設定等待期限，並分清「工作完成但回報資料錯誤」與「期限到時仍沒完成」。

本節先用本地 timer，不碰網路。真實執行時間可觀察，但機器忙碌會影響排程，不把某次剛好20毫秒當成固定答案。

## Duration 是一段長度，Instant 用來量經過時間

標準庫 **`std::time::Duration`** 表示時間長度；`Duration::from_secs(1)` 建立1秒，`Duration::from_millis(20)` 建立20毫秒，1秒等於1000毫秒。`duration.as_millis()` 回傳總毫秒數，供本例顯示，會捨去不足1毫秒的部分。Duration 具備 **Copy**；把同一份時間跨度按值交給 sleep 或 timeout 後，原 binding 仍可再用。

**`std::time::Instant::now()`** 取得適合量經過時間的單調時間點；之後 `start.elapsed()` 得到 Duration。它不是年月日鐘面，也不把別台機器的時間點拿來相減。本例從呼叫 sleep 前開始量，包含等待與這次排程的時間。

**`tokio::time::sleep(delay)`** 建立到期後可完成的 timer future，Output 是 unit。呼叫 sleep 時就以當時時間安排期限；await 推進並等待它，等待期間 thread 可以執行其他可前進的 async 工作。它不要求這條 thread 用同步阻塞一直留在原處，也不保證到期後立刻獲得執行機會。

早期 package `ch11-timing-steps` 沒有 library；完整設定增加 **`time` feature**，由前面的 Tokio 入口啟用 timer 支援。建立 timer 要在有這項支援的 runtime 內，否則可能 panic。path dependency 只指向14.2已完成的 foundations：

```toml
--8<-- "examples/ch11-async/stages/timing-steps/Cargo.toml"
```

完整提示用 `src/main.rs`：

```rust
--8<-- "examples/ch11-async/stages/timing-steps/src/main.rs"
```

完整 `examples/timing_measured.rs`：

```rust
--8<-- "examples/ch11-async/stages/timing-steps/examples/timing_measured.rs"
```

從 repo 根目錄執行：

```bash
cargo run -p ch11-timing-steps --example timing_measured --locked
```

前兩行固定顯示1000與20毫秒；最後一行是**這次量測值**。這裡 sleep 的目的是等待 timer 到期，不拿它猜其他 task 是否已開始。

## timeout 包住哪一份工作？

**`tokio::time::timeout(limit, work)`** 接手本例的一份 future，建立帶期限的等待；呼叫時就記下 deadline，緊接著 await 取得 **`Result<T, Elapsed>`**。T 是原工作 Output，Elapsed 是 Tokio 的逾時錯誤，具備 Debug 與 Display，可供 expect 的診斷與文字顯示。

原工作完成得到 Ok(output)；仍沒完成而期限已到得到 Err(Elapsed)。這份 wrapper 結束並 drop 它持有的工作，取消後續執行。本節都直接包住一份未 spawn 的 future；若持有的是另一份 task 的把手，還要分辨把手與 task，下一節再追那種責任。

先準備兩份容易辨認的材料：`async { 100 }` 立即交出100；標準庫的 **`std::future::pending::<i32>()`** 建立一份永遠不交出 i32 的工作。`::<i32>` 指定它假想的 Output 型別，不是傳入金額。

完整 `examples/timing_outcomes.rs`：

```rust
--8<-- "examples/ch11-async/stages/timing-steps/examples/timing_outcomes.rs"
```

```bash
cargo run -p ch11-timing-steps --example timing_outcomes --locked
```

第一份得到 Ok(100)，第二份逾時。固定版本先讓內層工作取得執行機會，立即完成的工作可直接成功；timeout 不是靠外部強制中斷任意一行同步程式。

## 工作 Output 是 Result，就保留兩層

14.2的 checked_total 可能交回 `Result<i32, String>`。加上 timeout，結果成為 **`Result<Result<i32, String>, Elapsed>`**：外層回答是否在期限內完成；內層才回答資料是否符合小計規格。

| 結果 | 這次發生什麼 |
| --- | --- |
| Ok(Ok(100)) | 工作完成，小計有效 |
| Ok(Err(message)) | 工作已完成，卻回報負小計 |
| Err(elapsed) | 工作到期限仍未完成，沒有取得其 Output |

完整 `examples/timing_nested.rs`：

```rust
--8<-- "examples/ch11-async/stages/timing-steps/examples/timing_nested.rs"
```

```bash
cargo run -p ch11-timing-steps --example timing_nested --locked
```

−10與80這兩份工作都立即完成，分別是外層 Ok 包住內層 Err、外層 Ok 包住內層 Ok(100)。資料錯誤不需要一直等到期限才回報。後面的受控測試會再驗真正尚未完成的第三條路徑。

## 丟棄工作不會回復已發生的修改

沿用14.2的 visits，這次先加1，再 await 永不完成的 pending，之後才有一行加100。timeout 到期時丟棄工作，結束它保存的 mutable 借用；已完成的加1仍留在 visits，後面的加100沒執行。

完整 `examples/timing_cancelled.rs`：

```rust
--8<-- "examples/ch11-async/stages/timing-steps/examples/timing_cancelled.rs"
```

```bash
cargo run -p ch11-timing-steps --example timing_cancelled --locked
```

assertion 確認 visits 是1。取消不是把資料回復到建立 future 前；需要補償的業務操作，仍須另外設計。

## 同步阻塞不會被 timer 硬切開

標準庫另外提供 **`std::thread::sleep(duration)`**，接收 Duration，阻塞目前 thread 等待後回傳 unit，不回傳 future。若把它寫進 async 本體，呼叫那一段時仍會阻塞，沒有因為外面是 async 就變成 Tokio sleep。

完整 `examples/timing_blocking.rs` 故意讓本體同步停30毫秒，再直接回100，外層只給1毫秒：

```rust
--8<-- "examples/ch11-async/stages/timing-steps/examples/timing_blocking.rs"
```

```bash
cargo run -p ch11-timing-steps --example timing_blocking --locked
```

本固定版本先推進內層；它沒有交還執行機會就一次完成，結果仍是 Ok(100)，實際等待可超過設定的1毫秒。這展示的是 timeout 的合作式邊界，不是建議用小期限處理阻塞工作。

## 用受控時鐘驗證三種結果

若測試只為檢查 timer 與 Result 路徑，讓它真的睡很久沒有幫助。Tokio 的 **`test-util` feature** 提供測試時鐘能力。`#[tokio::test(flavor = "current_thread", start_paused = true)]` 從測試開始暫停 Tokio 時鐘；沒有其他可前進工作、只剩 timer 等待時，runtime 能自動把這份時鐘推進到下一個 timer。

這時用 **`tokio::time::Instant`** 讀同一份 Tokio 時鐘，now／elapsed 的用法與 std Instant 相近；std Instant 仍量真實經過時間，不能拿它斷言虛擬100毫秒。本節不測真實網路延遲。

獨立的 no-lib `ch11-timing-clock` 只載入這一階段。完整 manifest 把 test-util 放進 dev-dependencies，讓測試能用；尚未加入後段的兩步工作：

```toml
--8<-- "examples/ch11-async/stages/timing-clock/Cargo.toml"
```

完整 `src/main.rs`：

```rust
--8<-- "examples/ch11-async/stages/timing-clock/src/main.rs"
```

完整 `tests/clock_and_results.rs` 包含本地 total_after。這份工作先拒絕負小計；有效小計才等待給定 delay，再交回加運費後的金額。四項測試分別驗時鐘，以及成功、資料錯誤、逾時：

```rust
--8<-- "examples/ch11-async/stages/timing-clock/tests/clock_and_results.rs"
```

```bash
cargo test -p ch11-timing-clock --test clock_and_results --locked
```

負小計在等待前回 Err，Tokio 時鐘沒有因此前進100毫秒。有效小計等待10毫秒、期限100可完成；等待100、期限10則取得外層逾時。這些測試支持本地受控 timer 的結果分類，沒有保證某次 OS 排程或外部服務耗時。

## 整段期限，和每步重新給期限

回到兩個必要步驟：每步等30毫秒，完成後把 completed 加1。**two_steps** 借用 caller 的 completed，讓整段被丟棄後仍能檢查已完成幾步：

```rust
--8<-- "examples/ch11-async/timing/src/lib.rs:two-steps"
```

整段只給50毫秒時，第一步在30完成，第二步還沒完成就到期限，completed=1。若每一步各包一份50毫秒 timeout，每次30都在自己的期限內，兩步完成，completed=2；整體允許的時間已改變。

完成版 `ch11-timing` 重用14.2已完成的 checked_total，再加入 total_after／two_steps。完整 manifest、`src/lib.rs`：

```toml
--8<-- "examples/ch11-async/timing/Cargo.toml"
```

```rust
--8<-- "examples/ch11-async/timing/src/lib.rs"
```

完整 `tests/timing_contracts.rs` 保存前面的時鐘／三種結果，以及剛才整段與逐步期限的兩項測試。實際編譯的是整個檔案，所以全部來源在第一條測試命令前提供：

```rust
--8<-- "examples/ch11-async/timing/tests/timing_contracts.rs"
```

```bash
cargo test -p ch11-timing --test timing_contracts --locked
```

完成版還保留本頁先前的五份完整示範，內容採同樣等待材料與政策；以下各自是全檔：

```rust
--8<-- "examples/ch11-async/timing/examples/timing_measured.rs"
```

```rust
--8<-- "examples/ch11-async/timing/examples/timing_outcomes.rs"
```

```rust
--8<-- "examples/ch11-async/timing/examples/timing_nested.rs"
```

```rust
--8<-- "examples/ch11-async/timing/examples/timing_cancelled.rs"
```

```rust
--8<-- "examples/ch11-async/timing/examples/timing_blocking.rs"
```

## 練習與完整答案：預算在哪一步用完？

整段 two_steps 各使用下列 `(每步毫秒, 整段期限毫秒)`，預測 completed 與外層 Result：`(20, 50)`、`(30, 50)`、`(30, 10)`。每組都從 completed=0 重新開始，使用暫停的測試時鐘。

完整 `tests/timing_solution.rs`：

```rust
--8<-- "examples/ch11-async/timing/tests/timing_solution.rs"
```

```bash
cargo test -p ch11-timing --test timing_solution --locked
```

三組依序留下2、1、0；只有第一組取得 Ok(())。後兩組取得 Err，已完成的步數仍保留。這回收的是被 timeout 直接持有的工作，不需要憑睡了多久猜它有沒有被丟棄。

全頁 source 與答案到齊後，可一起檢查：

```bash
cargo test -p ch11-timing-steps -p ch11-timing-clock -p ch11-timing --all-targets --locked
cargo clippy -p ch11-timing-steps -p ch11-timing-clock -p ch11-timing --all-targets --locked -- -D warnings
```

下一節把「等一份結果的 future」和「已提交的 task」拆開追蹤，再為整批工作訂出停止、取消與回收政策。

[下一節：select!、取消與收尾](14-06-lesson.md)
