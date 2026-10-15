# 第16章v3作者 ledger

狀態：第16章五節全部作者自查freeze；保留原16.1–2 prefix freeze，後半與全章freeze另存。root交接14.1–3後才正式寫作；作者親讀三頁及14.2/3完整Rust/manifests29檔，與交接freeze全匹配。clear-tutorial與兩份寫作/術語指引已讀。

## 研究與source隔離

work/ch16-v3/source-isolation-plan.md及research/read-manifest.json保存17份EN/ZH/API真實ranges/hash/取捨，local-read-manifest.json保存原五頁及完整source/fixtures/tests/7 SVG；actual-prefix-14.json記正式gate。research中的cache-status.json區分實讀/只快取/404。

root接唯一新member ch16-future-stages及lock後才selected Cargo。作者未改rootCargo/lock/nav/CI。無lib/第三方依賴/build.rs；每個target自己的module，16.1–2不載Future/poll/readiness/runner。原ch13-futures全lib及原18tests保留，最早16.5教完所有能力才載。

16.1用已教14.3 controlled_work的owned release收端/recv等待作動機，新增對應關係圖；不依賴未gate14.6。16.2補極小safe std pin!局部安置，完整教macro輸入/結果/借用來源，沒有projection/外部依賴。保存pointer/pointee、Unpin與pre-pin move、Drop guarantee、unsafe getter/Drop完整責任。

## 實際first-use位置

