# 13.5 小型 executor：收到通知，再安排下一次 poll

13.4 已有真正的外部事件 Future，但 main 仍先 join，再自己呼叫第二次 poll。現在要把「何時再檢查」交給一個小型 **executor（執行器）**：它持有一份工作，先檢查一次；Pending 時等待通知，收到後再檢查，Ready 時交回答案並停止。

仍沿用供80的事件與同鎖登記。這次改變的是 driver 的安排，不是外部事件如何保存結果。也可以接回11.1的問題：async 函式產生的工作需要使用端持續推動，頂層工作不能只靠呼叫函式或保存 future 就完成。

## 單工作：通知不用攜帶另一份 future

我們只處理一份 future，所以收到通知時，driver 已知道應檢查誰。通知 channel 傳一個 **unit 值 `()`**，意義是「請再檢查這份工作」，不是數值80、future本身或新的 ownership。

沿10.5，`mpsc::channel::<()>()` 回傳 `Sender<()>`／`Receiver<()>`；buffer保留已送的通知。wake即使早於recv發生，訊號仍可稍後取走。接收空buffer且Sender仍存在時，recv會blocking，讓呼叫thread等待，沒有不斷重跑poll。

| 對象 | 持有與負責什麼 |
| --- | --- |
| executor，也就是run_one的流程 | owned future的pin wrapper、通知Receiver、Waker及Context；只有它呼叫poll。 |
| ChannelWake通知回呼 | 一份`Sender<()>`，wake時送通知；不取得future，也不poll。 |
| 13.4的State／事件端 | 保存數值及最近owned Waker；worker供值、解鎖後叫通知回呼。 |

新的 **ChannelWake** 沿13.3實作required `Wake::wake(self: Arc<Self>)`。unit有Send，Std `Sender<()>`有Send／Sync，而且不借局部資料，因此符合Waker::from所需的Wake／Send／Sync／static。這些能力屬於通知回呼，不要求future也移到worker。

Sender::send只傳訊息，不等待接收方取走，也不為buffer空位等待；不將它的內部實作宣稱為lock-free。若Receiver已清理，send得到Err。這份回呼選擇忽略該Result，因為driver已結束時不用再安排它；不panic、不重試，也不因此取消外部worker。driver仍活動時持有Receiver，正常通知可交進channel。

本節仍在package `ch13-futures`／library `ch13_futures`，新增`runner` module。先引入所有稍後需要的名稱，再給回呼；沒有外部crate依賴：

```rust
--8<-- "examples/ch13-futures/src/runner.rs:imports"
--8<-- "examples/ch13-futures/src/runner.rs:notification"
```

## 接手工作、保留位置、等通知再檢查

自訂函式 **`run_one<F: Future>(work: F) -> F::Output`** 接手owned F，正常完成時交回該工作的Output。沿6.2的泛型及associated type，逐項讀這個新組合：

- F是這次呼叫所用的具體型別，`F: Future`要求它有Future實作，driver才能呼叫poll。
- `F::Output`是該Future實作選定的associated型別，不是F的欄位；例如F是EventAmount時，回傳i32。不同F可以有不同Output。
- work只留在呼叫thread，由它poll，沒有移交到thread::spawn；這份函式沒有額外要求F符合Send或static。

13.2已教`Box::pin(work)`取得owned F、回傳`Pin<Box<F>>`。driver藉它保留工作的位置，再以`pinned.as_mut()`取得當次的`Pin<&mut F>`，交給poll。這條safe路徑不要求F有Unpin；若內部需要地址穩定，也不用取得普通`&mut F`來移走它。後面會用沒有Unpin的有限工作驗這項介面用途。

Pending分支的recv在**driver**內，在poll已返回後執行；不把等待塞進Future::poll。收到通知回到loop頂部，再取得短借用並poll。Ready分支直接return Output，不再等待或poll；函式局部的future owner、Context及通知Receiver依正常流程收尾。

```rust
--8<-- "examples/ch13-futures/src/runner.rs:executor"
```

