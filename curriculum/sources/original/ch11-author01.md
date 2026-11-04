# 11.4 作者實讀來源：時間、timeout 與受控驗證

依 clear-tutorial、writing-prompt 與本專案術語指引寫作；入口為基本 Python，所有新能力在正文首次需要前負責教。本筆記是作者研究，不是獨立審閱。

## 實際先備

2026-10-14 重新實讀 `ch11-foundations-prerequisite-v3` 全9 units及凍結的foundations manifest／library：Future Output、async fn與block的建立／執行、capture、current-thread Tokio入口、Result Output、tokio::test與drop已成立。`ch05-review-fixed-v7` 的5.6全7 units與source已讀，包含相同錯誤型別的From identity及問號每次處理一層Result。11.2／11.3尚未交付，不用spawn、JoinSet、channel、select或signal作範例先備。

## 重新讀過的英文與中文教材

| URL | 實際閱讀範圍 | 值得學習與本節取捨 |
| --- | --- | --- |
| [英文Book17.3](https://doc.rust-lang.org/book/ch17-03-more-futures.html)／[簡體中文對應17.3](https://kaisery.github.io/trpl-zh-cn/ch17-03-more-futures.html) | Yielding Control／交還控制權的slow阻塞例、sleep與yield差別；Building Our Own Async Abstractions／建構timeout的輸入與Result、timer與工作競爭、第一項先取得執行機會 | 先定需要、再解讀外層Result與原Output的關係；不複製trpl::select／Either或先自寫通用timeout。採Tokio已提供的介面，參數順序及Elapsed按固定版核對，不能套用書中自訂函式。 |
| [英文Comprehensive Rust：Blocking the executor](https://google.github.io/comprehensive-rust/concurrency/async-pitfalls/blocking-executor.html) | 全46行，普通thread sleep阻塞current-thread、改async sleep的對照與speaker notes | 阻塞本體與timer等待的角色分清；11.4不用join_all、spawn_blocking或FFI例，改為一份future的timeout限制。 |
| [英文Comprehensive Rust：Cancellation](https://google.github.io/comprehensive-rust/concurrency/async-pitfalls/cancellation.html) | 全119行，drop取消、逐byte資料保存在future內的失敗、移到外部struct保存進度的改善 | 取消不等於回復已發生操作。用已教mutable borrow與小數字記錄開始，不先用DuplexStream、select、mem::take或後文async I/O能力。 |
| [Tokio官方Unit Testing](https://tokio.rs/tokio/topics/testing) | Pausing and resuming time in tests段落26–64行：無其他可執行工作時跳到timer、start_paused與test-util | 借鑑受控時鐘驗證timer的先後條件；頁面以std Instant印0ms只是可能觀察，不照搬成硬斷言。後段network mocking未選用，不算本節API先備。 |

Comprehensive Rust zh-CN／zh-TW對應async-pitfalls與tokio頁本次網路工具無法開啟，未把失敗URL算作已讀中文。中文實讀來源為上述Book17.3，保存當下原文，不把英文摘要當中文閱讀。

## 固定API與官方source核對

| 正式來源 | 實際讀過的部分 | 寫作操作契約 |
| --- | --- | --- |
| [std1.99 Duration](https://doc.rust-lang.org/std/time/struct.Duration.html) | span角色；from_secs／from_millis、as_millis；Copy／PartialEq／Debug；不具Display的說明 | 時間跨度有單位；毫秒總值為u128，丟棄不足一毫秒部分。採小值，不用浮點轉換或nightly constants。 |
| [std1.99 Instant](https://doc.rust-lang.org/std/time/struct.Instant.html) | opaque monotonic讀值、now／elapsed、量測跨度、平台單調性限制 | 量測所包範圍的經過時間，不當年月日時鐘、不保證timer精度；不以單次elapsed當性能改善證明。 |
| [std thread sleep](https://doc.rust-lang.org/std/thread/fn.sleep.html)／[pending](https://doc.rust-lang.org/std/future/fn.pending.html) | 全部短頁：阻塞目前thread至少指定跨度／pending永不交出結果 | 阻塞負面對照不拿來安排其他工作順序；pending只作明確無法完成的受控材料。其T指定Output，不能單獨await而期待完成。 |
| [Tokio1.53.3 timeout source](https://raw.githubusercontent.com/tokio-rs/tokio/tokio-1.53.3/tokio/src/time/timeout.rs) | timeout的完整doc／建構deadline，Timeout的Future Output與poll，取消段落 | span先、future後；建立wrapper時記期限；Output為 `Result<F::Output, Elapsed>`。本版先poll內層再檢查timer，即時完成工作即使0時間仍可Ok；blocking或不交還控制權的本體不能被硬切斷。 |
| [Tokio1.53.3 sleep source](https://raw.githubusercontent.com/tokio-rs/tokio/tokio-1.53.3/tokio/src/time/sleep.rs) | sleep的完整doc／建構，Sleep對直接await的說明 | normal fn建立timer future，完成Output為unit；等待可保存工作進度；毫秒粒度及平台差異，不保證剛好指定毫秒醒來。需啟用time並在runtime內建立。 |
| [Tokio1.53.3 clock source](https://raw.githubusercontent.com/tokio-rs/tokio/tokio-1.53.3/tokio/src/time/clock.rs) | pause doc與auto-advance；advance限制198–245行 | paused clock只影響Tokio Instant，std Instant仍依真實系統。current-thread的runtime無其他工作可進行時才跳到下一timer；本節只使用自動推進，不用advance越過多個期限而猜timer已處理順序。 |
| [Tokio1.53.3 Instant source](https://raw.githubusercontent.com/tokio-rs/tokio/tokio-1.53.3/tokio/src/time/instant.rs) | 型別的時鐘角色、now與elapsed實作；elapsed取Tokio now、回Duration | 用完整路徑區別std與Tokio；受控timer測試讀後者，不把它當真實機器耗時。 |
| Tokio crate1.53.3 Cargo.toml／tokio-macros2.7.2官方快取source | time feature；test-util要求rt／sync／time；test macro的current-thread與start_paused doc | normal dependency選rt／macros／time，dev-dependency追加test-util。選定feature會提供其他能力，不代表已教其API。每個test自有runtime，start_paused需先教再出現。 |
| Tokio1.53.3 time/error.rs官方快取source | Elapsed定義、Debug／PartialEq與Display／Error實作 | 外層Err保存期限經過的原因；不是測得的Duration。用pattern分類，不用顯示字串當邏輯判準。 |

固定版docs.rs的timeout／test頁本次網路工具無法開啟，故下載官方GitHub固定tag的四個time source，**與Cargo取得的1.53.3 source逐byte相同**；下載只是取得素材，以上doc／選定實作才是實際閱讀範圍。URL、SHA與相同檢查保存 `/workspace/work/ch11-timing-research/download-manifest.json`。未把內部Pin／Context／poll implementation展示成學生已教介面。

## 作者選擇

沿用11.1的checked_total小整數與String錯誤，正常nested Result例先用立即完成工作；等待例與scope比較在paused clock驗收。借用中的狀態只在timeout wrapper消耗並drop後讀；取消保存的工作不再繼續，但既有修改保留。整段期限與每步重設的期限是不同需求，用兩段timer及完成數清楚呈現，不靠某次真實排程。
