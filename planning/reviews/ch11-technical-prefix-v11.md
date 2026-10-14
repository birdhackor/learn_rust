# 第 11 章 11.1–11.7 技術原判：prefix v11

這是作者外的獨立技術來源初判，尚未互看第 11 章首讀、同行報告或協調處置。必要待修 1 項（N1），可選改善 1 項（I1）。其餘主要方法在下列有限範圍內已有足夠的角色、操作與用途證據；沒有把來源閱讀、局部執行或圖位檢查稱為全章驗收。

## 角色、來源與封存界線

- 審閱者 `/root/chapter04_traits` 未撰寫第 11 章。曾撰寫 4.1–4.3、14.1–14.3 與 14.6，亦審閱過前面章節，具 Rust、CLI 與驗證工具背景。本輪不是基本 Python 讀者的盲讀／新首讀。
- 第 11 章唯一來源是 `/workspace/work/tutorial-implementation/frozen/ch11-prefix-seven-for-review-v11`。依順序讀完 7 頁的 **68/68 BODY 單位**，再核對 **66/66 archive 檔案**：47 Rust 檔、8 Cargo manifests、1 isolated Cargo.lock、1 `.expected`、1 README、8 SVG。manifest 明列 7 個 package；沒有第 11.8 節。
- manifest SHA-256：`cf51d563887f94549e571c882a593e26fca4daf1e147798963ac1ae29d9a774e`。68 單位及 66 archive 的 SHA 均與指定 manifest 相符；八張 `assets/` SVG 也與 archive 對應來源相同。完成閱讀後再次核對指紋，沒有變動。
- 未讀第 11 章作者帳冊／來源筆記／任何同行結果／root disposition／live 教材／11.8。沒有修改教材、root Cargo／lock／nav／checker／CI。第 14 章 root gate 或 QA 不作本輪的獨立證據。
- 通用判準使用 clear-tutorial 的 SKILL、review-protocol 與 rust-terminology：基本 Python 入口、當下必要角色先於使用、機制與用途分存、例子與效果證據分判、正常複習與補洞分判。
- 前文只按必要性回查 actual frozen 稿。主要位置是 `ch05-review-fixed-v7/005-006` 的 ToOwned／ToString；`ch06-review-fixed-v10` 的 capture／impl Trait／IntoIterator；`ch07-review-fixed-v13` 的 7.1 dependency／feature／精確版本、7.4 Read／Write／flush／slice.is_empty、7.6 clap、7.7 owned JSON、7.8 Command／Output／UTF-8／starts_with、7.9 CLI 完成邊界；`ch10-complete-root-name-fixed-v14/006-002,006-005` 的 MutexGuard Drop／!Send。必要的前置搜尋以 frozen 1–10 正文為範圍，沒有拿大綱或後面第 14 章算已教。
- 我在此前第 14 章宏作者工作中已處理過 `--manifest-path` 的說明；這是可見作者背景，不能冒稱從未見過該選項。I1 是從本輪原文重建先備後的技術／呈現建議，不是無提示首次讀者的發現。

原始頁面指紋：

| 頁面 | SHA-256 |
| --- | --- |
| 11.1 | `eda6e27b8971845c77c50bc3daa8f741d83cd5a23abfebe71af1c493cc175243` |
| 11.2 | `83d22d13f08aace8621422249fbed43dcbc355627d530b8e555a968612254ada` |
| 11.3 | `3289a205b5ba861a4e9f95a8160c42ba72acff662cda5ff71e27fbe8df8ebda6` |
| 11.4 | `54fd181dd1f290744f5a859c3aeac316e5559f6d8fac1a540a1d288ad9d21b47` |
| 11.5 | `8f40f86ea4622ca508521cb45db4e9a905829d35823a425e82eff0dbed183591` |
| 11.6 | `7c9b75581105cdc5bf131b398053f2b18913312d5d0cf9f1ba46ddcebcfee4b1` |
| 11.7 | `1be42952cd0f1569fb3b998919922ec433ae04aa87afacc3d5f00c7a6ec6727f` |

## N1 必要：11.6 沒有落實沿用的輸出完成責任

