# 13.3 Poll、Context 與 Waker：通知之後，誰再次檢查？

11.1 已教 Future 的 Output：呼叫 async 函式先取得工作，await 後才取得答案。13.2 又建立了 `Pin<&mut T>` 這項介面約定。現在把兩者接起來：一次檢查若還沒有答案，工作如何保存「稍後請再檢查」的通知位置？通知發生後，又由誰真正繼續執行？

仍用數字80。本節的工作先保存80，等我們明確授權，才交出它；這樣可以自己安排事件次序，不靠時間長短猜工作是否完成。程式會分別觀察**資料已授權、通知已發生、poll 已執行、答案已交出**，這四件事不混成一件。

## 一次 poll，只回答現在能不能交出結果

**poll** 是 Future 的一次檢查操作。它借用工作、觀察或推進當前狀態，交回這次的結果；沒有答案時要先交還控制權，不能在裡面等到條件成立才返回。

**`std::task::Poll<T>`** 是標準的 enum，T 是工作完成時要交出的型別。沿3.1的 enum／match，這裡有兩個 variant：

| 結果 | 當下意義 | 使用端接下來做什麼 |
| --- | --- | --- |
| `Poll::Pending` | 目前還不能交出 Output，沒有帶一份 T。 | 保留工作；等能再取得進展的通知，再安排檢查。 |
| `Poll::Ready(value)` | 工作已完成，這次交出型別為 T 的 value。 | 使用答案，停止再 poll 這份已完成的工作。 |

Pending 不等於 Option 的 None，也不表示業務錯誤。沿11.1，如果 Output 是 Result，`Ready(Err(...))` 仍表示這份工作完成，只是應用結果失敗。本節 Output 選 i32，完成時會取得80。

Future 的使用端稱為 **driver**：負責呼叫 poll、接住結果並決定何時再檢查。在一般 async 程式裡，runtime／executor 安排這項工作；本節用 main 明寫每次呼叫，讓分工可以直接看見。

Future 回 Pending 時，還要安排條件改變後的通知。driver 也不能把 Pending 當作「立刻不斷重試」：忙等會一直占用執行時間，而條件仍可能沒變。收到通知後再次 poll，也不保證一定 Ready；通知表示應再檢查，答案仍由那次 poll 判斷。

## Context 借出這次的通知 handle

**`std::task::Waker`** 是通知 handle。一般 runtime 會用它通知 Context 對應的 task，也就是 runtime 管理的工作單位，需要再次執行；task 繼續時再檢查等待中的 future。wake 的通知動作不直接代替 Future::poll，也不交出 Output。通知之後的檢查仍由 driver 安排。

**`std::task::Context<'a>`** 保存這次檢查所用的 context；本節使用它提供的 Waker 借用。`'a` 連到原 Waker 的借用來源，不是工作從建立到完成所經過的時間，也不是 future 的存活期。

先介紹這三項實際要用的 API：

| 操作 | 輸入、回傳與 ownership |
| --- | --- |
| `Context::from_waker(&waker)` | 借用原 Waker，建立 `Context<'a>`；沒有接手 Waker，來源須在借用使用期間有效。 |
| `cx.waker()` | shared 借用 Context，回傳它所保存的 `&Waker`；仍是借用。 |
| `waker.clone()` | shared 借用 Waker，建立另一份 owned Waker handle，通知同一個對象；不是 clone 那份 future。 |

所以要在 Pending 之後保存通知位置時，保存的是 **owned clone**。Context 與原本那份 Waker 可以結束，這份 clone 仍持有自己的通知 handle；不能把短借用當成可任意延長的來源。

