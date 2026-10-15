# 第 13 章 ORIGINAL TECH13：v5 獨立技術初判

## 角色、隔離與原稿

本人不是第 13 章作者；曾撰寫第 2 章、4.4–4.5、第 5 章、9.2–9.4，也曾做第 3／7／8／10／12／14 章審閱。這是有前文背景的獨立技術審閱，不稱 blind first reading。本輪先依序讀完整正文，再讀來源；技術角色的原始正文閱讀包含折疊答案，實頁則另先看 closed 主文，再展開指定答案。既有 TECH12 cross `deb9b7a9…`、TECH14 original `93cfeddf…` 及其他封存保持不變。

唯一稿為 `/workspace/work/tutorial-implementation/frozen/ch13-complete-prerequisite-v5`；manifest SHA-256：`b4a997a6f10c76039c1a1a0e60a6ad60a395b1c4641b789756ffe4a46704f772`。沒有讀第 13 章 live／其他版本、作者 notes／ledger／handoff、root gate proof／處置、FIRST13／CONT13 或第 15 章。root 的作者前置檢查不作本人證據。未修改學生來源或共享檔案。

已核 AGENTS、clear-tutorial SKILL、review-protocol 與 rust-terminology；其本輪指紋保存於 `work/ch13-technical-v5/instruction-fingerprints.json`，與本人既有全文閱讀版本相同。依「實際先教」「原需與機制分開」「只認本人實看／實跑」核查。

## 結論與分級

**必要修改 0；可選修改 0。** 本稿所需介面／型別能力均可對回實際前文或當節首次使用前的最低說明；重要方法的安全、等待與用途邊界相符。這不是所有 future／executor 的正確性、平台行為或整站發布驗收。

原始順讀的未知另留於 `body-notes.md`，未拿事後查證回寫早期判斷：

- 13.1 的 `Option<&i32>` Debug：實際 4.3 `003-005` 已教「Option 和數值 reference 也能用 Debug 顯示」，因此解除疑點，沒有從只會 `&str` 推出所有 reference 的能力。
- shared Pin 是否偷用 Copy：完整 tests／答案均從 owner 或 mutable wrapper 重新 `as_ref`，沒有保存同一份 shared Pin 反覆接手；mutable wrapper 重用則明寫 `as_mut`。解除疑點，不新增讀者未學的 Pin Copy 條件。
- Poll 的 Debug／PartialEq：完整測試用 match 分類與已教的數值 assertion，沒有依賴這些額外能力。
- 測試讀父 module 私有 State：13.4 `004-007` 在 helper 之前明說子 module 可讀父 module 的私有 State 與 work.shared，足以提供本例最低規則；不把第 1 章 public module 的介紹當成完整私有性先備。
- Future／await、借用收端與 take 的具體前文：本輪實讀 11.1、11.2 recv 表、11.5 controlled_job、11.8 take；13.3 也在自身首用表補足 take 角色。

仍未實跑的正常／失敗材料列在後面；它們是執行覆蓋限制，不因未跑便判成缺陷。沒有以「多補完整 runtime／取消政策」或「多跑無改動 suite」湊可選項。

## 全文與來源覆蓋

依 manifest 讀完五頁 **32 BODY**：001-001–006、002-001–005、003-001–008、004-001–007、005-001–006。之後讀完 **44 archives**：27 Rust（六份 library／module、16 正常 examples／答案、五份故意失敗來源）、七 SVG、三 Cargo.toml、兩份隔離 fixture Cargo.lock、五份 expected。所有正常例、tests、helpers、完整答案、負例與配置都在技術核查範圍；不是只看摘錄或已報問題方法。

原始七批全文來源、批次對照、85 個 snapshot 檔案 SHA 與 44 份 source 映射保存在 `source-batches/`、`source-batch-map.json`、`source-full-read-coverage.json`、`input-fingerprints.json`。結束前重算，85 份仍同 byte；37 份非 SVG 在本人隔離副本也逐份同 byte。

實際先備並非從大綱放行：1–10 沿本人已讀凍結正文／來源背景，必要時回查 canonical 1v8、4v8；12 使用修後 v5 的 `003-002／003-003／003-005`，確認 raw pointer Copy 不取得 ownership、raw borrow、來源存活與 array→slice 能力。本輪新增必要 11v16 實讀 `001-001／002／003`、`002-003`、`005-003／004／005`、`008-006`；Future Output／await／runtime、recv mutable 借用與 controlled_job、take 接手原 Option 都有具體位置。選取正文、SHA 與逐段證據保存於 `actual-prerequisite-selected.json`；先前章節不是本輪重新全章審閱。

