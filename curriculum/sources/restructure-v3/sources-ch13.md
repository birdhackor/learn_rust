# 第13章來源實讀與取捨

這是repo內作者來源紀錄，不是學生正文。2026-10-15 親讀下列原文；舊 inventory 的人工結論只用來定位，沒有當作已讀原文。原始 HTML、擷取正文、URL、擷取時間、完整 SHA-256 與逐份實讀行範圍保存在 `external-sources/sources.json`。本次沒有執行任何外部或專案 Rust 範例，也沒有宣稱來源網站視覺驗證。

## 範圍與來源責任

已實讀 44 份原文：Book 第 16 章英文與繁中五頁逐頁對讀、Comprehensive Rust scoped threads、Nomicon HRTB 英中、Reference HRTB、Mara Bos memory ordering 的相關段落，以及實際範例使用的 std API。Book 的 thread 頁沿用既有下載快取，本次仍親讀完整原文。官方大型 API 頁只讀本章採用的項目與 trait implementation 範圍，不聲稱全文讀完。

Root 起初將 Condvar／Barrier／thread-local API 誤列為必保全，後來親核 Astra 核准 outline 後明確撤回。撤回前已下載的四份 std 文件保留歷史紀錄，但沒有閱讀，不計入上述 44 份，也不作教材依據。另一次 course.rs scope URL 失敗保留原紀錄，不視為已讀。保留舊 scoped 章原有的「thread body 完成與 TLS destructor 清理邊界」概念，不新增 TLS API。

## 各節採用與不採用

### 13.1 Send／Sync

依據 `book-sendsync-en/tw` 與 `std-send`、`std-sync`：先區分傳送值與跨執行緒共用參照，再用既有 `require_send`／`require_sync` 實際驗證。`&T: Send` 對應 `T: Sync`；`Cell<T>`／`RefCell<T>` 的 Send 有條件，不能直接說所有 Cell 都能傳送；Rc 不因放進 move closure 就變成 Send。Fn 呼叫能力、Send／Sync 與 `'static` 各回答不同問題。

不照 Book 的出場順序等到執行緒章末才介紹這兩個 trait；本書核准順序先教契約，後解 `spawn` 簽章。也不把「所有權規則通過」說成排除了邏輯錯誤、死結或效能問題。

### 13.2 thread／join／panic

依據 `book-threads-en/tw`、`std-spawn`、`std-joinhandle`、`std-thread-result`、`std-panic`、`std-any`。採用 Book 對比 join 放置位置與 move 捕獲的觀察方式；各 handle 都先妥善 join，再處理結果分支，避免看到第一個 Err 就把其他工作留成 detach。`JoinHandle::join` 消耗 handle；工作閉包與回傳值各有 Send／`'static` 條件，立刻 join 不會放寬 `spawn` 型別契約。owned 值符合 `'static` 不代表它永遠不會被釋放。

panic hook 的輸出與 join 收到的 payload 分開說明；一般 panic payload 可能是 String 或 `&'static str`，不能由表面訊息認定具體型別。`Box<dyn Any + Send>` 的第一次 downcast 失敗回傳原 Box，才可再嘗試第二種型別，最後處理非文字 payload。保留原測試 String、static str、`7_u32` 三條分支。panic／unwind 政策依實際前文契約再確認，不補 panic hook／panic_any／外部 unwind 教學。

不把 sleep 或輸出交錯當成平行度、排程公平性、同步順序或效能證明。Book 部分對 `'static` 的「程式全程有效」措辭只用於真正借用資料的來源，不套成 owned String 必須活到程式結束。

### 13.3 HRTB callbacks

依據 `nomicon-hrtb-en/zh`、`reference-hrtb` 與舊完整 source。區分外側由呼叫者提供的一個 `'a`，以及 `for<'a>` 要求 callback 能接受每次呼叫的輸入生命期。`with_local<F, T>` 的 T 在 binder 外，不可藉此傳出該次 local 的借用，但仍可能攜帶其他外部來源的借用。FnOnce／FnMut／Fn 與對所有輸入生命期成立是兩個維度。