| 正文位置 | 能力／階段 |
| --- | --- |
| 16-01-lesson.md:7 | 同一份工作裡，可能有指向內部的借用 |
| 16-01-lesson.md:19 | lifetime、move 與位置，各回答一件事 |
| 16-01-lesson.md:25 | reference 會擋住衝突的交換 |
| 16-01-lesson.md:43 | 改成地址記錄，先觀察整份值交換 |
| 16-01-lesson.md:61 | 新增的 **`ptr::eq(first, second)`** 接收兩個 pointer、回傳 bool，這次比較兩個 `*const i32` 的地址是否相同。它沒有讀取兩個 i32，也沒有檢查來源資格、存活或存取權限。true 不能單獨證明可 dereference；false 足以看出記錄與目前欄位位置不一致。 |
| 16-01-lesson.md:93 | 換 Box owner，裡面的資料留在哪裡？ |
| 16-01-lesson.md:120 | 練習：只交換數字，地址記錄會失配嗎？ |
| 16-01-lesson.md:148 | 完整測試：內容與位置分開驗 |
| 16-02-lesson.md:7 | Pin 包住 pointer，約定作用在 pointee |
| 16-02-lesson.md:19 | Unpin：這個型別不依賴 pinning 保證 |
| 16-02-lesson.md:25 | **`Pin::new(pointer)`** 接手一份 pointer，回傳包住它的 Pin；safe 版本要求 pointee 有 Unpin。這次接收 `&mut i32`，得到 `Pin<&mut i32>`。 |
| 16-02-lesson.md:27 | **`pinned.get_mut()`** 接手這份 `Pin<&mut T>` wrapper，回傳裡面的 `&mut T`，同樣要求 T 有 Unpin。它沒有取得 T 的 ownership；不符合 bound 時是編譯拒絕，沒有執行期 Err 分支。 |
| 16-02-lesson.md:53 | 讓數字模型不自動具備 Unpin |
| 16-02-lesson.md:55 | 上一節 AddressNote 的 i32 與 raw pointer 欄位都有 Unpin。因此只把它交給 `Box::pin`，仍不能依賴 Pin 禁止移出。現在另建 **PinnedNote**，保留數字與地址記錄，增加一個 marker 欄位。 |
| 16-02-lesson.md:57 | **`std::marker::PhantomPinned`** 本身沒有 Unpin。放進 struct，會使外層不自動具備 Unpin；本例不另手寫 Unpin 實作。它是沒有欄位的 unit struct，值直接寫 `PhantomPinned`。`_pin` 只是本例的欄位名稱，底線表明我們不需要讀取這個標記。 |
| 16-02-lesson.md:71 | 得到 `E0277`，因為 PhantomPinned 沒有 Unpin。這不表示它不能被安置在固定位置，而是 **safe new 不提供這條路徑**。 |
| 16-02-lesson.md:73 | 先放好資料，再記地址 |
| 16-02-lesson.md:75 | **`Box::pin(value)`** 接手 owned T，將它放在 owned heap 空間，交回 **`Pin<Box<T>>`**。這條 safe 路徑不要求 T 有 Unpin；對沒有 Unpin 的 T，後續 safe 介面保留 pinning 限制。 |
| 16-02-lesson.md:77 | owner 上的 **`as_ref()`** shared 借用 wrapper，交回 `Pin<&T>`；**`as_mut()`** mutable 借用 wrapper，交回 `Pin<&mut T>`。它們都借同一份 T，沒有新配或複製一份 T，也沒有直接交出普通 `&mut T`。 |
| 16-02-lesson.md:79 | 接著有兩種 getter。**`get_ref()`** 接手 `Pin<&T>`，交回 `&T`，可 shared 讀取；不能因此移出非 Copy 的 T。**`get_unchecked_mut()`** 則 unsafe 地接手 `Pin<&mut T>`，交回 `&mut T`：作者必須保證不透過它移出被固定的資料，並持續維持 pinning 契約。它給更大的能力，沒有取消責任。 |
| 16-02-lesson.md:81 | method 接收哪一份 wrapper？ |
| 16-02-lesson.md:83 | 沿第3.6節 receiver 的用途，這裡寫 **`self: Pin<&Self>`** 或 **`self: Pin<&mut Self>`**，直接指定 method 接收的 wrapper。Self 仍是目前的 PinnedNote；第一種作 shared 觀察，第二種作受控修改，都沒有接手整份 PinnedNote。 |
| 16-02-lesson.md:94 | Drop 也必須維持原位關係 |
| 16-02-lesson.md:96 | pinning 不只管這次 method。對依賴它的原值，儲存空間在 **Drop 返回或 panic 前，不能先被回收或重用**；回收／重用前先在原位收尾，稱為 **Drop guarantee**。第11.3節的正常清理由 owner 安排；這裡多了位置要求，不是保證任何值最終都一定被清理。 |
| 16-02-lesson.md:98 | **Drop 的 receiver 仍是 `&mut self`，不會自動換成 Pin。** 作者不能利用這份普通 mutable 能力移出地址敏感原值。本模型 Drop 只讀正常數字、比較地址，沒有移出或替換 Self；之後 Box 依正常流程處理空間。 |
| 16-02-lesson.md:106 | 交換 owner，修改數字，最後收尾 |
| 16-02-lesson.md:134 | 固定也能使用局部儲存 |
| 16-02-lesson.md:136 | Pin 本身不等於 heap 配置。標準 **`std::pin::pin!`** macro 接手一份值，把它安置在目前的局部儲存，產生 `Pin<&mut T>`；這個操作不為 T 新建 heap 配置，對沒有 Unpin 的 T 也有 safe 路徑。 |
| 16-02-lesson.md:152 | safe caller 不能取得 mutable reference 交換整份值 |
| 16-02-lesson.md:166 | 練習：交 owner，改值，再 drop |
| 16-02-lesson.md:190 | 完整測試與重借用 |

## prefix驗證與freeze

prefix-verification.json共14筆全部符合預期：7正常程式精確stdout、7原tests（4地址+3Pin）、3個compile-fail逐份rustc實測並比對非空.expected、9個selected example Clippy -Dwarnings、fmt。沒有重跑其他作者/其他章或根全workspace；沒有host UB或永久等待反例。

source-closures逐target列完整roots/helpers/manifest/hash；first-use另存JSON；兩頁markdown/snippets實際全展開。完整manifest/main/module/tests/fixtures/.expected/兩份答案都在同頁，沒有region-only閉包。未實際桌面/手機頁面視覺驗證；SVG source實讀不能代替實際視覺。獨立首讀/技術/銜接及整書harness由root統籌。

