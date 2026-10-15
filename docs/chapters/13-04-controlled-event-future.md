# 13.4 外部事件 Future：檢查與登記之間，不能漏掉通知

13.3 的 main 直接呼叫 release，再自行 poll。現在仍取得80，但數值改由另一個 worker 供應。main 保有 future，worker 保有一次供值的事件 handle；誰先執行可能不同，main 都應取得同一份結果。新問題是：**main 已看到「還沒結果」，事件卻在它登記通知前到達，之後還有人叫它再檢查嗎？**

本節先實作這份有限 Future，再用 channel 控制事件次序。每次 poll 仍由 main 明寫，worker 都有 join 收尾；通知如何接到 executor 的安排，下一節才教。

## 先分開兩份 owner 與一份共同狀態

13.3 的 ManualAmount 自己保存數字及通知位置，只有 main 修改它。現在 worker 不持有 future，也不取得對它的 mutable reference；兩者沿10.6以 **`Arc<Mutex<State>>`** 共同持有同一份 State，短暫取得 guard 後才能操作。

| 物件 | 持有與負責的內容 |
| --- | --- |
| EventAmount，main 持有 | 一份 Arc、completed 及 polls。提供 Future::poll，記錄是否已交出 Output、已檢查幾次。 |
| AmountEvent，worker 接手 | 另一份 Arc，以及一次供應數值的權限。它不持有 EventAmount，不呼叫 poll。 |
| 共同 State，Mutex 內 | `value: Option<i32>` 是尚未被 future 取走的結果，起始 None；`waiting: Option<Waker>` 是最新通知位置，起始 None。 |

State 中的 Some 表示已有數字，None 表示目前沒有待取數字；不是把 Option 當成業務錯誤。poll 取走數字後，value 又成 None，但 future 自己的 completed 已成 true，driver 必須停止 poll，不能把它當作重新開始的工作。

圖中兩條藍箭頭表示共同 ownership，連到同一個 Mutex，不是複製兩份 State。下方兩行是受保護欄位，不是 compiler 的實際布局；polls／completed 留在 main 的 future，沒有與 worker 共用：

![main持有EventAmount及其Arc，worker持有AmountEvent及另一份Arc，兩者共同保活同一Mutex中的value與waiting；Future的polls和completed留在main](../assets/ch13/event-owners.svg)

本節仍用 package `ch13-futures`／library `ch13_futures`。新內容在 `readiness` module。沿用已教的 Future／Pin／Arc／Mutex／Context／Poll／Waker，再定義這三種型別：

```rust
--8<-- "examples/ch13-futures/src/readiness.rs:imports"
--8<-- "examples/ch13-futures/src/readiness.rs:state"
```

## 建立一對權限，只能供值一次

普通函式 **`amount_pair()`** 不啟動 thread，不執行 poll；它建立一個 State，回傳 owned tuple **`(EventAmount, AmountEvent)`**。`let (mut work, event) = amount_pair();` 分別接手兩個值。Arc::clone 只增一份保活權限，沒有複製 State。

EventAmount 的 `is_completed()`／`poll_count()` 沿13.3的觀察方式，shared 借用後回傳 bool／usize 副本；不等待事件，也不取得 State 的鎖。

```rust
--8<-- "examples/ch13-futures/src/readiness.rs:pair"
```

事件端自訂 method **`event.send(value)`** 接收 i32、回傳 unit，**取得整份 AmountEvent 的 ownership**，因此同一個 handle 只能呼叫一次。它與稍後 channel 的 Sender::send 是不同的 method：事件端把結果寫入 State；控制 channel 的傳送端只是把數值交給 worker。

這份事件 handle 沒有 Clone，State 也不公開給 caller 任意修改。send 返回時，其 Arc 正常清理，future 的 Arc 繼續保活 State。若供應端丟棄 handle 而不供值，沒有替 future 製造 Ready 或 Err，也沒有保證新的通知；這是本有限介面的界線。下方工作流程會確定送一項並 join，不把「供應端已消失」誤當完成。需要回報供應端提前停止的應用，還須另設結束狀態與錯誤結果。

## 同一把鎖，把檢查與登記接成一段

光把每一次欄位讀寫分別放進 Mutex，還不夠。假設錯誤做法把檢查與登記拆成兩次持鎖，便允許這個次序：

1. main 取得 guard，看到 value 是 None，接著解鎖，還沒有登記 Waker。
2. worker 取得 guard，供應80；waiting 仍是 None，於是沒有通知可以發，然後解鎖。
3. main 再取得 guard，只登記 Waker，不重新檢查結果，便返回 Pending。
4. main 開始等通知；結果早已供應，但沒有第二次事件替它通知。

每一筆資料都受到互斥保護，這份工作卻可能停在等待。**漏掉通知**，就是資料變化與通知登記沒有接成可繼續前進的關係。不要執行這個永久等待反例；由上述次序就能指出缺口。