現在才讀標準 Future trait 的完整 poll 宣告。這是介面宣告，實作稍後會填入 body：

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
```

| 宣告中的部分 | 已建立的角色 |
| --- | --- |
| `self: Pin<&mut Self>` | 沿13.2的 typed receiver，接手一份保留 pinning 約定的 mutable 借用 wrapper；沒有接手整份 future。 |
| `cx: &mut Context<'_>` | mutable 借用這次的 Context。`'_` 沿10.6的 lifetime placeholder，請編譯器推導 Context 內的 Waker 借用，不表示 static。外面的 mutable 借用與裡面的 Waker 來源也不是同一項要求。 |
| `Self::Output` | 沿6.2／11.1，由這份 Future 實作選定的 associated type，不是值欄位。 |
| `Poll<Self::Output>` | 這次檢查是 Pending，或帶著 Output 的 Ready。 |

`std::future::Future` 是 trait 的來源；用 `use std::future::Future;` 引入它，才能沿4.1以點號呼叫 trait method。

## 先做一個看得見的通知觀察器

要分辨「收到通知」和「已再 poll」，本節建立 **WakeCounter**，只記錄通知次數。它不排程、不呼叫 future，也不啟動背景工作；main 會自己決定每次 poll。

**`std::task::Wake` trait** 描述通知回呼的動作，必須實作 **`fn wake(self: Arc<Self>)`**。沿13.2的 typed self，這次接手的是保活回呼物件的一份 Arc，不是直接接手 Self。它回傳 unit；body 決定通知發生時要做什麼。

使用安全的 **`Waker::from(arc)`**，就能將 `Arc<W>` 轉成 owned Waker。這份選定的 From 實作要求 **`W: Wake + Send + Sync + 'static`**：Wake 提供回呼；Send／Sync 讓通知可以安全地由不同 thread 使用；static bound 排除不夠有效的局部借用，不表示物件永遠不清理。這些是該轉換的要求，不能混成 Wake trait 本身的定義。

WakeCounter 保存 `Mutex<usize>`。沿10.6，usize 有 Send，Mutex 因而提供 Send／Sync，外層就能由 Arc 共同持有；沒有局部 reference，因此也符合 static bound。Arc 保活同一份計數，Mutex 管理短暫讀改寫，仍是前文的兩層分工。

在 code 前固定觀察器的兩項自訂 API：

- `WakeCounter::new(initial_count)` 接收 usize，建立計數觀察器；本節都從0開始，只有有限的幾次通知。
- `counter.notifications()` shared 借用觀察器，短暫 lock、讀出 usize 副本，再解鎖；取得的是快照，不是在等待 future。

回呼也只在短 lock 中加一。每個 guard 都在方法結尾正常清理，主程式不持有觀察器的 guard 去呼叫 wake／poll。沿10.6，expect 在 poison 時會 panic，不會修好計數；本節小次數正常操作沒有持鎖 panic，也不把這個有限觀察器推成任意大次數的計數器。

本節沿用 package `ch13-futures`／library `ch13_futures`，新內容放在 `polling` module，仍使用標準函式庫。先引入後面要用的名稱，再定義觀察器：

```rust
--8<-- "examples/ch13-futures/src/polling.rs:imports"
--8<-- "examples/ch13-futures/src/polling.rs:observer"
```

main 會保留一份 Arc，再把 `Arc::clone(&counter)` 交給 Waker::from。沿10.6，這個 clone 保活同一份 WakeCounter，沒有複製另一份計數；原 Arc 仍可用來讀通知次數。

## ManualAmount：保存資料、readiness 與最近的 Waker

現在加入同一 module 的 ManualAmount。它只保存五項狀態：

| 欄位 | 角色與起始值 |
| --- | --- |
| value | 要交出的 i32，這次為80。 |
| released | 是否已獲授權交出答案；起始 false。 |
| completed | 是否已由 poll 交出 Ready；起始 false。 |
| waiting | 最近一次 Pending 保存的 `Option<Waker>`，起始 None。 |
| polls | 已執行的 poll 次數，起始0；用來觀察沒有自動再執行。 |

這些欄位沒有保存指回自己的 reference 或 raw pointer。i32、bool、usize 都是 Unpin；Waker 的標準契約也有 Unpin，Option 保存它時保留這項能力，所以 ManualAmount 自動具備 Unpin。實作可以沿13.2直接 `self.get_mut()`，不需要 unsafe getter 或 projection；這個有限模型不用地址敏感型別分散目前的通知問題。

使用前先固定自訂 methods 與兩項通知操作：