## 重要通過項：原需、機制、實際用途與限制

| 位置 | 原需要與方法原句 | 本人核查的機制、有限例子用途與限制 |
| --- | --- | --- |
| 13.1 `001-001／002` | 「內部保存的位置資訊，之後仍對得上同一處資料嗎？」；「move 是 ownership 的語意」 | 11.5 實例的外層 owned Job 與 recv 借用是同 future 的資料關係入口，圖明說不是 compiler layout。lifetime、ownership、地址穩定分三問；沒有說每次 move 都搬整份 heap 內容，或每份 future 都必須有自參。 |
| 13.1 `001-003／004` | swap 「交換這兩個位置裡的整份 T」；藍箭頭「不表示此刻可以透過它讀取資料」 | mem::swap 的兩份同 T mutable 借用與 unit 先教；BorrowedNote 的後續 reference 使用阻止全值交換。AddressNote 的 raw 型別／null／eq 在 code 前，saved Copy 依實際 12.3；正常模型從未 dereference saved。80／120 交換對比同為80都能分清內容與位置，eq 並不證明存取資格。 |
| 13.1 `001-005／006` | 「移交 Box owner，不等於交換裡面的資料」 | 交換的 T 是 Box handle，不是 heap Note；普通 Box 仍允許取出內容，因此不足以提供後續共同 pin 契約。只交換 i32 欄位的練習保留整個 Note 位置，先引導再答案；測試與例子的數值用途一致，不能把結果外推成一般地址安全證明。 |
| 13.2 `002-001／002` | 「原值仍有效時，不能任意移出它所在的位置；收尾也要在該位置完成」；「Pin 是 library contract」 | Pin 包 pointer，約定作用在 pointee，wrapper ownership 仍能移交。Drop guarantee 是在儲存失效／重用前原位 drop，直到該 drop 返回或 panic，不等於保證每份值最終 Drop。借用 Pin 的共同介面與單型別 private Box 的保護各有角色，沒有從 Pin 新產物倒推需要。 |
| 13.2 `002-003／004` | get_mut 「要求 T 是 Unpin」；constructor「先安置資料，再記地址」 | 新 Unpin auto trait、i32、raw pointer、PhantomPinned unit struct 在所需 code 前。要求看 pointee，不拿 Box handle 能力代替。Box::pin／as_ref／as_mut／get_ref／get_unchecked_mut 與 typed self 逐項先教；重借與接手 mutable wrapper 分清。完整 constructor／全部公有 method／Drop 均核：unsafe 只記地址、改既有 i32，不取代 Self、不交出 mutable Self、不投影 Pin 欄位；Drop 同守約定。 |
| 13.2 `002-004／005` | 「把 get_mut 改寫成 unsafe getter，並不是讓那次整份交換變合法」 | 同 library 外部 fixture 的 E0277 擋的是 !Unpin pointee，不是跑壞交換後才檢查。owner swap／改140／Drop140、80及練習改100支持有限關係；不是靠一次地址輸出證明全部 unsafe。所有測試的新 API 與重借均有前置。 |
| 13.3 `003-002／003／004` | 「資料已授權、通知已發生、poll已執行、答案已交出」；Pending 不交 Output | Poll Pending／Ready、Future associated Output 與完整 poll receiver、Context borrowed Waker 來源、clone owned 通知 handle 先分角色。Wake 的 Arc typed self 與 Waker::from 的 Wake+Send+Sync+static bound 分開。Counter 只記 wake 回呼次數、不執行 future；Mutex 短 guard 無跨 poll 持有。 |
| 13.3 `003-005／006／007／008` | 「每次 Pending 都保存這次 Context 的 Waker」；「停止 poll 與值的 Drop 是兩件事」 | ManualAmount 的 value／released／completed／polls／waiting 與 Unpin 條件先教；take 接手通知、最新 pending 覆寫來源。release 後 poll 次數仍1、通知1，第二 poll 才 Ready80；另一例事件先120則第一次 Ready，無必經 Pending 假設。安全的再 poll 可 panic但不能 UB；隔離 runtime failure、expected panic 字串、cargo test --no-run 的編譯用途先教。 |
| 13.4 `004-001／002／003／004／005` | 「事件卻在它登記通知前到達，之後還有人叫它再檢查嗎？」；「同一把鎖，把檢查與登記接成一段」 | 遺失通知是具體 check-unlock-event-register 空隙，即使無 data race 仍可能不再前進；不是有 Mutex 名稱即足夠。EventAmount 與一次供值權限 AmountEvent 共有同 State，polls／completed 留 main。正確 code 同 guard 查結果或登記，事件同 guard 存值並取 Waker；next clone、old Waker Drop 與 wake 都在 guard 外。只有兩種持鎖順序，圖不把它畫成測量時間。 |
| 13.4 `004-004／005／006／007` | 「解鎖後才叫回呼」；「lock 競爭仍可能讓呼叫 thread 等鎖」 | 不聲稱 lock-free／所有 call 絕不阻塞；鎖內沒有 readiness recv／join／I/O／回呼等待。Arc／Option／Waker 的 Unpin、Send 條件在相關 code 前；Option.is_none 首用先教。先 poll 後控制供80與先供120再 poll 的兩條有限路線由 channel／join 安排，不用 sleep。ReadinessObserver 同 State 重鎖讀 Some(0) 副本，Copy 條件先說、不 take 原結果；它只檢查 callback 在鎖外，不證明所有排程。事件端 drop 不自動給 ReadyErr／wake 的限制有寫。 |
| 13.5 `005-001／002／003` | 「何時再檢查」；「通知不用攜帶另一份 future」 | run_one 接手並 pin 一份 F，no F:Unpin／Send／static 額外要求，poll 仍在 caller thread。ChannelWake 只送 unit，值80仍在 State；Sender Send／Sync 條件明說。Pending 後 executor 才 recv，Ready 即停止並返回 Output；callback 不 inline poll。driver 自己保送端，所以沒有事件不會靠 recv 自動關閉；expect 不提供進度、取消或期限，不能實跑永遠 Pending 當正常例。 |
| 13.5 `005-004／005／006` | 「確定先 Pending，再啟動供80的事件」；「main仍另外join，才說worker已結束」 | StartAmount 先 inner poll 登記再控制 send80，兩個 channel 分別傳 i32 與 unit；getter 的 Unpin 條件與 Option.take 先解，沒有未教 arbitrary projection。Recheck 在 poll 內先 wake_by_ref 再 Pending，buffer 保存通知，次 poll Ready120；default Wake::wake_by_ref clone Arc 的契約已說。PinnedAnswer 的 marker 只檢查 !Unpin 介面，不假它真的自參。練習改120、外部 worker join 與 driver Output 的收尾責任分開，沒有冒充 Tokio I/O／timer／多工作 runtime。 |