`sum_selected` 的契約只把該次輸出參照連回該次輸入，沒有選定 tuple 哪個欄位；真正選第二欄的是呼叫端 `callback_projection`。這會直接處理 audit A0610-C04。

借鑑 Nomicon 用展開前後對照凸顯 binder 的教學方式，但不照抄為說明用的 Fn 偽實作、nightly 語法或帶 lifetime 的借用運算式，也不說 closure 在「無限次執行」。沿用舊自足的 caller/local/projection 三段實際 source，先講每段所需契約才列程式。

### 13.4 scoped threads

依據 `comprehensive-scope`、`std-scope`、`std-scope-context`、`std-scoped-handle`。採用可借用外部 slice 的小例；先分清呼叫者執行 scope callback、worker 才另開 thread，callback 本身沒有 Send 門檻，worker 的 F／T 有 Send + `'scope` 門檻。`'env: 'scope` 並不容許任意借用 callback 內較短的 local。

std 對清理的精確界線優先於教材概略的「全部等待完成」：scope 自動等待保證 body 完成，TLS destructor 可能仍在執行；`ScopedJoinHandle::join` 則等到該 thread 完整結束，包含 TLS destructor。僅保留這個既有角色，不介紹新的 thread-local API。Scoped handle 被 drop 仍由 scope 追蹤等待，普通 JoinHandle 被 drop 則 detach。手動 join 已接住的 panic 與未接住而由 scope 自動傳出的 panic 分清；scope 等待不提供業務資料回滾。

### 13.5 channel／bounded channel

依據 `book-messages-en/tw`、`std-channel`、`std-sync-channel`、`std-sender`、`std-receiver`、`std-sync-sender`、`std-send-error`。採用 Book 的 message 所有權、clone sender、多 producer 例，但補齊 closure 精確條件：所有 Sender clone 都 drop，且緩衝已耗盡後，recv 才因斷線得到 Err。`Sender<T>: Clone` 不要求 `T: Clone`；Receiver 不可 Clone，也不是 Sync。

send 成功不代表對方已處理工作；容量 0 的 rendezvous 保證交接成功時已接收，仍不保證處理完成。bounded 的容量限制緩衝，不是整個程式所有工作與記憶體的上限。先 recv 再 join 避免互相等待；drop receiver 只是使後續 send 失敗，並非強制取消正在運行的 thread。SendError 的 `.0` 交回第一次未送出的值，不會自動交回整個剩餘 Vec。不要照搬 Book 對「任一端 drop 即關閉」過度簡略的概括。

### 13.6 Arc／Mutex／poison

依據 `book-shared-en/tw`、`std-arc`、`std-mutex`、`std-mutex-guard`、`std-poison-error`。採用先看單執行緒 guard 再加 Arc 的順序，補清 guard 的實際 scope／drop 時點；最後一次讀 guard 不等於立即解鎖。`Mutex<T>` 的 Send／Sync 以 `T: Send` 為條件，Arc 的跨 thread 能力另有自己的條件；MutexGuard 不可 Send。

重要不採用項：Book EN 擷取行 58–60／TW 行 26 的 poison 解說寫成其他 thread 不能再取得鎖。std 明確指出 PoisonError 內已包含取得的 guard，可用 `into_inner` 恢復。沿用舊章已正確的解說：poison 是建議性的損壞警示，取得鎖、修復資料、clear_poison 是不同動作；沒有 poison 也不能證明資料邏輯正確。panic join 與鎖的 poison 分開處理，不能以強制 unwrap 作常見業務政策。

### 13.7 RwLock／downgrade

依據 `std-rwlock`、`std-read-guard`、`std-write-guard` 與舊例。保留讀寫權限、try 的 WouldBlock／Poisoned、reader panic 不 poison／writer panic 可能 poison、釋放 read 後重新取得 write 必須重新檢查條件。RwLock 需要共讀，不能把 Mutex 可放 Cell 的條件套過來。

`RwLockWriteGuard::downgrade` 自 1.92 穩定，消耗 write guard，原子地改為 read guard，沒有先放鎖的空窗；它不是 read 升級 write。不得保證 OS 的公平性、reader 優先、無飢餓或讀多即更快；不得持有 worker 所需的 guard 再 join worker。原受控的先後視窗範例解釋條件變更，不改成 sleep 猜時間。

