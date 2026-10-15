# 14.3 spawn、JoinHandle 與 JoinSet：執行工作並收回結果

上一節保存 future，再由同一段程式 await。現在要把小計80的工作交出去，讓它先進入本體；main 收到「已開始」的通知後，才決定何時放行，以及何時等待結果。之後再把同樣的控制擴成四份工作，保留每份結果原來的輸入位置。

## task 已提交，不等於已執行到某一行

**task** 是交給 runtime 排程的工作。**`tokio::spawn(work)`** 接手一份 future，把它提交為 task，交回 **`JoinHandle<T>`**；T 是工作完成的 Output。runtime 能在我們尚未 await handle 前推進 task。spawn 本身不會同步進入該工作的本體，也不保證工作最後一定完成；runtime 結束時，尚未完成的 task 會被丟棄。

因此，「呼叫 spawn」不能證明工作已到達某一行。我們要工作親自送 started，controller 收到才往下走。這與第13章用 channel 確認進度的理由相同。

spawn 要在 Tokio runtime 內呼叫。本章仍用 current_thread，所以這些 task 共用該 thread，不能以建立多份 task 就宣稱 parallelism。這個 spawn API 仍要求 future 與 Output 符合 **Send + `'static`**：第13.1節的 Send 允許跨 thread 交付；`'static` bound 表示不攜帶只在短範圍有效的借用，不要求值必須活到程式結束。owned String 可以符合。即使這次選單一 thread，使用的 API 契約也沒有因此免除這兩項限制。

## 用兩條 channel 分開開始與放行

先使用只含基本工作的 `ch11-tasks-basic`。完整 manifest 加上 **`sync` feature**，提供 Tokio 的同步工具：

```toml
--8<-- "examples/ch11-async/stages/tasks-basic/Cargo.toml"
```

本例用 **`tokio::sync::mpsc`**，名稱同樣是 multiple producer, single consumer（多個送端、單一收端）。它與第13.5節 std channel 的用途相近，這次 send／recv 是可 await 的 async 操作。

**`mpsc::channel::<T>(capacity)`** 建立有界緩衝區，交回 `Sender<T>` 與 `Receiver<T>`；capacity 必須大於0。Sender 可以 clone，共用同一條 channel；Receiver 由一個接收者持有。`sender.send(value).await` 等待空位，成功交回 Ok(())，收端已關閉則交回帶原值的 SendError；它具備 Debug，可供 expect 在失敗時產生診斷。`receiver.recv().await` mutable 借用收端：有訊息得到 Some(value)，暫時沒有訊息但仍有送端時繼續等，所有送端結束且緩衝區已空才得到 None。

本節用 unit `()` 作不帶額外資料的通知，兩條 channel 的角色固定如下：

| 訊息 | 誰持有送端 | 收到代表什麼 |
| --- | --- | --- |
| started | 工作 | controller 確認本體已執行到送出通知 |
| release | controller | 工作獲准往下計算或製造指定失敗 |

放行訊息沒送、送端仍活著時，工作不能越過 release.recv；不是等了某個毫秒數就假定工作已開始。

工作 Output 是 `Result<i32, WorkError>`。**Negative(amount)** 表示小計規格不合，**ControlClosed** 表示尚未放行，release 的所有送端卻已結束；這兩者都由本體正常 return。測試另可要求放行後 panic，稍後用來區分工作錯誤與 task 失敗。

完整 `src/lib.rs` 只有 WorkError 和 controlled_work，沒有後面才要教的工作集合：

```rust
--8<-- "examples/ch11-async/stages/tasks-basic/src/lib.rs"
```

started 的 expect 有明確前提：本例 controller 會保留收端直到收妥通知。送完就 drop 工作自己的 started 送端；release 收到 None 則提前交回 ControlClosed，不會執行後面的計算。完整預設 `src/main.rs` 仍只提示選例：

```rust
--8<-- "examples/ch11-async/stages/tasks-basic/src/main.rs"
```

## JoinHandle：等回工作，也等回失敗