正確做法讓 poll 在**同一份 guard** 內完成「取結果，或在沒有結果時登記這次 Waker」。事件端也在同一鎖內完成「存結果、取通知」。其他 thread 不能插入這兩段各自的中間，所以只有兩種關鍵關係：

| 哪一段先取得鎖 | 為什麼不漏通知？ |
| --- | --- |
| 事件段先供值 | 之後 poll 會讀到 Some，直接 Ready；即使此前沒有 Waker，也不需要等通知。 |
| poll 段先看到 None | poll 在解鎖前已登記 Waker；事件段取得鎖後能取到它，先供值再通知。 |

圖把整個彩色框標成一次持鎖區段。框內沒有跨 thread 插入的空隙；灰箭頭只表示這一種取得鎖的先後，不代表量測時間。最後的「解鎖後 wake」是事件段之外的操作：

![兩種取得同一鎖的次序：先事件存120後首次poll直接Ready；先poll在同一guard內看到None並登記Waker後事件存80取通知，解鎖後wake，再由driverpoll](../assets/ch13/event-order.svg)

### 事件先存結果，解鎖後才叫回呼

沿13.3的 Option::take，`state.waiting.take()` 交出原 owned Option，原槽改成 None。下面的內層 block 以這份 Option 作最後的表達式，交給外面的 waiting；guard 自己沒有被帶出，在 block 結尾正常 Drop、解鎖。之後才對取出的 owned Waker 呼叫 wake：

```rust
--8<-- "examples/ch13-futures/src/readiness.rs:event"
```

為什麼不能持著 State 的鎖去 wake？通知回呼由 driver 提供，它可能再檢查這份共享狀態。如果仍持原 guard，就可能出現10.6的等待循環：回呼要取得原鎖，原鎖卻要等回呼返回才釋放。把結果及通知 handle 更新完，再離鎖 wake，接到通知的使用端也能看到新的值。

### Future 只取結果或保存最新通知

EventAmount 的欄位是 Arc handle、bool及usize，都有 Unpin。**Arc handle 自己的 Unpin 不要求 pointee 的 State 有 Unpin**；這與13.2「pointer wrapper 和被指向值要分開」一致。這份型別沒有地址敏感的內部 reference，所以可以沿13.3用 Pin::get_mut；不是從編譯成功反推所有 Future 都能如此。

新增的 **`option.is_none()`** shared 借用 Option，回 bool：None 為 true，Some 為 false；不取走或修改其中的值。本例用它判斷剛取出的 answer，而不是解鎖後再讀 State。

poll 先在鎖外取得這次 owned Waker clone。內層 block 取得 guard、take 結果、take 舊通知；沒有結果時登記新通知。block 回傳 `(answer, previous_waiting)`，兩項都是 owned 值，不夾帶 guard。舊 Waker handle 的 drop 也安排在解鎖後：資源的清理不混到 State 持鎖區段。每次 Pending 都登記這次的通知位置，沿13.3保留最近 Context 的約定。

```rust
--8<-- "examples/ch13-futures/src/readiness.rs:future"
```

Ready 後仍採13.3的規則：driver 不再 poll；本型別在再次呼叫時明確 panic，safe method 不會因此允許 UB。

這是使用短標準 Mutex 區段的有限模型。沿10.6，**lock 競爭仍可能讓呼叫 thread 等鎖**；此處不宣稱完全非blocking或lock-free。持鎖部分只有短狀態操作，沒有 recv、join、等待事件、I/O或通知回呼。它讓同步關係可見，不用來證明通用 executor 的延遲或吞吐。lock的expect沿10.6檢查有限正常條件，poison仍會panic，不會自動修復資料。

## 先 poll，再准許外部 worker 供80

現在組合10.2的 spawn／join及10.5的 channel。先固定兩種交接的分工：

- `mpsc::channel::<i32>()` 是**控制 channel**：main 持 Sender，worker 接手 Receiver，recv 到80才呼叫 event.send(80)。buffer 會保留已送訊息，不用猜worker是不是先跑到recv。
- work 保留在 main；move closure 接手 Receiver及AmountEvent。i32與Waker有Send，Option保留其Send能力，因此State有Send，Mutex提供Send／Sync、Arc可移交；owned資料不依賴局部reference，符合spawn的static要求。
- 本節仍用13.3的 WakeCounter觀察通知；它不排程。main 在第一個Pending之後才送控制訊息，然後join事件worker，最後明確再次poll。

recv 的等待發生在**worker**；join 的等待發生在**main的driver流程**，兩者都不塞進Future::poll。join確認worker及其通知回呼已返回，不是由join替future交出Output。下方expect只檢查有限正常流程：接收端仍存在、main會送控制值、worker應正常返回；join的標準panic payload也有Debug可供診斷，expect不處理任意thread失敗。

```rust
--8<-- "examples/ch13-futures/examples/event_after_poll.rs"
```