| 操作 | 當下的工作與結果 |
| --- | --- |
| `ManualAmount::new(value)` | 普通函式立即建立狀態，回傳 owned 工作；沒有執行 poll，也沒有已完成的 Output。 |
| `work.release()` | mutable 借用工作，先把 released 設 true，再使用已保存的通知 handle；回 unit。這是我們手動安排的事件，不直接交出80。 |
| `work.is_released()`／`is_completed()` | shared 借用，分別回傳兩個 bool 快照；授權與交出答案分開。 |
| `work.poll_count()` | shared 借用，回傳已執行的次數。 |
| `waiting.take()` | 回收11.8的操作：mutable 借用 Option，交回原本的 owned Option，原槽改成 None。本例可以接手那份 Waker；原來 None 就仍回 None。 |
| `waker.wake()` | 接手一份 owned Waker，執行它的通知回呼，回 unit；本例增加觀察器通知數，不執行 poll。 |

poll 尚未授權時，保存 `Some(cx.waker().clone())` 再回 Pending。**每次 Pending 都保存這次 Context 的 Waker**，取代上一份：同一份工作可能換了 driver 的通知位置，不能只在第一次登記。

release 先更新 released，再 take 出 Waker、呼叫 wake。這樣接到通知的使用端再檢查時，能看到新的狀態。這次所有事件與 poll 都由同一個 main 依序操作，沒有同時存取 work 的跨 thread 競爭；外部事件同步會在後續節處理。若先 release、才第一次 poll，waiting 原本 None，不需通知，第一次就能 Ready。

Ready 後 driver 應停止 poll。Future::poll 仍是 safe method，即使 caller 再次呼叫，實作也不能因此造成 UB。標準契約不要求重複呼叫仍得到可用結果；本模型選擇明確 panic，讓不正確的重試可辨認。這不是把所有 future 都宣告為可無限重用。

下面接在同一個 polling module，包含普通 constructor、狀態操作及完整 Future 實作：

```rust
--8<-- "examples/ch13-futures/src/polling.rs:amount"
```

poll 只檢查狀態、保存通知 handle 或交回數字，沒有阻塞等待授權，也沒有忙等迴圈。觀察器計數不在 poll 內取得鎖；release 所呼叫的短回呼也在返回前結束。

## 實際跑兩次 poll，中間只發通知

main 建立 Waker、借它建立 Context，再以 `Pin::new(&mut work)` 取得 mutable wrapper，將 `&mut cx` 交給 poll。每次這份借用結束後，就能以普通 method 檢查 work 的狀態。

```rust
--8<-- "examples/ch13-futures/examples/manual_poll.rs"
```

```bash
cargo run -p ch13-futures --example manual_poll --locked
```

```text
建立工作：poll次數=0，通知=0
第1次poll：Pending，已保存目前通知handle
release後：已授權=true，通知=1，poll次數仍=1，已完成=false
driver第2次poll：Ready(80)
完成後：poll次數=2，停止poll
```

關鍵是第三行：資料已授權、通知也到達，但 poll 次數仍是1，尚未交出 Output。是下一次 main 呼叫 poll 才得到80；WakeCounter 沒有隱藏一個執行器替我們做這件事。一般 executor 會把通知接到待執行工作的安排，本節先把必要責任看清楚。

圖中的框是步驟快照，不是 struct 的實際布局。released／completed／polls 屬於 work；通知數屬於 counter。灰箭頭只表示本程式的操作順序，第三框明確由 driver 再次呼叫 poll；不能把第二框的 wake 當成已執行第三框：

![同一份80工作第一次poll回Pending；release把資料授權並通知一次，poll次數仍為1且未完成；driver再次poll才取得Ready80，次數成2並停止](../assets/ch13/poll-cycle.svg)

Ready 交出 Output，沒有轉移 future 自身的 ownership。main 仍可以讀 is_completed／poll_count，再讓 owned work 正常收尾；停止 poll 與值的 Drop 是兩件事。