**`handle.id()`** 取得 task 的 **Id**，可用於辨識與顯示；它不是我們的輸入索引，也不應假定某次固定印出幾號。Id 也支援相等比較與 Hash，稍後可用作 HashMap 的鍵。**`handle.is_finished()`** 查詢 task 此刻是否已結束，沒有等待它；false 只描述當下。

**`handle.await`** 取得 `Result<T, JoinError>`。JoinError 具備 Debug 與 Display，因此可用診斷格式顯示，也能沿第8.5節以 to_string 保存文字。本例 T 已經是 `Result<i32, WorkError>`，所以有兩層：

| 取得的值 | 意義 |
| --- | --- |
| Ok(Ok(80)) | task 正常交回成功的工作結果 |
| Ok(Err(WorkError::Negative(-1))) | task 正常交回工作錯誤 |
| Err(join_error) | task 因 panic、取消等原因沒有正常交回 Output |

完整 `examples/join_handle.rs` 先等 started，確認還沒放行所以未完成，再送 release，最後 await handle：

```rust
--8<-- "examples/ch11-async/stages/tasks-basic/examples/join_handle.rs"
```

```bash
cargo run -p ch11-tasks-basic --example join_handle --locked
```

先印「收到啟動訊息；還沒有 await JoinHandle」，最後得到80。這次有訊息作為本體已開始的證據，與上一節只建立一份尚未執行的 async future 不同。await handle 負責收回結果，並不是 task 唯一能開始執行的時點。

## abort 後仍要 await

**`handle.abort()`** 請求取消這份 async task，沒有等清理完成；接著 await handle，才取得它的結束結果。JoinError 的 **`is_cancelled()`** 辨認取消，**`is_panic()`** 辨認 panic，**`id()`** 仍能對回原 task。已完成的工作不一定因後來 abort 改成取消；本例刻意保持 release 未送，使工作仍在等待。

完整 `examples/abort_and_wait.rs`：

```rust
--8<-- "examples/ch11-async/stages/tasks-basic/examples/abort_and_wait.rs"
```

```bash
cargo run -p ch11-tasks-basic --example abort_and_wait --locked
```

等到取消後，task 持有的 release 接收端已清理，再 send 會得到 Err。這個結果比「已呼叫 abort」多了一步完成證據。

若直接 drop JoinHandle，則是 **detach（放開這份等待把手）**：task 可繼續執行，並不等於 abort，也沒有收回結果。runtime 結束又是另一個邊界。需要結果或清理完成的證據時，應保留把手並等待。

## 多份工作完成時，怎樣找回原輸入？

下一個問題仍用相同 started／release，改成四筆輸入。main 要按受控完成順序收報告，並且知道每份報告原來是哪一筆。

**`JoinSet<T>`** 持有一組 Output 同為 T 的 task。`JoinSet::new()` 建立空集合，**`tasks.spawn(work)`** 提交並納入一份工作，交回 **AbortHandle**，可查 id 或請求 abort；它不是用來 await 結果的 JoinHandle，結果改由集合收回。

**`tasks.join_next_with_id().await`** mutable 借用集合，等待並取出一份已完成的 task，得到 **`Option<Result<(Id, T), JoinError>>`**。空集合交回 None；非空但尚未有完成項目就等待。成功時帶 id 與完整 Output，JoinError 則透過 error.id 找 id。這不是輸入順序；多份都已完成時，也不要憑提交順序猜下一筆。

我們用第6.5、6.6節已教的 HashMap，保存成 `HashMap<Id, usize>` 保存「task Id → 原輸入位置」。加入工作時 insert；回收後用 **`inputs.remove(&id)`** mutable 借用 map，刪掉那組資料並取得 `Option<usize>`。Some 是已移出的輸入位置；None 代表沒有那個鍵。本例每次 spawn 都先登記，因此用 expect 檢查這項內部約定。完成後 map 應空，不會留下已回收的工作。

**Outcome** 保存四種主要分類：Value、WorkError、Panic、Cancelled；OtherTaskFailure(String) 則保存其他 task 失敗訊息。**Report** 將 input 和 outcome 放在一起。這是我們的報表政策，不是把所有 Err 都當成同一件事：

```rust
--8<-- "examples/ch11-async/tasks/src/lib.rs:outcome-model"
```