```bash
cargo run -p ch13-futures --example event_after_poll --locked
```

```text
先poll：Pending，已登記通知位置
worker已供值並收尾：通知=1，poll次數仍=1，已完成=false
driver第2次poll：Ready(80)，停止poll
```

第二行確定通知已發生、worker也收尾；future的poll次數仍為1，completed仍false。新State同步沒有改變13.3的分工：仍要driver再次poll，才取走結果並交出Ready。

### 先事件供120，再做首次poll

把事件次序反過來，不需要控制channel：先spawn供應120，再join；這樣首次poll之前，worker必已完成供值。沒有先前Pending，waiting仍None，通知數為0。這次換120是為了分辨兩次獨立工作，介面與同步規則不變：

```rust
--8<-- "examples/ch13-futures/examples/event_before_poll.rs"
```

```bash
cargo run -p ch13-futures --example event_before_poll --locked
```

```text
先事件：已供120，worker已收尾，通知=0，poll次數=0
driver首次poll：Ready(120)，停止poll
```

所以不需要「每個事件都先發一個通知」才能正確完成；已存結果由之後的poll直接讀取。兩份範例用控制訊息及join建立先後關係，沒有以sleep或當次印出順序猜排程。

## 練習：外部事件要通知最新的使用端

沿13.3建立兩份獨立WakeCounter及各自Context，保留同一份80工作與控制channel。

1. 尚未送控制訊息，依序以第一、第二份Context做兩次poll，皆為Pending。
2. 再讓worker供80並join；預測兩份通知數，以及work的poll次數與completed。
3. 用第二份Context再poll取得80，停止poll。解釋同一Mutex保護哪兩種操作，為什麼更換Context沒有供值。

??? example "完整可執行答案"

    ```rust
    --8<-- "examples/ch13-futures/examples/event_solution.rs"
    ```

    ```bash
    cargo run -p ch13-futures --example event_solution --locked
    ```

    worker收尾後第一份通知0、第二份1，poll次數仍2；driver第3次poll才Ready80。事件端取得的是State最後登記的owned Waker，不是固定第一份通知位置。

## 有限驗證：回呼真的能重新取得State鎖

四個tests分別核對先poll後事件、先事件後poll、最新Waker，以及解鎖後回呼。前三項使用上述公開介面與WakeCounter，worker皆join，沒有重複跑大量競爭次數來代替同步理由。

第四項專門建立 **ReadinessObserver**：它的Arc連到同一State，另一個Mutex保存觀察到的`Option<i32>`。Wake回呼先取得State鎖、讀出結果副本、解鎖，再鎖自己的觀察欄位存副本；它不poll。`Option<T>`在T有Copy時也有Copy，因此`Option<i32>`可複製讀出，不取走future的結果。

這個test位在readiness的子module，沿module私有性規則可讀父module的私有State及work.shared；它沒有把State公開給library caller。下面只摘該helper及完整test，接在該test module的imports之後；所用名稱皆為上述型別與已教的Std API：

```rust
--8<-- "examples/ch13-futures/src/readiness.rs:callback-probe"
```

它在callback內實際再次取得同一鎖，讀到事件已供的0；返回後worker完成join，main再poll仍能取到0。這核對本實作的「先存值、解鎖後wake」關係；同步理由仍來自前面同guard區段，不是單憑test通過推得所有排程都安全。沒有執行「持同鎖wake」的永久等待版本。

沿13.3的測試名稱篩選，只選本module四tests：

```bash
cargo test -p ch13-futures readiness::tests --locked
```

最後以同一library的獨立fixture確認一次供值權限。第一個send取得event ownership，第二次再用同一名稱應被拒絕：

```rust
--8<-- "examples/ch13-futures/fixtures/cross-crate-fail/examples/event_send_once.rs"
```

沿13.2，`--manifest-path`選獨立fixture、`--example`選完整target；只編譯檢查，不執行：

```bash
cargo check --manifest-path examples/ch13-futures/fixtures/cross-crate-fail/Cargo.toml --example event_send_once --locked
```

預期編譯失敗含`E0382`。這是我們選的按值send介面，不是宣稱std channel的Sender只能傳一次；10.5的Sender::send借用傳送端，仍可送後續訊息。

本節driver使用join控制收尾與再檢查的位置，尚未由通知安排執行。丟棄future也不強制停止外部thread，本例保留handle並join。下一節才把通知接到有限executor的安排，回收同一份Future與狀態約定。

延伸閱讀：[英文Async Book的事件通知範例](https://rust-lang.github.io/async-book/02_execution/03_wakeups.html)、[中文對應章](https://huangjj27.github.io/async-book/02_execution/03_wakeups.html)、[Mutex的lock契約](https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.lock)、[Waker跨thread通知](https://doc.rust-lang.org/std/task/struct.Waker.html)。本節需要的同步理由、通知位置與driver責任都在正文。