**原位置／短引文：**`006-003` 說「client／請求／body／輸出失敗回 1」；`006-007` 說「main 沿用 7.9 的 … stderr 與 ExitCode」，但同步 `render` 在 writeln! 後直接 `Ok(())`，同步 main 的 `Ok(true)` 直接 `ExitCode::SUCCESS`。兩處均未呼叫／檢查 stdout 的 flush。完整來源是 archive `examples/ch11-http/src/lib.rs` 的 `render-run` 及 `src/main.rs`。

**已教部分：**本節逐項檢查 writeln! 的 Result，run 能把這些寫入失敗包成 AppError::Output；途中失敗可能留下部分輸出，也已明說。7.4 的 `004-006` 已區分接受 bytes 與 flush 到出口；7.9 的 `009-006` 更明說「main持有Stdout…明確flush；這一步也成功，才回正常狀態」，且示範 flush Err 回 Output／狀態 1。因此不是尚未教會 Write 或退出碼。

**尚缺關係／影響：**移植到這份完整同步 CLI 時，writer 接受全部 bytes 與真正確認出口完成的責任消失，正文沒有改訂較窄的完成政策。泛型 render 可回 Ok 而沒有要求出口推送緩衝；main 也沒有補回此項確認。讀者沿用 7.9 的介面，會把這份正常狀態讀成同一種完成責任，卻無法由實作得到該保證。11.7 的 flush 不是 11.6 當下的補救，也不能回算前節已完成。

**獨立局部實驗：**在自己的 std-only isolated package 中，原樣提取 frozen 的 Inspection 與 render；使用人工 Write，其 write 接受全部 bytes、flush 明確回 PermissionDenied。執行得到 `render=Ok(())`、接受 67 bytes、`flush_calls=0`；對同一 writer 顯式 flush 才得到 Err。這只證明 frozen render 沒有檢查 flush，沒有聲稱已重現某個真實 stdout／OS 失敗，更沒有送 HTTP。

**分級：**必要；輸出契約的技術／前後責任缺口，影響有限於完整 CLI 的輸出完成與失敗政策，並非否定 HTTP 分層或同步網路流程。不是主張任何 writeln! 後都必須 flush：本例自己的完整 CLI 承諾與已引用的前文已提出該完成責任。

**最小修法：**在 main 或 render 選一處承擔 flush 並保留其 Err；若沿用 11.7 的安排，render 末尾交回 `output.flush()`，既有 run→AppError::Output→退出 1 即可承接。在該段說明一次完成責任即可，不新增 I/O 章、socket helper 或公共網路測試。無需讓 library 與 main 重複 flush。可用已教的人工 Write 做針對性驗證。

**時點與處置：**本輪獨立技術閱讀先看出同步 source 沒有 flush，回讀 actual 7.9 確認原有責任，再作相稱實驗；未受第 11 章同行提示。只提出待修，未改教材／未做修後複查。

## I1 可選：替新 Cargo 選項補一個直接對應

**原位置／短引文：**`005-009` 的負例說「它有獨立的 Cargo fixture manifest 與 lock，以固定 Tokio 依賴檢查故意失敗的 example」，隨後給 `cargo check --manifest-path fixtures/cross-crate-fail/Cargo.toml --example held_guard --locked`。

**已教部分：**同段已說明空 `[workspace]` 的隔離用途，已有完整相對位置、目前 package 工作目錄、故意失敗 target 與預期診斷。Cargo check／example／locked 的角色可沿前文；7.1 也已教本機 path dependency、行內 TOML 表格與 `=版本` 的精確要求。不能把這些整體說成未教。

**缺少與收益：**指定 frozen 1–10 正文未出現 `--manifest-path`。本段說明已足以讓讀者連起「用獨立 manifest 檢查這份 example」，但沒有直接把新選項名稱對應到「選哪份 Cargo.toml」。在命令前補半句，可減少第一次讀新旗標的猜測，也方便讀者分辨選 package manifest 與選 example target。

**分級：**可選的名稱／操作對應改善。現有文字已給完整檢查對象、隔離理由與命令，沒有因此阻斷 !Send 的主要推理；不因缺少逐旗標表格而判整份負例先備不成立。