### 13.8 全部五種 Ordering

依據 `std-atomic-model`、`std-ordering`、`std-atomic-usize/bool`、`std-atomic-concrete-methods`，以及 Mara 指定段落。保留逐操作合法值：load 可 Relaxed／Acquire／SeqCst；store 可 Relaxed／Release／SeqCst；fetch_add 五種皆可。fetch_add 是 read-modify-write 並回傳舊值，普通 load 再 store 是兩個動作；Atomic 不可 Copy。範例數值不溢位，仍核實 fetch_add 的 wrapping 契約。

Relaxed 保留同一 atomic 位置的修改順序，不跨位置發佈資料；所有 worker join 後的最終計數由完成同步支撐。DATA／READY 一次性發佈的結論須列條件：唯一 producer、資料寫入先於 Release READY、Acquire 讀到那次發佈、資料後續沒有其他寫入。join 若放在 consumer 已讀取之後，不能倒過來證明先前那次讀取。Mara 的 read-from 段落用來防止把「讀到同樣數字」誤當「讀到指定那次寫入」。

SeqCst 的單一順序只涵蓋相關 SeqCst 操作，四步循環推導可排除雙方都讀 0；它不會把兩次 load／store 合成 RMW，原 deterministic split 例仍可留下 1。測到某輸出不是所有合法執行的證明，沒測到也不是不可能。`yield_now` 不保證公平性或進度；一次性 busy-wait 若 producer 失敗可能永遠等待，實務等待可用已教 channel／join。

不新增 release sequence、fence、unsafe shared payload、硬體 cache flush 或完整 C++ 模型教學。當前 live std 的 AtomicUsize／AtomicBool 文件轉為 generic Atomic 的 alias，已核對具體方法；教材仍使用既有穩定具體名稱，不引入該頁標示為 experimental 的 generic Atomic／AtomicPrimitive。當前文件閱讀不代替將來 pinned 1.99 的實測。


## 實讀原文指紋

逐份range是保存TXT的一基底inclusive行號；完整HTML及TXT兩種hash另存ch13-preparation/external-sources/sources.json。本表保留完整HTML hash，來源網站頁面未視覺驗證，來源範例未執行。