也可以把事件順序反過來。`examples/ch13-futures/examples/already_ready.rs` 把數字換成120，先呼叫 release，才第一次 poll；此時 waiting 還是 None，不需通知。它沿用相同的介面，核對「已可交出答案」並不一定要先經過 Pending：

```bash
cargo run -p ch13-futures --example already_ready --locked
```

```text
第1次poll前已授權：Ready(120)
poll次數=1，通知=0，停止poll
```

## 練習：第二次 Pending，該通知哪個位置？

保留同一份80工作，建立兩份從0開始的 WakeCounter，以及各自的 Waker／Context。

1. 先用第一份 Context poll，得到 Pending。
2. 還沒 release，再用第二份 Context做一次受控檢查，也得到 Pending。
3. release後，預測兩份計數及 work.poll_count；最後用第二份 Context poll 取得80，然後停止。

這裡多做一次 Pending 是為了觀察通知位置更新，不是在寫忙等 driver。兩份通知觀察器是不同物件；更換 Context 沒有授權資料，取代的是 waiting 裡保存的 owned handle。

??? example "完整可執行答案"

    ```rust
    --8<-- "examples/ch13-futures/examples/polling_solution.rs"
    ```

    ```bash
    cargo run -p ch13-futures --example polling_solution --locked
    ```

    release後第一份通知為0、第二份為1，poll次數仍是2。driver第3次poll才取得Ready(80)。若只保存第一次的Waker，通知就會送到舊位置，無法維持「請目前的使用端再檢查」這項需要。

## 驗證狀態與完成後的界線

四項 library tests 核對：通知沒有代替 poll／完成、在第一次 poll 前授權能直接 Ready、只有最近一次 Context 的通知位置收到 wake，以及 owned Waker clone 在原 Context 結束後仍能通知。它們只使用上面已教的介面與普通 match，不靠隱藏 executor。命令中的 `polling::tests` 是測試名稱篩選字串，選出本 module 的四項 tests：

```bash
cargo test -p ch13-futures polling::tests --locked
```

最後故意對已 Ready 的同一份工作再次 poll。這份獨立 integration test 使用同一個 library，預期 panic；正常範例都已停止 poll：

```rust
--8<-- "examples/ch13-futures/fixtures/cargo-test-fail/tests/poll_after_ready.rs"
```

沿7.2，`--test poll_after_ready` 選 `tests/poll_after_ready.rs`。新增的 **`--no-run`** 只編譯測試、不執行：先確認不是編譯錯誤，再真正執行、檢查預期的 panic。`--manifest-path` 沿13.2指定獨立 fixture 的 Cargo.toml，其 path dependency 連到本 package：

```bash
cargo test --manifest-path examples/ch13-futures/fixtures/cargo-test-fail/Cargo.toml --test poll_after_ready --locked --no-run
cargo test --manifest-path examples/ch13-futures/fixtures/cargo-test-fail/Cargo.toml --test poll_after_ready --locked
```

第一條應編譯成功；第二條預期失敗，診斷包含 `ManualAmount 已完成，不應再次 poll`。這次驗的是本型別選定的完成後 panic，不是宣稱所有 Future 在這個時點都做相同動作。

現在可以把11.1的 await 接回這項分工：future 提供每次的狀態與 Output，Pending 保留之後通知的責任，通知提示 driver 應再檢查；何時真正再次執行由 driver／executor 安排。下一節把手動授權換成受控的外部事件，再處理等待與通知之間的同步，逐步接到有限 executor。

延伸閱讀：[英文 Async Book：Task Wakeups](https://rust-lang.github.io/async-book/02_execution/03_wakeups.html)、[中文 Async Book：用 Waker 喚醒任務](https://huangjj27.github.io/async-book/02_execution/03_wakeups.html)、[Future](https://doc.rust-lang.org/std/future/trait.Future.html)、[Context](https://doc.rust-lang.org/std/task/struct.Context.html)、[Waker](https://doc.rust-lang.org/std/task/struct.Waker.html)、[Wake](https://doc.rust-lang.org/std/task/trait.Wake.html)。本節的狀態、ownership 與通知責任都在正文，不需靠外部的 timer／executor helper 才能讀懂。