**collect_one** 先處理 Option，再解開 task 與工作的兩層 Result，最後移除對照並交回報告：

```rust
--8<-- "examples/ch11-async/tasks/src/lib.rs:collect-one"
```

## 固定控制順序，觀察四種結果

run_demo 為每筆建立獨立 release，started 共用一條容量4的 channel。四份 task 都送完通知並 drop 自己的 started 送端，controller 也 drop 原送端；因此 recv 迴圈在收妥四筆後才會 None。

| 原輸入 | 材料與控制 | 取回順序與分類 |
| --- | --- | --- |
| 0 | 80，最後放行 | 第4份，Value(80) |
| 1 | −1，先放行並收回 | 第1份，WorkError(Negative(-1)) |
| 2 | 120，指定放行後 panic | 第2份，Panic |
| 3 | 200，不放行而 abort 並收回 | 第3份，Cancelled |

每一步先讓唯一指定的工作完成，再回收，才得到這個固定順序。panic 可印出診斷，但 controller 仍能處理 JoinError，繼續回收其他工作。

以下是完成版 `ch11-tasks` 的**完整設定與 library**，包含 WorkError、controlled_work、所有報表 helper、run_demo 與兩項測試。測試分別確認開始證據及四種結果；沒有尚未提供的測試 helper：

```toml
--8<-- "examples/ch11-async/tasks/Cargo.toml"
```

```rust
--8<-- "examples/ch11-async/tasks/src/lib.rs"
```

完整 `src/main.rs`：

```rust
--8<-- "examples/ch11-async/tasks/src/main.rs"
```

```bash
cargo run -p ch11-tasks --locked
cargo test -p ch11-tasks --lib --locked
```

stdout 固定為：

```text
input 1：WorkError(Negative(-1))
input 2：Panic
input 3：Cancelled
input 0：Value(80)
四份 task 都已回收
```

stderr 包含 `task 故意 panic`；整體 binary 仍正常退出，這是 main 接受該筆 Panic 報告的政策。若只看到程序退出成功，不能據此說每份工作都成功。

本完成版保留剛才兩份基本 example，改用完成的 library。以下是實際會載入的完整檔案：

```rust
--8<-- "examples/ch11-async/tasks/examples/join_handle.rs"
```

```rust
--8<-- "examples/ch11-async/tasks/examples/abort_and_wait.rs"
```

完整 `tests/cli.rs` 以真正 binary 驗 stdout 與 panic 診斷，Command／env! 的契約沿用上一節：

```rust
--8<-- "examples/ch11-async/tasks/tests/cli.rs"
```

```bash
cargo test -p ch11-tasks --test cli --locked
```

## 練習與完整答案：沒有放行，控制端卻結束

只提交 input0、金額80，不指定 panic。收到 started 後，不送 release，改成 drop 它唯一的送端。使用 collect_one 收回報告，再確認集合沒有下一份結果、inputs 已空。這應是 WorkError::ControlClosed，還是 JoinError 的 Cancelled？

完整 `examples/control_closed_solution.rs`：

```rust
--8<-- "examples/ch11-async/tasks/examples/control_closed_solution.rs"
```

```bash
cargo run -p ch11-tasks --example control_closed_solution --locked
```

工作自己的 recv 得到 None，接著正常 return Err(ControlClosed)，所以報告是 `WorkError(ControlClosed)`。它沒有被 abort；外層 task 結果是 Ok，內層工作結果才是 Err。此時 inputs 與集合都已清空，控制端沒有只記錄失敗卻遺漏回收。

全部 source 與答案已到齊，可驗證兩個 package 的所有 target：

```bash
cargo test -p ch11-tasks-basic --all-targets --locked
cargo test -p ch11-tasks --all-targets --locked
cargo clippy -p ch11-tasks-basic -p ch11-tasks --all-targets --locked -- -D warnings
```

JoinSet 被 drop 會請求取消其中所有 task，但 drop 本身不能提供逐筆結果。下一節保留顯式回收，再限制同時提交且尚未回收的工作數：一份回來，才補下一份。

[下一節：限制同時在途工作](14-04-lesson.md)