這裡沒有用通知直接取得答案。即使收到通知後仍Pending，也依Future契約再次安排之後的通知，driver繼續等待；不能把每個wake都當成一次必完成的承諾。若Ready時仍有其他待收通知，driver也立即返回，不為了清完佇列再poll已完成的工作。

### 完成與無法繼續的界線

run_one持有的Waker保活ChannelWake及其Sender。**供值端消失，不等於通知channel已關閉**：13.4的AmountEvent若被丟棄而不供值，future可能仍Pending；通知Sender卻還在，所以recv不會因此自動Err，可以持續等待。expect只核對本例通知端仍存在，不會製造通知、逾時或業務失敗。

本節正常例會確定供值並join。run_one不保證任意Future永遠完成，沒有一般取消／逾時、I/O或timer支援；不要拿永遠Pending的工作來執行「看看會怎樣」。13.4的短State鎖仍可能等待鎖競爭，這份單工作driver也沒有多工作公平排程的承諾。

## 確定先Pending，再啟動供80的事件

若一建立worker就讓它供值，可能在第一次poll前已完成，只看到Ready。這是合法情況，卻看不出本節要驗的通知循環。因此加入一個具體的 **StartAmount wrapper**，把事件的控制Sender跟EventAmount一起交給executor。

`StartAmount::new(work, start, value)`接手EventAmount、`Sender<i32>`及i32，回owned StartAmount。它不啟動thread或傳送；內部start保存`Some(sender)`，value是要交給worker的控制數值。

wrapper的poll先用同一Context檢查EventAmount。第一次內層Pending時，它已完成13.4的通知登記；wrapper才take出控制Sender並send數值，然後返回Pending。start變None，後續Pending不重送；內層Ready則把同一數字交回。worker仍從控制channel收數值，再用AmountEvent供值。**控制channel與通知channel是兩份不同的channel。**

StartAmount的EventAmount有Unpin，Std Sender有Unpin、Option保留這項能力，i32也有Unpin，因此wrapper能用get_mut。`Pin::new(&mut inner.work)`所借的EventAmount同樣符合Unpin，才走得通；不是任意內部Future都能如此投影。

控制send不等待worker完成。worker尚未收時，無容量上限buffer仍保留數值；若worker已丟棄接收端，send會Err，本有限demo的expect就panic，不冒充一般失敗處理。poll內沒有recv或join。這項控制副作用只為建立可檢查的事件次序，仍沒有讓wake自行執行poll：

```rust
--8<-- "examples/ch13-futures/src/runner.rs:start-amount"
```

看下面的分工圖：橘框是事件及回呼，綠框是通知buffer，藍框是executor操作；灰箭頭表操作／訊號的順序，藍色點線則表示executor仍持有同一份future。通知送的是`()`；80留在State，只有poll才能取成Output：

![executor持有同一份pinned future，首次poll登記通知後回Pending；外部worker供80並wake，通知channel保存unit，executor收到通知才再次poll並Ready80停止；通知不攜帶答案或future](../assets/ch13/executor-notice.svg)

### 完整跑一次，再join外部worker

main先建立13.4的一對值與控制channel，worker接手事件handle及控制Receiver。main把work及控制Sender包進StartAmount交給run_one；通知channel則由run_one自己建立。原EventAmount的結果、最近Waker與State鎖約定都沿13.4。

```rust
--8<-- "examples/ch13-futures/examples/executor_event.rs"
```

```bash
cargo run -p ch13-futures --example executor_event --locked
```

```text
executor：poll
executor：Pending，等待通知
executor：收到通知，安排再poll
executor：poll
executor：Ready，停止poll
main收到答案=80
worker已收尾
```

第一次Pending確定發生，因為送控制值在內層EventAmount登記之後。worker的wake可能在driver開始recv之前或之後，buffer都能保留它；這份stdout沒有量測哪一種排程發生。Ready讓main取得80，worker也可能還在離開send的最後收尾，main仍另外join，才說worker已結束。

## 兩個小對照，分開驗通知與Pin

### 通知早於recv，也不會消失

再建一個有限 **Recheck**，只有i32及bool欄位，具備Unpin。`Recheck::new(value)`保存數字、requested起始false；第一次poll先設true、發通知，再返回Pending，下一次poll才Ready。它沒有外部thread，專門驗「通知先於driver等待」的buffer關係。