prefix凍結hash：

- docs/chapters/16-01-lesson.md: `a02196df700ac59c39a0d5d0a73a457d54661a69110d145086deac890aca84a3`
- docs/chapters/16-02-lesson.md: `e23aa436d2fae0c6d9a0d80d17f053ae3bed168bc40a1cd42ea00fdce2a54aca`

## 16.3–5 final交接

16.3先教Option::take再用，不引用未gate14.9；Waker/Wake/Context與SendSync/static轉換bound完整解釋，手動通知觀察不藏executor。16.4同一guard完成check/register、供值/take，Waker clone/舊handle Drop/wake離鎖；有限模型仍可能等Mutex鎖。16.5前段四target各自最小runner，Recheck與PinnedAnswer教完才載原全library。

原package全16examples、18tests、五負例及兩份獨立fixture manifests/locks永久保全，完整同頁；原source未修改。新增stage無lib；所有五個新故意失敗.rs同名.expected非空且實測核對，fixtures無被harness當負例的子helper。

poll-event-verification.json共13筆全部符合預期：六正常程式精確stdout、八原tests（通知4/事件4）、event_send_once編譯拒絕、runtime-fail先編成功再執行指定panic、八selectedtargets Clippy與selectedfiles fmt。executor-verification.json包含新executor快照先格式化、四stage及四原lib入口精確stdout、原18tests、原四compile-fail/跨crate-fail、原完成後panic先編成功再執行指定失敗、四stage Clippy、原package全已教targets Clippy、selectedfiles fmt。

原Cargo integration panic診斷實際可能在stdout，該條以stdout+stderr比對指定原因，未當任意非零成功。fixture Cargo使用work/ch16-v3/fixture-target避免改舊source；rootworkspace整合仍root維護。所有Cargo/rustc/rustfmt新shell先source env，INCREMENTAL0/DEV及TEST_DEBUG0/JOBS2，只測自身相關selectedtargets；沒有root check/build/test。

全五頁snippets實展開、泛型保持code spans、first-use位置/完整target closure/hash保存；未實際桌面手機頁面視覺，不以SVGsource讀錄冒充。獨立review與全書harness待root統籌；本作者沒有已知未修Rust/snippet失敗。

| 章節 | SHA256 |
| --- | --- |
| 16.1 | `a02196df700ac59c39a0d5d0a73a457d54661a69110d145086deac890aca84a3` |
| 16.2 | `e23aa436d2fae0c6d9a0d80d17f053ae3bed168bc40a1cd42ea00fdce2a54aca` |
| 16.3 | `a95a5c1a963f7dce7c6be06c75e077f696a4dd6ffc01cc5d26f77155452e81d1` |
| 16.4 | `812dd287942e8bdfe9835c8dafe5f0039b76bd2058c0c7c2a12937b411faa1d1` |
| 16.5 | `682ee2b551cda73f3589643ce3f9827570e368d10377759af42a9182fd0755e7` |

### 後半實際first-use