**最小修法與負擔：**「`--manifest-path` 選這份 fixture 的 Cargo.toml」即可；不重講 Cargo workspace 或再貼完整 manifest。此建議有此前宏作者使用同一工具選項的背景，不冒稱無工具知識的首讀發現。未改稿。

## 主要方法的來源通過與有限界線

以下分開保存「機制／性質」與「需要／用途」。來源列的是本輪實讀正文及完整 archive，沒有把測試名稱、外部連結或 root 曾經通過當正文已教。

| 範圍／角色及先教後用 | 機制／性質的最強支持 | 原需要／用途與最短連結 | 例子／結果及限制 |
| --- | --- | --- | --- |
| 11.1 `001-001..009`：Future／Output、async fn／await、runtime／executor、current-thread、Tokio main/test attributes、async block／move、impl Future | `001-002` 在兩函式前解釋箭頭是 Output、引數先求值、本體後執行；`001-003` 在入口前解釋 runtime、rt/macros 與 attribute；`001-005..008` 再逐步教 capture、普通 future 工廠及 Result Output。四份正常 examples、library、main、測試與答案均同序可讀。 | 原文需要先分清建立／執行／取得結果；保存工作表示讓之後決定 await 或 drop。沒有虛構 thread 方案做不到，也沒有以 async 名稱聲稱變快。 | 完整 stdout 順序與 visits 的 drop/await 兩路，能區分本體時點；unused 小計120不印本體。明說本例無外部等待，不證暫停／交錯／效能。小整數限定避免宣稱處理全部 i32 溢位。 |
| 11.2 `002-001..009`：spawn／Send+'static、Tokio mpsc、JoinHandle、兩層 Result、abort+await、JoinError／Id、JoinSet／AbortHandle、HashMap.remove | channel 容量、Sender clone、recv Option、send 成功不等於工作完成先教；spawn 的值與借用 bound 不因 current-thread 消失。`002-004` 先分 Output 的 WorkError 與 JoinError；`002-005` 先給取消／等待清理；`002-006` 在 collect_one 前解釋 ID 對照及 remove 契約。完整7份來源與关送端答案相符。 | 要處理多份工作及每份失敗，不能只把 input 塞入正常 Output；JoinError 無正常 Output，仍有 Id，因而能移除對照並回原輸入。 | 四項 channel gate 控制 Value／WorkError／panic／取消；容量及保留放行送端避免用毫秒猜排程。關閉 release 得 ControlClosed 的答案仍是正常 Output 的 Err，沒有混成 JoinError。ID opaque，不固定數值或輸入次序。 |
| 11.3 `003-001..009`：Job／pending owned iterator、Scheduler、tracked／submitted、限額檢查／回收再補工 | `003-002..006` 先給同序 Jobs/releases、IntoIter<Job> ownership、狀態欄位與 len 檢查。fill 在取一份 Job／spawn 前查 `< limit`；next 先移除一份結果再 fill。完整 source 的 outcomes 沿前節能力，沒有引進新收集捷徑。 | 原需要避免一次提交全部工作；把未提交材料留在 pending，每次回收才釋出一個提交位置。界線選「已提交、尚未回收」，不是只有正執行的 task 數。 | 五份受控材料在 WorkError／panic 後都補工且保原 input；zero-limit 提交前拒絕，空輸入立即完成。圖②是方法中間狀態、③是返回狀態，正文明确說明。上限1答案僅在該特例保輸入次序，不推廣所有上限；不承諾 pending/reports 或全部記憶體有界。 |
| 11.4 `004-001..009`：Duration／std Instant、Sleep、Timeout／Elapsed、pending、同步 sleep、paused clock／test-util／Tokio Instant | 每個操作在 code 前有輸入／Output／單位；wrapper 期限在呼叫時開始，Result 包原 Output；drop 不復原外部 visits。`004-007` 在 tests 前給 clock 自動推進條件與 std/Tokio 時鐘差異。完整9份來源含全部七項 tests／答案相符。 | 原需要知道願意等多久與逾時留下什麼；整段共用50ms和每步各50ms回答不同的預算需求，先選範圍才決定包哪一層。 | 即時工作／pending 分支、nested Result、visits=1、有限30ms阻塞對照、paused clock三路與兩步案例有互補用途。Tokio1.53.3 source 的 poll 確為 inner-first，正文版本限定正確；不是 timer 硬切 thread、真實延遲上限或全部排程證明。 |
| 11.5 `005-001..013`：select!／biased／cancellation safety、借用 join 等待、Runner 分類、abort_all、try_recv／is_empty、Ctrl+C、ready、guard與CLI收尾 | `005-002` 先教分支 syntax、同一 task、優先次序與取消哪個值；`005-006` 在 stop_and_finish 前教 abort_all 不移除／不等清理；`005-007` 先解釋 Empty/Disconnected、空輸入與補工前檢查。`005-009..011` 前文guard!Send、listener first-execution、Unix全process影響、entered與observer的責任均在 code 前。完整12份來源／負例／lock／答案均讀。 | 原需要停止提交後仍交完整每輸入分類；Id保已提交身分，pending 無task所以記 Unstarted，先 abort+drain 再清理pending，才有完整Batch。stop try_recv 是補工前查已排入事件，不聲稱訊號瞬間原子禁止提交。 | 借用 join 輸掉之後原task仍回80；受控兩種失敗後停止保分類；提前停止提交0；current-thread returning觀察點支持完成未回收成果保留。Ctrl+C ready 不拿 job啟動當 listener註冊證明。所有 gate、controller、observer 都有收尾；不保證外部副作用還原。實際OS訊號操作本輪未重現。 |
| 11.6 `006-001..010`：URL／HTTP／TLS／status／redirect、blocking client builder、parse/scheme/as_str、Response／status/body、Error、or pattern與byte literal | URL各部分與示意HTTP角色先教；`006-005` 在流程前給選定API與formatter能力、UTF-8替代解碼、同步send/text分別10秒；`006-006` 先教Body狀態與pattern，保存status/URL後才消費Response。本 package 的7份 archive 均讀。 | 原需要分對方回了什麼、HTTP是否通過、或根本未取得回應；200–299政策、404報告、Body保存已收到status分責任。預設headers完成，body開關另要求完整內容。 | url/source/表示與CLI離線 tests沒有冒稱已測redirect/timeout網路；公開網站只手動示例與假設輸出形狀。byte literal／borrowed array與slice比較在 tests前解釋。此列通過限HTTP與已檢寫入Result；完整輸出責任仍有N1。 |
| 11.7 `007-001..009`：async Client／pool／net、shared client與owned URL、send普通工廠／text async、同一請求預算／量測、flush、ownedJSON模型 | `007-002..004` 先分runtime/client/future與features，指定send在呼叫時準備工作和timer，立即await；兩步共用請求deadline。`007-005..006` 把Request／Body／Output保留，render明確flush。末尾PageRecord／ReplyError在decode前教；所有9份archive檔與完整答案相符。 | 原需要保持同一HTTP工作，先確定等response還是body再組多份；client pool是重用安排，單請求不聲稱同步版不能完成或async更快。應用JSON仍是HTTP工作完成後的另一個Result。 | 人工report含None／Some(empty)、人工flush失敗、parse拒絕、真binary參數與owned PageRecord能驗其局部契約。elapsed排除parse/build/output、非固定42ms。未引進學生未知server/helper；11.8網路驗收合理待教，本節不需要它才懂當前流程。 |