全部六 module、16 正常 examples／答案、18 library tests 的輸入與分支也逐份核：無未教 Poll assertion traits、Pin Copy、raw dereference、projection、RawWaker、pin!、super／grouped use；helper 與答案沒有提前帶入後節 API。Source 第一階段沒有把後節檔案內容灌回早節正文。五負例是 E0502、兩 E0277、E0382 與 finite 完成後 panic，正常 workspace member 沒把故意失敗 target 混入；兩現有 fixture locks／path dependency 相符。

## 官方契約核對

讀固定 Std **1.99.0**，不是拿最新版方法名猜契約。下載 byte SHA、選取區段保存在 `official/downloads*.json`、`official/selected-contracts-*.json` 及本人的文字摘取：

- [Pin module](https://doc.rust-lang.org/1.99.0/std/pin/index.html)：address-sensitive lifecycle、library contract、Drop guarantee；[Pin](https://doc.rust-lang.org/1.99.0/std/pin/struct.Pin.html)：new／as_ref／as_mut／get_ref／get_mut／get_unchecked_mut 的完整 Safety。配合 [Box::pin](https://doc.rust-lang.org/1.99.0/std/boxed/struct.Box.html#method.pin)、[PhantomPinned](https://doc.rust-lang.org/1.99.0/std/marker/struct.PhantomPinned.html) 核前述限制。
- [Future::poll](https://doc.rust-lang.org/1.99.0/std/future/trait.Future.html#tymethod.poll)：只保最新 Context 的通知、快速返回、完成後不再 poll，但安全 call 不能 UB。[Context](https://doc.rust-lang.org/1.99.0/std/task/struct.Context.html) 的 from_waker／waker 保借用來源。
- [Wake](https://doc.rust-lang.org/1.99.0/std/task/trait.Wake.html)、[Waker](https://doc.rust-lang.org/1.99.0/std/task/struct.Waker.html)：Arc receiver、From bound、default wake_by_ref，以及 executor 持續執行／工作未完成時 wake→後續 poll 的 happens-before 要求與可合併通知。此例先發送再接收的 channel／受鎖 State 支持資料與通知關係；有限 stdout 不是 memory model 或所有 schedule 的證明。
- [Arc](https://doc.rust-lang.org/1.99.0/std/sync/struct.Arc.html)、[Sender](https://doc.rust-lang.org/1.99.0/std/sync/mpsc/struct.Sender.html)、[Option](https://doc.rust-lang.org/1.99.0/std/option/enum.Option.html)、[raw pointer](https://doc.rust-lang.org/1.99.0/std/primitive.pointer.html)：實際 selected Unpin／Send／Sync／Copy 條件，take／is_none／send 契約；[ptr::eq](https://doc.rust-lang.org/1.99.0/std/ptr/fn.eq.html) 是地址比較、[mem::swap](https://doc.rust-lang.org/1.99.0/std/mem/fn.swap.html) 交換兩位置而不 deinitialize。沒有把官方額外例子算成讀者先備。

## 本人有限執行與畫面

自建 `work/ch13-technical-v5/probe-workspace/`，37 份 archive copy SHA 全同；只列正常 package member，故意失敗 fixture 保持隔離。本人另 generate-lockfile --offline，保存 own lock SHA `0fd6d8695f35d28ddf9df3246775cedee87accc94ce1ee126e6d21af8659ec2f`；它是本人的最小 workspace lock，不稱 repo 根原 lock。rustc 為 `1.99.0 (b940084d7 2026-09-28)`。四個 purposeful probe 每個設25秒上限：

| 本人執行 | 結果與實際證據範圍 |
| --- | --- |
| 篩選 `readiness::tests::the_callback_can_lock_the_state_and_observe_the_supplied_value`，--lib --locked --offline | exit0，**1 passed／17 filtered out**；callback 能重鎖讀 Some(0)，原結果仍由 future 取得。沒跑持鎖呼叫 wake 的死鎖版本。 |
| `executor_early_notice` --locked --offline | exit0，與原正文六行完全一致：先 poll／Pending／收到通知／再 poll／Ready，答案120。支持通知早於 recv 保留，不當一般 executor 活性證明。 |
| `pinned_owner_swap` --locked --offline | exit0，與原正文七行一致：owner 交換120／80、改140、依序 Drop140與80、正常收尾。沒有透過 saved dereference。 |
| 隔離 fixture `cargo check … --example pinned_get_mut --locked --offline` | exit101，兩處 E0277 與 expected 相符，真正連同一份 library；不是因 helper 缺件／未上鎖失敗。 |

詳細 command、stdout／stderr、exit code、時間留於 `probes.json` 與 `probe-01…04.json`。最初 Python 呼叫未帶 env 時找不到 cargo，沒有執行任何 Rust；帶既有 `scripts/env.sh` 後完成上述檢查，工具入口失敗不列教材缺陷。未全跑18 tests／16 examples／五負例，未跑 cargo clippy／全 target suite／Miri，未執行 UB、無界 Pending／壞通知協定／死鎖；其他負例為 source／expected／契約核查，不冒稱親跑。

本人用 exact v5 32 BODY 重建五頁、只移除重複 H1／改靜態圖 URL；seven SVG byte 全同，公開 CSS 明列 extra_css，沿本人既有原稿 theme 保留 navigation.top。preview config SHA `331604ae4e0b5fffd8c96b1d4c0bca26d03a65c83abd3c70f0f2bf4f13644dba`，CSS SHA `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`。本人 zensical build exit0，實際 Chromium／Playwright 在 `1280×1200`、`390×1200` 看原位七圖，共14張；另13.2／13.5答案先 closed 後展開，共8張，**22張均本人 view_image 親看**。每次 capture 已比對 served SVG 與 CSS bytes，不以 bbox 或 root 截圖代實看。

七圖的字／框／箭頭／圖例／80、120數字角色清楚，手機約343px圖寬的24px原字縮放後仍約20.6px；沒有遇到缺字、字越框或誤把 wake 畫成 poll。caption 在圖前交待借用／共同 owner／saved地址／操作序與鎖段的表示；原需→機制→有限結果相鄰。兩個選讀答案閉合時不先揭完整code，展開後位置與主文續讀清楚，手機長code採局部橫滑。證據為 `visual-metadata.json`、`visual-viewed-proof.json`、`preview-proof.json` 與22張 PNG。不聲稱測遍所有 scroll／TOC／menu／深色模式、不同viewport高度、無障礙或整站 UI。

## 封存與停止範圍

這是本人唯一 ORIGINAL TECH13 initial。完整原始判斷、來源指紋、selected 前文／官方、有限 command 與親看畫面隨報告封存；同 byte initial copy 與 SHA sidecar 不因後續同行資訊改寫。完成後停止，等待 root 另開 cross；不先讀 peer13、其他章作者結果或第15章。