| 位置 | 能力／階段 |
| --- | --- |
| 16-03-lesson.md:5 | 仍用數字80。這份工作先保存80，等我們明確授權才交出；程式分開觀察 **已授權、已通知、已 poll、已交出 Output**，不把四件事混成「完成」。 |
| 16-03-lesson.md:7 | 一次 poll，回答現在能不能交出結果 |
| 16-03-lesson.md:9 | **poll** 是 Future 的一次檢查：借用工作，觀察或推進目前狀態，交回這次結果。沒有答案時應先交還控制權，不能在裡面等到外部條件成立才返回。 |
| 16-03-lesson.md:15 | 呼叫 poll、接住結果並安排何時再檢查的使用端，稱為 **driver**。一般 runtime／executor 負責安排，本節 main 明寫每次 poll。Pending 後，future 還要安排條件改變時的通知；driver 不能把它當成「立刻一直重試」。忙等占用執行機會，條件卻可能沒變。 |
| 16-03-lesson.md:17 | Context 借出通知 handle，Waker 可以保存 |
| 16-03-lesson.md:19 | **`std::task::Waker`** 是通知 handle，一般用來通知 executor：對應 task 應再次取得執行機會。wake 不代替 poll，也不交出 Output。**`Context<'a>`** 則提供這次檢查使用的 Waker；`'a` 連到原 Waker 的借用來源，不是 future 的存活期或工作耗時。 |
| 16-03-lesson.md:23 | - **`Context::from_waker(&waker)`** 借原 Waker 建立 Context，沒有接手 Waker。 |
| 16-03-lesson.md:24 | - **`cx.waker()`** shared 借用 Context，交回它保存的 `&Waker`。 |
| 16-03-lesson.md:25 | - **`waker.clone()`** 建立另一份 owned Waker handle，通知同一對象；沒有 clone 那份 future。 |
| 16-03-lesson.md:39 | 先做一個只觀察通知的回呼 |
| 16-03-lesson.md:41 | **WakeCounter** 只計通知次數，不排程、不呼叫 future、不開背景工作。先把「收到通知」做成看得見的資料，再接工作。 |
| 16-03-lesson.md:43 | **`std::task::Wake`** 是通知回呼 trait，required method 是 **`fn wake(self: Arc<Self>)`**。這次接手的是保活回呼物件的一份 Arc，回傳 unit；body 決定通知時做什麼。 |
| 16-03-lesson.md:45 | 安全的 **`Waker::from(arc)`** 接手 `Arc<W>`，交回 owned Waker。這份 From 實作要求 **`W: Wake + Send + Sync + 'static`**：回呼能通知、可跨 thread 使用、沒有不夠有效的短借用。這是轉換的 bounds，不能當成 Wake trait 自己的全部宣告。 |
| 16-03-lesson.md:67 | 手動授權工作，保留最近的通知位置 |
| 16-03-lesson.md:69 | **ManualAmount** 保存五項狀態：數字 value；是否獲准的 released；是否已交出結果的 completed；最近通知的 `Option<Waker>` waiting；已 poll 次數 polls。constructor 立即建立 owned 工作，沒有執行 poll。 |
| 16-03-lesson.md:73 | 先取走 handle，再通知 |
| 16-03-lesson.md:75 | 這裡新增 **`option.take()`**：mutable 借用 `Option<T>`，交回原來的 owned `Option<T>`，原槽改成 None。原本 Some(waker)，就能取走那份 Waker；原本 None，仍交回 None。它沒有 clone T，也沒有產生工作結果。 |
| 16-03-lesson.md:77 | **`waker.wake()`** 接手 owned Waker，執行通知回呼、回傳 unit。**`waker.wake_by_ref()`** 則 shared 借用 handle 通知，不消耗它；本章稍後會直接使用。兩者都沒有自己呼叫 Future::poll。Wake 的 default wake_by_ref 會 clone Arc，再呼叫 required wake，實作只提供 wake 也可以使用。 |
| 16-03-lesson.md:79 | 本模型的 **`release()`** 先設 released=true，再 take waiting、呼叫 wake。這樣通知後再檢查，才看得到已授權狀態。`is_released()`、`is_completed()`、`poll_count()` 分別 shared 借用並回傳 bool／usize 快照；沒有等待或再 poll。 |
| 16-03-lesson.md:81 | Pending 每次保存目前 Context |
| 16-03-lesson.md:83 | poll 尚未授權時，保存 `Some(cx.waker().clone())` 再回 Pending。**每次 Pending 都保存本次 Context 的 Waker**，取代舊 handle：同一份工作可能改由另一個通知位置負責，不能永遠只記第一次。 |
| 16-03-lesson.md:97 | 兩次 poll 中間，只發通知 |
| 16-03-lesson.md:125 | 事件先到，可以直接 Ready |
| 16-03-lesson.md:145 | 練習：第二次 Pending，該通知哪裡？ |
| 16-03-lesson.md:173 | 完整測試與完成後的界線 |
| 16-04-lesson.md:7 | 兩份 owner，共同保留一份狀態 |
| 16-04-lesson.md:11 | **EventAmount** 由 main 持有，保存一份 Arc，以及 completed、polls；後兩項只屬於這份 future。**AmountEvent** 交給 worker，保存另一份 Arc 及一次供值權限。 |
| 16-04-lesson.md:15 | 下圖藍箭頭表示兩份 Arc 共同保活**同一個 Mutex**，沒有複製兩份 State；polls／completed 留在 main，不是共享欄位： |
| 16-04-lesson.md:36 | 建立一對權限，只供值一次 |
| 16-04-lesson.md:38 | 普通函式 **`amount_pair()`** 建一個 State，交回 owned tuple **`(EventAmount, AmountEvent)`**。`let (mut work, event) = amount_pair();` 各接手一份；這個函式沒有開 thread，也沒有 poll。Arc::clone 只增保活權限，不 clone State。 |
| 16-04-lesson.md:46 | 自訂 **`event.send(value)`** 接收 i32、交回 unit，**取得整份 AmountEvent ownership**，所以同一 handle 只能供一次。它與 std channel 的 Sender::send 不同：前者把答案存進 State，後者稍後只用來把控制數值交給 worker。 |
| 16-04-lesson.md:50 | 每次存取都加鎖，仍可能漏通知 |
| 16-04-lesson.md:61 | 正確 poll 在**同一份 guard** 內完成「取到答案，或在沒有答案時登記這次 Waker」。事件端也在同鎖內「存答案、取通知」。其他 thread 無法插進各段中間，因此只有兩種關鍵先後： |
| 16-04-lesson.md:70 | 先更新，離鎖後才 wake |
| 16-04-lesson.md:80 | poll 取走結果，或保存最新 handle |
| 16-04-lesson.md:84 | 這裡的 **`option.is_none()`** shared 借用 Option，回 bool：None 為 true，Some 為 false，不取走或修改值。本例檢查的是已從 State 取出的 answer，不是解鎖後再偷讀 State。 |
| 16-04-lesson.md:96 | 先 poll，再允許 worker 供80 |
| 16-04-lesson.md:132 | 先供120，再首次 poll |
| 16-04-lesson.md:156 | 練習：外部事件通知最近的使用端 |
| 16-04-lesson.md:180 | 完整測試：回呼能再次取得 State 鎖 |
| 16-04-lesson.md:182 | 四項 tests 核對先 poll／先事件、最新 Waker，以及離鎖回呼。前三項只用已教公開介面；第四項的 **ReadinessObserver** 連到同一 State，另用 Mutex 保存觀察的 `Option<i32>`。 |
| 16-05-lesson.md:3 | 上一節已有外部事件 Future，main 卻仍先 join，再自行 poll。現在把「何時重新檢查」交給一個小型 **executor（執行器）**：它接手一份工作，先 poll；Pending 時等通知，收到後再 poll；Ready 時交回答案並停止。 |
| 16-05-lesson.md:7 | 只有一份工作，通知不必攜帶 future |
| 16-05-lesson.md:11 | 沿13.5，**`mpsc::channel::<()>()`** 回傳 `Sender<()>`／`Receiver<()>`，buffer 保留尚未取走的訊息。wake 就算先於 recv，通知也能留待收取；空 buffer 且 Sender 仍存在時，recv 會 blocking 等訊息，不是一直重新 poll。 |
| 16-05-lesson.md:13 | **ChannelWake** 回呼持有一份 `Sender<()>`，wake 只送 unit，不持有 future、不 poll。unit 有 Send，`Sender<()>` 有 Send／Sync，也沒有短局部借用，因此滿足16.3的 Waker::from 所需 bounds；不是要求 future 也移給 worker。 |
| 16-05-lesson.md:27 | 接手 owned 工作，保留位置，再等待通知 |
| 16-05-lesson.md:29 | 自訂 **`run_one<F: Future>(work: F) -> F::Output`** 沿5.1的泛型及6.4的 associated type：F 是這次呼叫的具體工作型別，bound 讓 driver 能呼叫 poll；`F::Output` 是該實作選定的結果型別。它接手 owned F，正常完成時交回 Output。 |
| 16-05-lesson.md:31 | work 只留在呼叫 thread，沒有移交到 thread::spawn，**不另外要求 `F: Send` 或 `'static`**。通知回呼可以跨 thread，與 future 本身在哪裡被 poll，是兩項不同責任。 |
| 16-05-lesson.md:33 | `Box::pin(work)` 取得 `Pin<Box<F>>` owner；每次 `as_mut()` 借出 `Pin<&mut F>`，交給 poll。這條 safe 路徑**不要求 F 有 Unpin**，也不取得可能移出 F 的普通 mutable reference。 |
| 16-05-lesson.md:35 | Pending 後的 recv 在 **driver 裡、poll 返回以後**才執行，沒有塞進 Future::poll。收到通知回到 loop，再借用同一工作、再 poll。Ready 分支直接 return Output，不再等通知或 poll；局部 owner／Context／Receiver 依正常流程清理。 |
| 16-05-lesson.md:37 | 先讓內層登記，再啟動事件 |
| 16-05-lesson.md:39 | 若一開始就讓 worker 供值，第一次 poll 可能直接 Ready；這合法，卻看不到通知循環。因此用具體 **StartAmount** wrapper 建立受控先後。 |
| 16-05-lesson.md:45 | 控制 send 不等 worker 完成；worker recv 後再 event.send。**控制 channel 傳 i32，通知 channel 傳 unit，是兩份不同 channel。** poll 裡沒有 recv 或 join。 |
| 16-05-lesson.md:85 | 通知早於 recv，仍然能再檢查 |
| 16-05-lesson.md:87 | **Recheck** 只有 i32 與 bool，具備 Unpin。`new(value)` 保存數字，requested 起始 false；第一次 poll 先設 true，呼叫16.3的 **`cx.waker().wake_by_ref()`**，再回 Pending；下一次才 Ready。 |
| 16-05-lesson.md:109 | driver 不需要 F 有 Unpin |
| 16-05-lesson.md:111 | **PinnedAnswer** 保存 i32 與 PhantomPinned，沒有 Unpin 實作；marker 在這裡只驗介面能力，不表示數字模型真的自引用。new 建普通 owned 值，尚未 pin 時仍能交接。 |
| 16-05-lesson.md:131 | 練習：同一分工，改供120 |
| 16-05-lesson.md:155 | 能完成什麼，也要說清不能繼續的條件 |
| 16-05-lesson.md:157 | run_one 的 Waker 保活 ChannelWake 及 Sender。**供值端消失，不代表通知 channel 已關閉**：若 AmountEvent 被丟棄而未供值，future 可能仍 Pending，通知 Sender 卻還活著，因此 recv 可以繼續等待。expect 沒有製造訊息、業務錯誤或逾時；不執行永遠 Pending 的工作來等待它自行結束。 |
| 16-05-lesson.md:161 | 標準 Waker 允許多次通知被合併成一次後續檢查，wake 不是一通知必交一份答案，也不保證別的 task 一定先取得執行機會。這份 driver **只有一份工作，沒有多工作公平排程承諾**。原 State 的短 Mutex 仍可能等鎖；這不是通用效能或完成保證。 |
| 16-05-lesson.md:165 | 接回完整 library，保留所有原測試 |
| 16-05-lesson.md:207 | 完整 library 的四個 executor 入口 |
| 16-05-lesson.md:288 | 真正跨 crate 的失敗界線 |
| 16-05-lesson.md:341 | 完成後再次 poll 的 integration fixture |
| 16-05-lesson.md:371 | **`--no-run`** 讓 cargo test 只編譯、不執行。先以它確認不是編譯失敗，再執行同一個 `--test poll_after_ready` integration target： |