補充的跨例核對：

- 普通 async block 可借用 heading；async move 在建立時按值 capture，String 的 owner移轉與 reference 的借用資訊不同。沒有用「move=把 reference 變owned」推理。
- spawned工作與停止controller的 send 均只有受控容量與收端仍存在的範圍才 expect；「send Ok 不等於工作完成」在一般敘述保留。`005-012` 的 returning證據額外依賴 current-thread、send之後沒有下一個await且直接return，足以支持該受控例，不推廣任意channel成功。
- 各工具輸出順序的固定部分來自 gate 與收取流程，不依賴task啟動次序、隨機ID、panic完整OS診斷或精確sleep。panic主文保留stderr與CLI退出政策，不把捕捉panic說成沒有發生panic。
- HashMap對照只在每份spawn後登記，再在回收時取除；Id存活保證涵蓋JoinSet追蹤與AbortHandle，不擅自把Id當永久序號。已完成但未回收仍計tracked，峰值定義與tests/圖相同。
- 兩HTTP package把Url／status／URL字符串保存契約與ToOwned/Copy/Display接起；Response.text消費後沒有再讀Response。charset未選時不合法UTF-8替代，不誤稱嚴格字元編碼驗證；body無大小上限的限定有當下用途。
- 根共享 Cargo.lock 沒有包含在本輪66個archive中；因此沒有拿某個可能變動的 live 根 lock 冒稱 frozen v11 的獨立執行版本。隔離 !Send fixture的自己的 lock已讀，固定Tokio1.53.3；本輪查證用該固定版API及自己的std-only probe lock。