| 原文 | 保存TXT已讀行 | 原始HTML SHA-256 |
| --- | --- | --- |
| [book-intro-en](https://doc.rust-lang.org/book/ch16-00-concurrency.html) | 1–43 | `0ab0d6759cf1760eb7f9747fe1a318d9b520a11b7ac36d6defdd2fcadebfaf5e` |
| [book-intro-tw](https://rust-lang.tw/book-tw/ch16-00-concurrency.html) | 1–10 | `6be75b56975ea59d2fa2a2560fd6ece298fd7705cbc028e2e6d2e6984efecf07` |
| [book-threads-en](https://doc.rust-lang.org/book/ch16-01-threads.html) | 1–291 | `55f8ef0e16bbaa3a1195118a3219ae859d2e136355697b19bf44aa52f1ccef08` |
| [book-threads-tw](https://rust-lang.tw/book-tw/ch16-01-threads.html) | 1–194 | `d6297aababa423817103458ec60dff7fdcaf57f3c5e680dd386a23593761f078` |
| [book-messages-en](https://doc.rust-lang.org/book/ch16-02-message-passing.html) | 1–262 | `2d6c139981aa3b1ec012d21c09d8be88b6fe74c3c79c303238eae69399c896f1` |
| [book-messages-tw](https://rust-lang.tw/book-tw/ch16-02-message-passing.html) | 1–166 | `f583a7bfd16da3cd060fa200f3a2317787d38a3db94c03c71a2e7cf8536c13f1` |
| [book-shared-en](https://doc.rust-lang.org/book/ch16-03-shared-state.html) | 1–270 | `4513cce93535d39ba09caaf9d5a35dda5a59120244b0f21371687e73f20415d5` |
| [book-shared-tw](https://rust-lang.tw/book-tw/ch16-03-shared-state.html) | 1–152 | `092881807128a4b428a671ba4f960ee01499c328a66f6d3c555990d6040c4ead` |
| [book-sendsync-en](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html) | 1–68 | `51b894d2e176aaac4baae2a6dc7cbd9cf74a0d863e2dc8a2b7fa49cff81a1e3c` |
| [book-sendsync-tw](https://rust-lang.tw/book-tw/ch16-04-extensible-concurrency-sync-and-send.html) | 1–18 | `a139481c26c055d60bfa50eb38a2264ad72f30863b35f8858e83b83fdf436983` |
| [comprehensive-scope](https://google.github.io/comprehensive-rust/concurrency/threads/scoped.html) | 1–34 | `bd18f81d4e3e7a8a750845ecfa9e0c22bca92a8af016de3098401f91894bf3d0` |
| [nomicon-hrtb-en](https://doc.rust-lang.org/nomicon/hrtb.html) | 1–72 | `d5f84217d703b3271e7b88ed91cd5af19713b91a0bbf5dcb512590ba6cb897d5` |
| [nomicon-hrtb-zh](https://nomicon.purewhite.io/hrtb.html) | 1–62 | `98949926b12199ba77c29f1045ae9288329a0bcb02de78e91d498e9d6646ec6e` |
| [reference-hrtb](https://doc.rust-lang.org/reference/trait-bounds.html) | 128–166 | `3d9474d20502592473716b440703e222bbaeb94d4cce72583e5acdb83acc1d4c` |
| [std-send](https://doc.rust-lang.org/std/marker/trait.Send.html) | 1–70、178–191、216–222 | `4f1c6beb69e6cb3cea37f0af6965b5e7b94f67c3a2a7b243e6f3c4c241e440e8` |
| [std-sync](https://doc.rust-lang.org/std/marker/trait.Sync.html) | 1–90 | `211d6eef06d382f4be667c372f9fd84812012dde032e16c0b59d32d81677b71d` |
| [std-spawn](https://doc.rust-lang.org/std/thread/fn.spawn.html) | 1–80 | `5c5de4b0bb1e3b5b1af0353f92e224f490494568602c18146ec0c277b0a85d5a` |
| [std-joinhandle](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html) | 1–130 | `9025687b86d29545d385fc69fd1f724a268f2199802aa5d4e23a6e53d2ea86ac` |
| [std-thread-result](https://doc.rust-lang.org/std/thread/type.Result.html) | 1–44 | `391c382af5a77c2d478825bad57020c1e5f2ba1c7b697bc92420d196404b53f0` |
| [std-scope](https://doc.rust-lang.org/std/thread/fn.scope.html) | 1–54 | `fe2d8333925772173c4fdcd9ea611b42b9f5ad960b537b9c550b923b6a4e15b4` |
| [std-scope-context](https://doc.rust-lang.org/std/thread/struct.Scope.html) | 1–100 | `a7dfb499c580b2ce6535dc806d0df697eb34fc111c4a40b2b5a6591a2fb1fc36` |
| [std-scoped-handle](https://doc.rust-lang.org/std/thread/struct.ScopedJoinHandle.html) | 1–110 | `d297333f46227a07187cbc437f40604451292ef0708e3660418f37469466d075` |
| [std-yield-now](https://doc.rust-lang.org/std/thread/fn.yield_now.html) | 1–23 | `34c203716e46004d493d37f19834348f53bee87835c2fd567047fdf1d131a107` |
| [std-channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) | 1–28 | `170c3d94e440324804d25b249cd7fecd1897e17650931ced49df84d0985c0668` |
| [std-sync-channel](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html) | 1–33 | `418e8ee8c7e89701b46c305f28ded380ee7b6db37522aa5b6de26d393ebb06e8` |
| [std-sender](https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html) | 1–115 | `53d2d0a5c90ee6ade8e879cf4040f0196fd1d70d6ffaa09b764ab31b55e46976` |
| [std-receiver](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html) | 1–113、222–230、247–260 | `9f1be484856d8ceefee78e226b32edcf56dc9cac365077f727a5b21c06158478` |
| [std-sync-sender](https://doc.rust-lang.org/std/sync/mpsc/struct.SyncSender.html) | 1–134 | `55e2e35cdd0502b467c0fdcf79537c3b12de1aca5921b029f822a29115fe7eff` |
| [std-send-error](https://doc.rust-lang.org/std/sync/mpsc/struct.SendError.html) | 1–78 | `169aedceb1e643ea77c75fe3c394ec191494f93b3d0e92f2518e9a962018060b` |
| [std-arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) | 1–130、1233–1249、1789–1809 | `d22b2abcee0bd265cf7b158d85aa56b68050e88c2bda73af0f7a2ca51a351172` |
| [std-mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html) | 1–170、210–319、368–389 | `0e83e7ca04ae2abf148c26b563beeae1f48fe0bd86bc3f0c9055678de6cfcd12` |
| [std-mutex-guard](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html) | 1–13、43–84 | `760af2e5ca071a55458b8cdb1ed22141fa98900425b8441248e83a9027636c49` |
| [std-poison-error](https://doc.rust-lang.org/std/sync/struct.PoisonError.html) | 1–78 | `23fa85f2b250f82348118f50859616500c7080a08f7de1323dc0b417ec16d146` |
| [std-rwlock](https://doc.rust-lang.org/std/sync/struct.RwLock.html) | 1–252、317–323 | `468563676d4e87159b5dc2a773e7d35a8da858b71502e916f27a11816fb66182` |
| [std-read-guard](https://doc.rust-lang.org/std/sync/struct.RwLockReadGuard.html) | 1–12、49–79 | `281484a5c80954d9439bd84f1095369b82b2185e69a81dd38af566cc9726a626` |
| [std-write-guard](https://doc.rust-lang.org/std/sync/struct.RwLockWriteGuard.html) | 1–51、80–115 | `59c446da78ecc27881802a7f43a4b09b50943a4850d75f471b1b38b1ad79b1e6` |
| [std-atomic-model](https://doc.rust-lang.org/std/sync/atomic/index.html) | 1–192 | `f663a014b43a880cd79dc911398ab3e3f1c220599732d65ef6deda6520cc22c8` |
| [std-atomic-usize](https://doc.rust-lang.org/std/sync/atomic/type.AtomicUsize.html) | 1–17 | `b49b2688980ed384872b37f99728a758c0ac54f54b79a5d841c8e3039888b6c8` |
| [std-atomic-bool](https://doc.rust-lang.org/std/sync/atomic/type.AtomicBool.html) | 1–12 | `3453afc31b30efa59b2045df08e31f5ef5ee5c7a4d6162ca99b60761b2c43d60` |
| [std-ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html) | 1–184 | `96715854025aa72775638f6f4ef3ce76991c9ee0c94a220731a1d06c22bb25fd` |
| [std-atomic-concrete-methods](https://doc.rust-lang.org/std/sync/atomic/struct.Atomic.html) | 1–27、122–147、5969–5988、6073–6097、6247–6262、6740–6754 | `077121e18b62e34952c5e3539e75a20e61ad5e3d1180443ec98e22eb73b3af40` |
| [mara-memory-ordering](https://mara.nl/atomics/memory-ordering.html) | 1–216、260–378、540–589、729–809 | `e8431bfa59ba763e920f4f461b0b49fa0a12ebefb955167c20dfb6ab0a0e5419` |
| [std-panic](https://doc.rust-lang.org/std/macro.panic.html) | 1–73 | `7fa6236c1f60d51b1b3cc1e0235cb24469d3032ab7f96d98865af2aacd9e17e5` |
| [std-any](https://doc.rust-lang.org/std/any/trait.Any.html) | 1–12、304–316 | `413848da9597095fe9659d2d1c59acab75a7d65150a9db0e196bb978b9c7c6a8` |

完整舊八頁、47 examples、26 compile-fail與3 runtime-fail、所有.expected、六src/manifest與九SVG XML實讀證據在ch13-preparation/old-source-read-evidence.json；該檔的來源實讀不等同本輪新位置執行。本輪65檔actual prefix閱讀與實際Rust驗證分別見ch13-authoring/actual-prefix-read.json和各stage validation。初期root要求與後來明確撤回的新API範圍均保留歷史，沒有回寫舊研究成原本已核准。