這裡新增 **`waker.wake_by_ref()`**：shared借用Waker、通知同一位置、回unit，不消耗原handle。本例由`cx.waker()`取得借用後立即通知。ChannelWake只實作required wake仍可使用它：Std Wake提供的default wake_by_ref會clone Arc，再呼叫wake。它仍只送unit，不重新執行Recheck。

```rust
--8<-- "examples/ch13-futures/src/runner.rs:recheck"
```

```rust
--8<-- "examples/ch13-futures/examples/executor_early_notice.rs"
```

執行`cargo run -p ch13-futures --example executor_early_notice --locked`，會有相同的兩次poll循環，最後印出`poll內先通知，main仍收到答案=120`。通知在第一次poll返回前已送入channel，driver之後recv仍取得到；不是依sleep猜訊號已到。

### driver不需要F有Unpin

**PinnedAnswer**保存i32與13.2的PhantomPinned，沒有額外Unpin實作，因此型別不具備Unpin。`PinnedAnswer::new(value)`建立普通owned值；進入pinning約定之前仍可正常移交。marker在這裡只用來驗driver的能力，不表示這份i32模型真的有self-reference。

它的poll直接Ready，不需要Context通知。由`self.as_ref()`取得`Pin<&Self>`，再get_ref取得shared reference、讀i32副本；沒有get_mut或unsafe操作。run_one可以用相同Box::pin／as_mut路線處理它：

```rust
--8<-- "examples/ch13-futures/src/runner.rs:pinned-answer"
```

```rust
--8<-- "examples/ch13-futures/examples/executor_pinned.rs"
```

執行`cargo run -p ch13-futures --example executor_pinned --locked`，只有第一次poll及Ready，最後印出`沒有Unpin的future，main收到答案=0`，沒有進入等待通知的分支。Box::pin的用途是提供不同Future都能遵守的pinned介面，不是每份工作都必須等待。

## 練習：同一分工，改供120

沿第一份外部事件範例，把StartAmount的控制數值改成120，預測兩個channel各傳什麼、poll何時重新執行，以及run_one返回後為何仍要join。先自己完成完整main，不改13.4的EventAmount或AmountEvent。

??? example "完整可執行答案"

    ```rust
    --8<-- "examples/ch13-futures/examples/executor_solution.rs"
    ```

    ```bash
    cargo run -p ch13-futures --example executor_solution --locked
    ```

    控制channel交120給worker，通知channel仍只送unit。兩次poll後run_one交回120，main再join；外部數值改變，沒有改變誰供值、誰通知、誰再次poll的責任。

三項library tests沿上述三個有限關係：外部事件登記後通知、poll內先通知仍被driver取得、沒有Unpin的Future也能執行。它們都只用已教helper；外部worker仍join。沿前文只選本module tests：

```bash
cargo test -p ch13-futures runner::tests --locked
```

回到11章：async函式產生保存執行狀態的Future，await把等待接到外層工作；頂層仍由runtime／executor推動。這份小例子看見了最小分工：Future提供狀態與Output，事件端透過Waker通知，executor保留工作、接收通知並安排poll。

13.1的地址問題也接回來：有些future可能保存內部借用，lifetime有效之外還需要保留資料位置；13.2的Pin約定讓driver使用共同的`Pin<&mut F>`介面。這裡的Box owner保留工作直到正常收尾，不移出被pin的F。通知handle保活通知位置，沒有替future執行、延長任意reference或修正地址。

一般runtime還需安排更多工作及外部資源。這份Std-only executor只負責本章的單工作通知循環，不替第11章使用的Tokio I/O／timer提供完整環境；實際應用仍使用對應runtime。

延伸閱讀：[英文Async Book：Build an Executor](https://rust-lang.github.io/async-book/02_execution/04_executor.html)、[中文對應章](https://huangjj27.github.io/async-book/02_execution/04_executor.html)、[Receiver::recv](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.recv)、[Waker::wake_by_ref](https://doc.rust-lang.org/std/task/struct.Waker.html#method.wake_by_ref)。正文已包含本例需要的機制與界線。