## 正式契約查證與實際執行

查證保存於 `/workspace/work/tutorial-implementation/review-ch11-technical-prefix-v11-author04/`，資料夾 `official-source/` 是受管理Cargo registry下載的指定版本原始碼副本，`official-source-manifest.json` 保存逐檔SHA；不是作者的source-notes。另實際取得並閱讀相應區段的正式HTML，URL與HTML SHA在 `official-web/manifest.json`。

| 正式來源與實读位置 | 對本輪判断的作用 |
| --- | --- |
| [std1.99 Future](https://doc.rust-lang.org/1.99.0/std/future/trait.Future.html)：Output、poll與runtime characteristics | 一般future需被驅動；另task的工作可獨立前進，支持11.1與11.2不同啟動安排。正文不須提前教完整Poll/Pin/Waker才可await。 |
| [std1.99 Write::flush](https://doc.rust-lang.org/1.99.0/std/io/trait.Write.html#tymethod.flush)：trait與flush契約 | 接受資料與推送緩衝到出口分開，支持N1的有限判斷，沒有把flush說成永久儲存保證。 |
| Tokio1.53.3 `runtime/task/id.rs:5–32`、`task/join_set.rs:126–151,303–319,458–466` | ID可重用的條件／Copy Eq Hash Display、spawn bound、取消join等待不移除task、abort_all需後續drain，與正文一致。 |
| Tokio1.53.3 `sync/mpsc/bounded.rs:187–205,325–338`、[select!](https://docs.rs/tokio/1.53.3/tokio/macro.select.html) 的runtime characteristics／fairness／cancellation safety | recv與join選擇的保留責任、同task、biased次序合理。正文所選未使用receiver.close或Permit，sender全drop+buffer讀空的限定按本節subset核對，不為未用API擴寫。 |
| Tokio1.53.3 `signal/ctrl_c.rs:9–30,59–62` | first-poll註冊與Unix全process行為符合ready路線；未以固定sleep猜註冊時點。 |
| Tokio1.53.3 `time/timeout.rs:86–100,209–225` | deadline在呼叫建立；實作先poll內層再delay，支持正文刻意版本限定，包含即時完成與有限同步阻塞反例。這不是拿一般文件概括覆蓋不同版本實作。 |
| reqwest0.13.5 `redirect.rs:126–139` | 判斷 `previous.len() > max` 排除initialURL，5次follow後第6次Err；未誤套舊版邊界。 |
| reqwest0.13.5 `blocking/client.rs:388–395,1500–1540`、`blocking/response.rs:295–302` | send與text分別啟動wait::timeout，支持11.6各段10秒而非總10秒。 |
| reqwest0.13.5 `async_impl/request.rs:521–525`、[RequestBuilder.send](https://docs.rs/reqwest/0.13.5/reqwest/struct.RequestBuilder.html#method.send)、`async_impl/client.rs:1444–1454,2670–2683,3097–3106,3138–3147` | send是普通工廠，立即execute_request設定timer；取得Response把同一 total timeout帶入body。11.7總期限與只立即await的設定有來源。 |
| reqwest0.13.5 `async_impl/response.rs:148–177` | 未選charset時bytes讀完後UTF-8 lossily轉String，與本文替代字元、Response ownership和body錯誤範圍一致。 |

實際執行只有 N1 的有限 std-only probe：`output-boundary-probe/` 的空 `[workspace]` isolated package、自己的 Cargo.lock；以本專案選定 `rustc 1.99.0 (b940084d7 2026-09-28)` 執行，exit 0，來源提取資訊在 `output-boundary-probe-source.json`、結果在 `output-boundary-probe.log`。這些是審閱工作檔，不放學生程式或正常workspace targets。

沒有重跑整個workspace／本章所有tests來代替閱讀；沒有host執行無限pending、壞協定或deadlock。未送公共HTTP、未以公共網站固定內容作答案，未自行重現SIGINT CLI／TLS／實際redirect／timeout網路路徑。正文作者的「另有實跑」陳述本輪只核可觀察介面與安排，沒有查其作者日志或冒稱自己實驗。

## 圖解與實際頁面

八張SVG source全部讀完，逐項對應必要資料；不把「圖存在」代替對應證據。角色與最強支持：

| SVG／所在單位 | 必要對應與結論界線 |
| --- | --- |
| future-lifecycle／`001-004` | 同一work保存80→await→Output100→total；另unused120為不同工作直接drop。向下線表示操作順序，不表示時間或thread。 |
| task-result-identity／`002-006` | 同一submittedtask分正常Output與JoinError，兩路都以它的Id進同一inputs.remove，取原input並回Report；沒有用兩路相等數值當同物證明。 |
| bounded-refill／`003-005` | 第一批0/1、pending2/3/4；收回1後tracked1且submitted2；fill2後tracked2且submitted3。正文明确圖②在next內部，③才返回。 |
| timeout-scope／`004-008` | 上方整段50ms第一步30、剩20第二步未完成；下方每步重給50、兩步完成2。圖自標手工示意，不當真實排程截圖。 |
| borrowed-join-wait／`005-004` | select丟棄的是借用JoinSet的等待；同一集合／input0仍追蹤，放行重新join後才清空與Value80。 |
| stop-and-finish／`005-006` | 不再fill→abort_all仍持task→按ID逐項drain→未提交pending的Job清理／Unstarted→集合與對照空→完整Batch。顏色區分來源責任，箭頭是處理順序。 |
| http-response-boundary／`006-006` | 同GET取得headers預設可交報告，--body才再读完整text；兩次同步等待各10秒。正文排除「網路必尚未收到body byte」的誤讀。 |
| single-http-budget／`007-004` | Client/Url準備在deadline外，兩個等待與中間status保存被同一虛線預算框包住；body只按開關走，owned報告/elapsed結束在下方。沒有把兩步各重給10秒。 |

使用僅指定凍結68BODY及原樣SVG建立獨立Zensical預覽，沒有讀live第11章。渲染轉換只是合併同頁chunks、移除重複H1，以及把freeze絕對SVG路徑改為預覽相對asset路徑；shared theme/extensions/CSS的當次指紋與做法在 `preview-transformation.json`。其nav僅含11.1–11.7。預覽建置成功不能代替以下實看。

**實看範圍：**Chromium，desktop **1280×900**、mobile **390×844**，1×CSS pixels，八圖各兩種尺寸共16張**原位viewport PNG均實際親看**。八圖都出現在相應正文圖位，圖前/後文字對應固定稿；未需點擊放大，沒有圖中文字截斷、必要箭頭遺失或浮動header遮圖。340px圖保持自然大小，390px timeout圖在手機縮至約343px仍可辨字。较小的13/14px標示在本輪尺寸可讀，沒有只因與其他圖字級不同就報必要缺陷。

PNG與bbox、前後鄰接DOM在 `visual/`、`visual-capture.json`、`visual-viewed.json`。本輪確認原位必要圖文與此兩組尺寸；沒有逐一操作所有details／copy按钮／深色模式或驗收實際GitHub Pages部署，不称全部互動／全站視覺通過。

## 完成狀態

本輪原判完成範圍僅11.1–11.7：68 BODY／66 archive／8 SVG，正式契約局部查證、1項有限實驗、16張原位圖位實看。N1待修；I1可選；主要通過項保留上列有效範圍。報告和同字節initial副本／SHA封存後停止；未交叉、未看11.8、未修稿。後續任何提示、處置或修後查核另存，不覆寫這份來源初判。
