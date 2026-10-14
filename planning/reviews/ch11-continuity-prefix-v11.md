# 第 11 章 11.1–11.7：獨立銜接 ORIGINAL 初判 v11

本輪原判：**2 項必要的先備修正、2 項可選改善**。主要 future → task → 有上限排程 → 期限 → 停止與收尾 → blocking HTTP → 單一 async HTTP 的教學鏈可由實際前文接起。這不是整章 release 判定：11.8 明確未讀，未自行執行本章 Rust tests、負例或 SIGINT／HTTP 實驗。

## 角色、版本與隔離

- 審閱者：`chapter10_concurrency`，沒有撰寫第 11 章；曾是第 10 章作者、第 9 章 TECH 原判與修後覆核者，也做過第 1、2、4 章技術審閱。已有專家知識及前文背景，**不是盲讀初學者**。目標讀者仍是基本 Python 程度、依教材實際教過的內容前進。
- 唯一第 11 章來源：`/workspace/work/tutorial-implementation/frozen/ch11-prefix-seven-for-review-v11`；7 頁、68 個 BODY 單位、66 份 archives。包含正常程式、完整答案、tests、Cargo manifests、負例及其 expected、README、8 張 SVG；負例 Cargo.lock 核對雜湊並讀取相關 package／version metadata，沒有宣稱逐行審查全部 transitive lock entries。
- manifest SHA-256：`cf51d563887f94549e571c882a593e26fca4daf1e147798963ac1ae29d9a774e`。開始及封存前重核 68 個單位、66 份來源，沒有 hash 差異。
- 未讀 peer11、FIRST11、TECH11、root disposition、作者 source-notes／checks、live 第 11 章、其他第 11 章新 gate 或 11.8。root 的進度催辦沒有附教材問題提示；本報告所有發現均來自自己的原稿與前置核對。
- 未改教材、共享 Cargo、nav、lock。原始逐單位觀察保存於 `work/tutorial-implementation/ch11-cont-prefix-v11/observations.md`；本報告收束未知時仍保留原紀錄，不回填成當時已知。

| 凍結頁面 | 原頁 SHA-256 |
| --- | --- |
| 11-01-future-and-runtime | `eda6e27b8971845c77c50bc3daa8f741d83cd5a23abfebe71af1c493cc175243` |
| 11-02-task-lifecycle | `83d22d13f08aace8621422249fbed43dcbc355627d530b8e555a968612254ada` |
| 11-03-bounded-scheduling | `3289a205b5ba861a4e9f95a8160c42ba72acff662cda5ff71e27fbe8df8ebda6` |
| 11-04-time-and-timeout | `54fd181dd1f290744f5a859c3aeac316e5559f6d8fac1a540a1d288ad9d21b47` |
| 11-05-cancellation-and-finish | `8f40f86ea4622ca508521cb45db4e9a905829d35823a425e82eff0dbed183591` |
| 11-06-http-basics | `7c9b75581105cdc5bf131b398053f2b18913312d5d0cf9f1ba46ddcebcfee4b1` |
| 11-07-single-async-http | `1be42952cd0f1569fb3b998919922ec433ae04aa87afacc3d5f00c7a6ec6727f` |

依 clear-tutorial 的 SKILL、review-protocol、Rust 術語指引核對；Q2 的「機制／性質」與「需要／用途」分開判。名稱有出現、API 能編譯、圖有檔案或作者已跑 tests，都不算實際依賴已教的證據。

## 實際閱讀路線與可用前文

按 `reading-order.txt` 完整讀 `001-001…001-009`、`002-001…002-009`、`003-001…003-009`、`004-001…004-009`、`005-001…005-013`、`006-001…006-010`、`007-001…007-009`，共 68 個 BODY 單位。取 001-009 完整單位時已見 receipt 的折疊答案，因此不能宣稱整輪答案未見；此影響據實保留。其餘單位先遮去閉合 folds 走主文，全部正文完成後再讀完整 archives／答案；沒有用折疊答案替主文補足必要機制。

前文可用的是實際教材，不是課綱。既有背景包含 author10 當時已核的 4.5、6.1–6.6、8.1–8.10 及第 10 章 actual gates；QA9 已核第 9 章原稿。這次另外實讀／回查以下必要位置，沒有宣稱重新全文閱讀全部 1–10 最新稿：

- actual 4：`ch04-publication-target-fixed-v8/002-004` 的 String／`&str` 各自比較與 `.as_str()`、以及原先完整審閱過的 trait／derive／借用查詢契約。該段沒有教跨型別的 String／`&str` equality operator；Borrow 的等價查詢規則也不能代替這項操作契約。
- actual 6：`ch06-review-fixed-v10/002-003` 的 `type Output`／`Self::Output`、`004-002` 的回傳 `impl Trait`、`005-002` 的 Iterator／`next`、`006-002` 的延後逐項 map；既有 6.1 capture、6.3 Fn、6.4 lifetime capture、6.5 owned IntoIter 與 6.6 enumerate 作為實際前文。Future 的 Output 不當成值欄位，ordinary factory 不因回傳 future 而一律 lazy。
- actual 7：`ch07-review-fixed-v13/004-005` 的 **slice** `is_empty`、6 節 clap 與 bool／positional／help 相關正文、7 節 owned Serde 模型及 `007-004` 的 from_str／Error、`008-005/006` 的 Command／Output／UTF-8／starts_with／binary path、`009-005/006` 的 library 交付與 main flush 分工。另搜尋實際 1–7 正文中的 Vec／is_empty／String equality 說明，沒有把搜到 method 名稱當成完整機制。
- actual 10：`ch10-review-fixed-v15/001-003` Send 的值移交、`001-007` Send 與 static 分開、`006-005` MutexGuard 沒有 Send／guard scope；先前 author10 的 thread join／正常 Result 與 panic／共享資料／解鎖依賴背景保留。第 11 章仍自己先教 Tokio 的 selected API，沒有把 std thread 的 handle 契約默套給 Tokio。
- 11.6 另明訂「7.9 及更早」的 Rust 入口，故獨立核對該入口；不能從完整第 11 章路線借用 11.5 已教的方法替它補洞。

正文的 shell 指令與 manifests 同樣算例子：edition／package／dependencies／features／`-p`／`--example`／`--test`／`--locked` 沿既有 Cargo 教學；新增 Tokio feature 與 attributes 在相應 source 前有角色說明。005-009 在負例命令前交代獨立 manifest／lock、空 `[workspace]` 的用途及相對執行目錄，沒有把該故意失敗 fixture 混入正常 package targets。

## 必要項 N1：11.6 三處比較暗靠未教的跨型別 PartialEq

**分級：必要修正；增加理解負擔／嚴格先備依賴。沒有阻礙 HTTP 主要概念，也不是 Rust 編譯錯誤。**

- 版本與位置：v11 `006-009` 主文 tests；`006-010` 折疊完整答案。source 對應 `sources/examples/ch11-http/src/lib.rs:123`、`examples/status_line.rs:23` 與 `:29`。
- 原句／程式：`Err(AppError::Scheme(scheme)) => assert_eq!(scheme, "file")`；兩處 `assert_eq!(status_line(&inspection), "通過：…")`／`"HTTP 未通過：…"`。Scheme 欄位是 owned String，status_line 宣告回傳 String，literal 則是 `&str`。
- 已教部分：3.4 明確區分 String／`&str` 並教 `.as_str()`；4.2 教兩種型別各自有 Debug／PartialEq，且原文說「所以稍後可以比較 `.as_str()` 得到的借用文字」。本文有教 byte array reference 與 byte slice 的特定跨表示比較，但那不等於所有不同型別均可相等比較。
- 缺少關係：兩邊各自有 PartialEq，不推出它們之間也有 cross-type equality 契約。Rust 1.99 `alloc/src/string.rs:2726–2727` 的確另外提供 String／str 與 String／`&str` 比較，證明程式有效；這份官方實作沒有在此前正文教給讀者。
- 影響：讀者須由專家常識或編譯結果補出 comparison 的額外能力，破壞「所有範例只使用當時已教內容」。答案也算例子，不能因折疊就略過。
- 最小修正：`assert_eq!(scheme.as_str(), "file")`；兩份 status_line 結果先存成 String，再以 `.as_str()` 和 literal 比較。兩側成為已教的 `&str`，保留原測試目的與完整文字核對，不增加一段新 operator 教學。
- 收益／代價：消除三個暗藏先備，僅增加明確 borrowed view；無須改 HTTP 模型、輸出規則或 source 行為。發現者為本輪 CONT，自行於讀 tests 時提出未知、回查 actual 前文與官方契約後判定；未看 peer。
- 修後狀態：未修改、未看修後稿、未複查。

## 必要項 N2：11.6 的 7.9 入口缺少 Vec::is_empty

**分級：必要修正；增加理解負擔／宣告入口的先備依賴。僅限 11.6 公佈的 7.9 入口；完整 11.1→11.6 順序路線已在 11.5 教過。**

- 版本與位置：v11 `006-001` 宣告「使用的 Rust 前置是 7.9 及更早內容」；`006-009` CLI test 使用 `assert!(!output.stderr.is_empty())`。source 為 `sources/examples/ch11-http/tests/cli.rs:28`。
- 已教部分：7.8 的 Output.stderr 是 `Vec<u8>`；3.6 有 Vec 的 `.as_slice()`；7.4 明教 **slice** `.is_empty()`，6.7 的文字 `.is_empty()` 也有指定 receiver。直到 `005-007` 才明教「Vec 的 is_empty shared 借用清單、回 bool 表示長度為0」。
- 缺少關係：7.9 入口不能使用後面 11.5 的 Vec method 教學；`Vec<u8>` 與 byte slice 不是同一 receiver。Rust 1.99 `alloc/src/vec/mod.rs:3125` 也顯示 is_empty 是 Vec 自己的 inherent method，不需要且不能以尚未教的 Deref 關係含混代替說明。
- 影響：當讀者按教材公佈的入口進入這份 synchronous HTTP 範例時，CLI test 額外使用一個未教方法；不能為此默默要求先學 async 停止排程。
- 最小修正：`assert!(!output.stderr.as_slice().is_empty())`，直接組合已教 Vec view 與 slice method；也可用已教 `.len() > 0`。前者和本頁其他 bytes view 一致。
- 收益／代價：維持既有 7.9 入口及「不用 async 就先理解 HTTP」的學習價值；沒有新增 API 或改測試觀察內容。
- 發現者與時點：本輪 CONT 讀 006-009 自行提出疑點，查 1–7 actual 與 005-007 後限定入口裁定；未讀 peer、沒有把完整順序入口誤判成缺少先備。
- 修後狀態：未修改、未看修後稿、未複查。

## 可選項 I1：把 HTTP 表格的逾時範圍就地寫窄

**分級：可選文字改善；不列必要缺口。**

- 位置：v11 `006-003` 表格「連線失敗，或等待超過期限」→「沒有取得最後的 HTTP 回應」。
- 已教／後文釐清：此時上層已提 --body，但尚未完整教 Response／text 兩步。006-005 的 selected API、006-006 的 Body variant 和 006-008 的診斷，在真正需要判 body timeout 前說清楚：已取得 status 後，讀 body 仍可能失敗；Body error 保留已有 status。
- 可選收益：把第三列改成「連線失敗，或等待最後回應超過期限」，避免首次掃表時把一切 timeout 都讀成「沒有 HTTP response」，之後再回頭收窄。
- 最小修正／代價：只限定這一列等待的對象，沒有追加 timeout 教學。這是早期表格的呈現收益，後文已在首次使用 Body error 前解釋足夠，故不升級為機制阻礙或技術錯誤。
- 發現者與時點：自己的 006-003 當時未知，讀完 006-005/006/008 追加釐清，原始未知保留。未受 peer 提示，未修後查證。

## 可選項 I2：維持 7.9 的明確 stdout flush 分工

**分級：可選一致性／設計呈現改善；沒有實際重現假成功，不能算已證實的輸出錯誤。**

- 位置：v11 `006-007` 的「接回熟悉的輸出與行程邊界」、「沿用 7.9 的輸出邊界」及 main；run／render 最後沒有 flush，main 的 `Ok(true)` 直接 SUCCESS，`Ok(false)` 直接 FAILURE。
- 已教部分：actual 7.9 `009-005` 明確分開 library 把 bytes 交給 writer 與 caller flush；`009-006` 的 main 成功後必檢查 flush 才回 0。本頁 generic render 不 flush 本身符合原 library 分工，並非錯誤。
- 理解收益：保持同一份 CLI 收尾責任，讓「library write 成功」與「caller 明確確認出口完成」的已教區別在 HTTP 素材上繼續清楚。11.7 `007-006/008` 已另外把 flush 放進 render 並驗該邊界；不能借它回填 11.6。
- 最小可選改法：沿 7.9 在 main 取得 `Ok(passed)` 後檢查 `output.flush()`，成功再按 passed 決定 0／1；flush 錯誤用現有 AppError::Output 分類。或就地明記本例沿用 library 交付邊界與 main 政策的差別。
- 限制：Rust 1.99 Stdout 的 selected 官方文件說明 terminal newline 自動 flush，實作以 LineWriter 建立；本文每次資料輸出都使用帶 LF 的 writeln。沒有執行證明現有特定 Stdout 路線丟失 bytes 或漏報錯誤，所以不以假想故障升級為必要技術項。
- 發現者：本輪 CONT 對照 actual 7.9 main 自行補查；沒有使用 TECH11／root 發現。未修改、未做 flush fixture 執行。

## 主要通過項：四題、實際依賴與證據範圍

以下是本輪自己從正文建立的依賴與通過理由。Q2M 與 Q2N 各自有判定；Q1／Q3／Q4 不代替方法理由。全部 BODY 的當時觀察另存原紀錄，這裡合併相同已教關係，保留首次需要位置及來源原承諾。

### 11.1：先保存工作，再取得結果（001-001–009）

原需 `001-001`：「當一份工作暫時沒有結果，能不能先保留它的進度，讓執行者處理其他可進行的工作，之後再回來繼續？」本節明確先分建立／執行／取結果，多工作交錯留給下一節；不能把 100 的結果當成已示範多工作排程。

- **Future／async fn／await（001-002/004）。Q1**：Future 是可保存一份工作資料及進度的值，Output 沿 actual 6.2 的 associated type；async fn 呼叫先得 future，不得 i32。**Q2M 已足夠**：引數在呼叫時求值、owned 引數先交付，本體之後才執行；await 取得 Output 且不是每次必暫停。**Q2N 已足夠到本節子問題**：要先保留工作、決定何時取得結果，呼叫與本體分開正好提供這個位置；讓其他工作前進目前合理待教於 11.2，不參與本節 100 的推理。**Q3**：async body／block 內的 `.await` 是後綴語法，不加 method 括號；未用的 future 可 drop。**Q4**：80+20、未 await 的 120 與明確 prints 分別支持建立／執行／丟棄差別，不能支持每次 await 都 suspend、加速或任意 i32 無溢位。
- **runtime／executor／attribute（001-003）。Q1/3**：executor 推進工作、runtime 提供執行環境；rt／macros、current_thread flavor、同步 main 轉入口在 manifest／attribute 首用前有角色。**Q2M 已足夠**：單 thread runtime 沒有新增 async worker threads，但 future 不是 thread。**Q2N 已足夠**：用本機可重複材料先觀察工作時點，不要求網路與額外 worker。**Q4**：本例是 immediate arithmetic，不驗外部等待；初始化 panic 與 app Result 分開。原稿沒有用 current_thread 推出所有第三方 library 永無其他 threads。
- **async block／async move（001-005）。Q1/3**：末尾決定 Output；普通 block 按 body capture，move 在建立時按值保存，仍分 Copy 值與 reference 值。**Q2M 已足夠**：借用來源有效性與 owned 資料交付沿 2／6／8 實際規則。**Q2N 已足夠**：要在稍後執行時仍持有必要資料，不能因按值保存 reference 就假造 owner。**Q4**：兩種 capture 範例足教該差別；沒有要求手寫未教 Pin／poll。
- **ordinary factory → impl Future（001-006）。Q1/3**：`impl Future<Output=i32>` 是由實作選定具體型別的能力介面，不是無型別結果。**Q2M 已足夠**：prepared_total 的普通函式立即印準備標記，再交回內部 async block。**Q2N 已足夠**：避免把 async fn 的本體規則推成所有回傳 future 的 API 都完全 lazy；後面的 sleep／send 工廠正需要這個分辨。**Q4**：立即準備與延後本體兩標記明確，不需要額外等待才能看見這個性質。
- **Output 是 Result／驗收／練習（001-007–009）。Q1/3**：await 交回 Result 後仍依 5 的 Ok／Err 處理；receipt 的 String Output 與 total 的 i32 Output 分開。**Q2M 已足夠**：負輸入仍是正常完成的 Err；drop 未執行本體的工作不增加 visits。**Q2N 已足夠**：先區分工作完成與資料可接受，收據工作等另一份工作再格式化；未先建新 task。**Q4**：-10／80、visits 0／1、完整 CLI stdout 與 receipt 次序各驗自己的關係，測試 attribute 在使用前教，題目的步驟在 fold 外。

### 11.2：提交之後，仍追蹤結果與清理（002-001–009）

原需 `002-001`：「若迴圈每次都等完一份工作才建立下一份，其他工作還沒有開始」；並要求 Err／panic／取消後仍收回每份輸入及等清理。

- **task／spawn／JoinHandle（002-002/004）。Q1/3**：task 是 runtime 排程的工作，spawn 接 Future、回 JoinHandle<Output>，不是原 future 本體；不等 caller await handle 才開始。**Q2M 已足夠**：spawn 不同步 poll，提交不保證此刻本體已開始；whole future 及 Output 都要 Send／static，即使 current_thread。**Q2N 已足夠**：先交出幾份工作，讓等待一份時其他可執行工作仍能前進；handle 保存追蹤能力。**Q4**：啟動訊息而非 sleep 證明開始；這不是多核心效能測量，也不保證 runtime shutdown 完成全部 task。Send/static、async move 不補 non-Send 或短借用都在 actual 10 與本節新界線上接起。
- **Tokio mpsc／雙 gate（002-003）。Q1/3**：MPSC 全名、端點、capacity、send future、recv Option、關閉後 drain、unit 能力與 selected errors 在 code 前教。**Q2M 已足夠**：started 送出後立即 drop 該 sender，release receiver 仍讓 worker 等待；只有 release 正常／關閉才前進。**Q2N 已足夠**：需要可重複觀察「已開始但未回成果」，兩條訊息各負責證明與放行。**Q4**：started channel 讀到 None 只證明訊息路線結束，不能代替 task 完成；本節沒有混用 std sync channel 與 Tokio capacity。
- **nested Result／Id（002-004/006）。Q1/3**：JoinHandle await 的外 Result 表工作交回與 JoinError，內 Result 表 WorkError；Id opaque、不是 input index／永久識別，traits 先選用。**Q2M 已足夠**：正常分支取得 `(Id,T)`，panic／取消從 JoinError.id 仍找 original input。**Q2N 已足夠**：即使沒有 Output 也能保存每份輸入的結果，因此必須在提交後先登記 Id→input，不靠成果自己帶 input。**Q4**：remove 對照的兩條分支都有解釋；原條目回收後移除，沒有依 Id 大小猜順序。
- **abort／JoinSet／drop（002-005/006）。Q1/3**：abort 是提出取消要求；JoinSet 同 Output、join_next_with_id 逐份回收、None 表空。**Q2M 已足夠**：取消要求後 await 到結果才可判清理；drop JoinHandle detach，drop JoinSet abort 但不 wait。**Q2N 已足夠**：本節原需等真正清理完成，所以不能只交出要求或丟把手。**Q4**：release sender 仍活、工作阻塞等 release、abort 後 wait 再 send Err，才能把收端清理與 route 意外關閉分開；非 preemptive 同步工作例外有角色，不執行無限反例。
- **四項受控回收／tests／練習（002-007–009）。Q1/3**：每個 input 的 Value／WorkError／Panic／Cancelled 對應及 panic stderr／status0 的 application policy 都有先教。**Q2M 已足夠**：放行1、放行2、取消3、放行0，每步回收才往下。**Q2N 已足夠**：完成順序與輸入順序分開仍不失身份，並以 channel closure 練習區分正常 Err 與取消。**Q4**：固定輸出來自 gate，不是 Tokio 公平排程承諾；unit／CLI各驗材料已宣告的部分，fold外步驟足夠。

### 11.3：有限 task 狀態與完成後補工（003-001–009）

原需 `003-001`：「即使有很多輸入，也只讓上限內的 task 交給 runtime；收回一份，才提交下一份。」上限是**已提交、尚未回收**，不能縮成正在跑 body 或 whole-program memory。

- **Job／source／IntoIter（003-002–004）。Q1/3**：Job 是尚未造 future 的 owned 原始工作材料；兩份 Vec 的同位置 Job／release sender 由同一 push 順序建立。owned IntoIter.next 沿 actual 6.5；tracked／submitted／peak分開。**Q2M 已足夠**：剩餘資料留來源，只有 vacancy 才建工作狀態。**Q2N 已足夠**：避免一開始為所有輸入建立 tasks，保留尚未提交者供逐次補工。**Q4**：五筆 input 表先給完整成果規則，沒有把來源 Vec 本身宣稱 constant memory。
- **zero limit／new／fill（003-004/005）。Q1/3**：ConfigError、正空輸入、各欄位／method 改變先給。**Q2M 已足夠**：len<limit 才 spawn 一份，每次 +1 後重查，已完成未回收仍佔 len；zero 在第一次提交前拒絕。**Q2N 已足夠**：零上限沒有第一份工作可以回收騰空位，拒絕有工作理由，不是任意魔術數；正空輸入則可以正常結束。**Q4**：正文 proof 和 tests 各支撐條件，不以一次測量 peak 代替全部保證。
- **next／refill／Batch（003-006/007）。Q1/3**：先 collect／移 Id→input，取得 report，再 fill，最後交 report；圖②是 method 內暫存，③才補工返回。Batch 沒有活 task。**Q2M 已足夠**：回收釋出一格，待補來源未耗盡就填新一份。**Q2N 已足夠**：input0仍等待時，1先完成便可讓2前進，不用等整批最慢工作才開始下一批；不是只聲稱新方法較好。**Q4**：error／panic 也騰位，受控 example實際使作用可見；沒有量測 throughput／公平政策。
- **控制 sender 生命週期／驗收／limit1（003-007–009）。Q1/3**：保留原 started sender 因後續 fill 要 clone，不能沿前節過早等 started None；最後才 drop 並確認 routes、map、set 結束。**Q2M 已足夠**：limit1 每次只有一個 submitted unreaped，故特例依 input 完成。**Q2N 已足夠**：同一上限政策下 error仍補下一份，有限來源全處理完再交 Batch。**Q4**：limit1不能推出 limit>1固定順序；各fold外引導與數量 tests足教該特例。

### 11.4：等待範圍與時間範圍（004-001–009）

原需 `004-001`：「願意繼續等多久？到了期限，應該交回哪種結果，哪些已經做過的事會留下來？」沿用小計材料，先隔離網路與 timer 的不同變化。

- **Duration／Instant／sleep（004-002）。Q1/3**：非負跨度與單調鐘位置分開，from_secs／from_millis／as_millis 的 u64／u128、精度及 elapsed 範圍先教；Sleep 普通工廠建立時設 deadline。**Q2M 已足夠**：建立資料／推進等待／重新安排納入 start 到 elapsed；和 asyncfn 本體 lazy不同。**Q2N 已足夠**：先能表達願等跨度及觀察範圍，才設定限定等待；不用年月日 clock代替。**Q4**：20ms不是固定硬體實測毫秒，沒有 exact elapsed assertion。
- **timeout／pending／nested Result（004-003/004）。Q1/3**：wrapper接工作、交 `Result<T,Elapsed>`，Elapsed不是Duration；pending<T>永不完成是需要期限的受控材料；兩層 Result各問不同問題。**Q2M 已足夠**：本版本 poll inner-first，已 ready 的成果優先；完成 Err不等於 timeout。**Q2N 已足夠**：保留原 Output 的資料判定，同時讓 caller 得知「沒有在此等待得到結果」。**Q4**：ready100／pending、-10／80分別隔離 readiness與資料正確性；官方固定1.53.3原始碼已核，不依舊版簡述。
- **取消／borrow／同步阻塞（004-005/006）。Q1/3**：wrapper取得inner，結束後 drop inner 讓 visits借用結束；Std thread sleep普通同步阻塞，非asyncsleep。**Q2M 已足夠**：visits已+1不會回滾；30ms同步本體沒有把許可權交還，1mstimeout不能硬切，返回ready成果。**Q2N 已足夠**：要知道哪些修改留在原資料及期限保障的邊界，才能解讀結果，不把取消當交易回滾。**Q4**：有限30ms例只教非preemption，不宣稱所有blocking必成功，也未跑無限loop。
- **paused clock／test-util（004-007）。Q1/3**：start_paused current_thread、TokioInstant與StdInstant及feature在tests前教。**Q2M 已足夠**：runtime沒工作時auto-advance到下一timer；std wallclock不跟著變。**Q2N 已足夠**：不靠現實機器幾毫秒排程猜結果，在明確只有timer的材料上驗等待關係。**Q4**：合法睡10／短期限與負輸入elapsed0有各自因果；未推多IOtask／equaldeadline的未測結論。
- **whole budget／每步 budget／練習（004-008/009）。Q1/3**：包整段與每步重給budget的位置先明示，completed表示已完成步數。**Q2M 已足夠**：30+30／whole50第二步被取消，completed1；每步50兩步完成。**Q2N 已足夠**：總工作上限與每次操作各自可等多久是不同需求，本段直接連起兩種選擇，不宣稱其中一種普遍更好。**Q4**：圖是手工timer示意、非量測；20/50、30/50、30/10的2／1／0步與是否Ok既有規則即可預測，取消不歸零。

### 11.5：停止輸入、保留結果、真正收尾（005-001–013）

原需 `005-001`：「停止後仍交回完整報告：哪些已完成、哪些回錯誤、哪些被取消、哪些根本沒提交。」因此本節同時處理停止觀察、工作生命週期及每個 input 的最終分類。

- **select／biased／cancellation safety（005-002）。Q1/3**：同task等待分支、binding=future=>handler、不是forktask，候選future建立和handler執行分開。**Q2M 已足夠**：選中一個後其餘waitfutureDrop；recv／join_next_with_id 的 selected cancel-safe意思是沒移走message/task，不泛化所有future。**Q2N 已足夠**：等待結果時仍能接停止事件；兩者同時ready先stop是應用政策，之後仍回收已完成成果。**Q4**：沒有把bias當公平機制、stop瞬間全禁止或所有副作用rollback。
- **Outcome／EndReason／borrowed join（005-003/004）。Q1/3**：Unstarted沒TaskId，不能abort；停止原因與每份成果不同enum。borrowedjoin的等待只借集合，map/set對象原樣保留。**Q2M 已足夠**：丟掉joinwait結束借用，不丟set／task；放行後還可Value80、再清map/set。**Q2N 已足夠**：避免把「停止選擇等待」誤作「全部工作已取消清理」，這是上層完整收尾問題的必要子區別。**Q4**：圖跟蹤同一份JoinSet/input0，controller也await，不靠相同80數值認定同一task。
- **Runner／abort-drain／pending cleanup（005-005/006）。Q1/3**：outcome slots按輸入位保存None/Some，new不fill；abort_all不移／不wait，迴圈join，pendingJob只dropreceiver記Unstarted。**Q2M 已足夠**：保留已回正常Output，其他真實JoinError分類；最終set/map空，owned enumerate按input序交Batch。**Q2N 已足夠**：要交完整輸入報告，不只顯示已啟動者；未提交者沒有任務可取消，已完成者不能被停止政策覆寫成Cancelled。**Q4**：處理順序圖與正常/錯誤/取消/未提交材料實際對到該需求，沒有宣告rollback外部來源。
- **try_recv／check-before-fill（005-007）。Q1/3**：Empty／Disconnected／buffer事件優先、同步非等待、Vec／JoinSet is_empty在code前分別教。**Q2M 已足夠**：每次補工前看已經排入stop，取結果後先record再回loop，暫未觀測的event仍可能與submission交錯。**Q2N 已足夠**：改善先fill再select會忽略已排入停止事件的具體情況，不把tryrecv的性質反推成原需求。**Q4**：prequeuedstop練習使new不fill的作用可見；不是訊號到達瞬間的原子禁止。
- **受控stop／guard（005-008/009）。Q1/3**：controller返回仍活releaseSender防ControlClosed，收尾後senderErr驗receiver清理；actual10.6guard!Send沿wholefuturebound。**Q2M 已足夠**：1WorkError/2Panic留下、0/3取消、4未提交；短block拿Copy快照解鎖後await。**Q2N 已足夠**：保存全部輸入且不留任務；此處只需要數值快照，不需要等待時持鎖。**Q4**：guard例單owner不會證明多worker更新政策；compile-fail指明await後的guard仍用、selected bound失敗；未將過早解鎖改成永遠不鎖。
- **ctrl_c／listener-ready／CLI boundary（005-010/011）。Q1/3**：signal feature、ioResult、SignalFailed、firstpoll註冊、Unixhandler不恢復在use前給出。**Q2M 已足夠**：bias先pollctrlc，再entered支路send+pending；jobStarted與signal註冊不是同一證據。finishcontrollerabort且await，observer/controller和batch都join再出口。**Q2N 已足夠**：把真人停止轉換成同一種event，同時處理正常結束時listener清理；READY需既定註冊與兩工作啟動條件，不靠固定sleep。**Q4**：OS註冊失敗的event模擬只驗應用政策；正文作者實測SIGINT記載不當作我的probe。argumentsString/nonUnicode與args_os區別在前，用0/1/2/130應用exitpolicy不作HTTPstatus。
- **controlled tests／returning／練習（005-012/013）。Q1/3**：seven關係表、CLI狀態/完整stdout/診斷及prestop步驟都足夠。**Q2M 已足夠**：returning job在此current-thread送started後沒有其他await，立即回Output；done但unreaped仍佔set，這個受控前提解釋測試。**Q2N 已足夠**：區分已完成成果保留與取消未完成者、在提交前停止與先啟動再取消，回收原完整報告問題。**Q4**：不把任意send成功當全部工作完成證據，不把測試名稱當機制解釋；fold外指引足夠，未提交senderErr不證明曾建task。

### 11.6：先把 HTTP 意義與兩個完成界線教清楚（006-001–010）

原需 `006-001`：「送出請求，知道對方回了什麼，也能分辨『對方說找不到』與『一直沒有取得回應』。」本節 synchronous單次請求，明定7.9入口；可以先學HTTP，不先要讀者掌握async排程。

- **URL／HTTP(S)／response（006-002）。Q1/3**：全名、scheme/host/port/path/query/fragment、GET/headers/status/body及HTTP1.1 CRLF示意先給；path非localPath，fragment不傳server；TLS有足夠角色而不提前cryptography。**Q2M 已足夠**：client按URL請求，server以status/headers/body回答，reqwest負責parse和protocol。**Q2N 已足夠**：需要辨別所請求資源及server實際答案，而不是把URL當檔案路徑或把body當status。**Q4**：示意明標HTTP1.1 representation、較新版本形式可不同，沒有要求手拼網路bytes。
- **200–299／404／redirect／exitpolicy（006-003）。Q1/3**：HTTPstatus u16不同processu8，404有回應但不pass；302Location換請求，報告最終URL，5最多／第6錯誤在fixedversion已查。**Q2M 已足夠**：正確持有Response才判斷status；有限redirect與期限約束等待。**Q2N 已足夠**：單一exit1不能抹掉失敗階段，report stdout與診斷stderr保存兩類原需。**Q4**：200只支持HTTP成功類不支持body模型正確；上述表格範圍有I1可選改善，不從未教HTTP內容推body保證。
- **features／Settings／selectedbuilder（006-004/005）。Q1/3**：reqwestblocking/rustls/defaultfalse/charsetoff、clap選項、proxy/direct、builderbyvalue、Urlparse規範化、statusCopy／u16、textconsumes與Errortraits都先教再code。**Q2M 已足夠**：send取得最終Response；text是另一步body讀取，blocking各新10秒預算；UTF8lossy不同std嚴格UTF8。**Q2N 已足夠**：default只要HTTPstatus無需主動讀完整body；--body需要另一完成條件，能分別解釋等在哪裡及原因。**Q4**：5redirect與10秒都是宣告政策不是網路觀測，公共URL非固定答案；no_proxy只解釋排查環境，不宣稱所有環境都可直接連線。
- **Inspection／AppError／執行分工（006-006–008）。Q1/3**：owned報告保留URL/status/bodychoice；orpattern/Body{..}在首次使用前給；parse→build→send→copydata→text各錯分類。**Q2M 已足夠**：None沒有要求讀body，Some(empty)真的讀完空內容；提前copyowneddata再consumeResponse，Bodyerror還保已有status。**Q2N 已足夠**：既要分HTTP未過與請求失敗，又要知道headers完成和body完成不同；保留階段variant沒有把相同外error型別flatten掉。**Q4**：圖兩條完成界線只說程式要求，不保證wire尚無任何bodybyte；知道大小的text頁/no_size_cap與後續headers工具分工有明說，不作安全/任意大型body承諾。輸出分工有I2可選一致性項，未否定write失敗傳遞。
- **offline驗收／byte literal／格式練習（006-009/010）。Q1/3**：bytearrayreference與slicecomparison在test前最低契約給出，&&str helper可作Commandarg有明說；starts_withactual7.8已教。**Q2M 已足夠除N1/N2**：模型404仍正常報告，bodyflag改變輸出內容，parse相對URL與unsupportedScheme不同inputError。**Q2N 已足夠**：不用網路便先驗輸入/報告/CLI邊界；status_line是給定報告摘要，不偽造收到HTTP。**Q4**：stdout完整bytes／status2／noReport不證明redirecttimeout網路行為；完整答案仍有N1跨型別比較待修。

### 11.7：保留 HTTP 規則，只改等待的表示（007-001–009）

原需 `007-001`：「先能指出『現在等的是 response，還是完整 body』，並固定每種結果與期限的意思，後面才把多份工作組合起來。」原文明確一次請求的synchronousCLI也可完成，沒證明async更快。

- **runtime／Client／future分工與features（007-002）。Q1/3**：runtime驅動IO/future，Client存settings/可用連線pool，build即時普通函式，inspectasync保存borrowedclient/ownedUrl/選項；net/time/rustls和SerdeJSON各能力在manifest前。**Q2M 已足夠**：buildclient與inspect分開可給caller重用；Client不是runtime/Response。**Q2N 已足夠**：同一HTTP工作改成可保存等待進度的形式，後續才有多份組合需求；現例不靠pool重用效果說明async優點。**Q4**：單請求沒有測pool／交錯／效能，不借9featureunion誤推每feature單獨enable。
- **總requestdeadline／send與text（007-003/004）。Q1/3**：兩個future Output/consume/save步驟明確；普通sendfactory不同asyncfn本體lazy，totaldeadline在fixedsource建立。**Q2M 已足夠**：一次請求從準備send計時到完整body共用10秒，Response攜同期限；blocking兩步各10秒不同。**Q2N 已足夠**：同一個總request預算不能在讀body時重置；先約定包哪份工作避免只看有timeout名稱。**Q4**：4秒+剩餘6秒是示意，parse/build/output在budget外、每request各自deadline不globalClientclock；沒有硬preemption保證。
- **ownedreport／elapsed／typederrors（007-005）。Q1/3**：start在GET前，elapsed到網路選擇與報告準備後，u128/nonfixed；ownedString不會借已consumeResponse。**Q2M 已足夠**：sameerrorType仍按Client/Request/Body步驟分類，Body保status。**Q2N 已足夠**：保持上一節HTTP語意的同時知道等待階段與量測範圍，不為了方便From丟失原因。**Q4**：42ms手動示意不作固定test／async更快證據，明說測量含本地處理而不僅wire時間。
- **run/render/main／tests（007-006–008）。Q1/3**：render普通Write包含flush，runawaitinspect再render、mainFutureOutputExitCode，沿clap/IO/Output前文；RefuseFlush方法契約前7已教。**Q2M 已足夠**：只有全部輸出成功才返回passed，flush失敗AppErrorOutput；invalidinput發生在client前。**Q2N 已足夠**：async入口仍要保正確CLI出口，不把HTTPpass代替輸出完成；網路還未受控所以先離線驗證。**Q4**：RefuseFlush證明宣告的writer邊界，2CLI/4library測試不是HTTPintegration；Stringscheme用as_str、Vecstderr用as_slice().is_empty足保這裡先備。
- **HTTP→body→JSON模型（007-009）。Q1/3**：PageRecord ownedDeserialize/denyunknownfields/from_str來自7.7；ReplyError先HTTP、再Bodychoice、後Json。**Q2M 已足夠**：200invalidrevision仍可能JSON失敗，404不先解JSON；成功模型可活過Inspection。**Q2N 已足夠**：HTTP成功不代表應用內容符合schema，要把兩個問題分層回答，這是11.6已宣告HTTP判準的合理深化。**Q4**：200/"3"/None/404給足兩層關係與ownedness，不作為網路到達證據；fold外steps/hint夠指引，server可靠測試於11.8合理待教且本輪未查。

## 小變化與主張範圍

本輪使用正文已給的小變化檢查遷移，不另造必須懂的未教API：未await的120不執行body；started route關不等於task完；正常ControlClosed是Output不是JoinError；limit1負輸入仍補第三份；wholebudget的2/1/0步不把取消視為回滾；prequeuedstop沒有任務可abort；404正常Response與200錯誤JSON分兩層。它們各支持對應機制與需要，不會因為能算這些結果就自動放行pool效能、排程公平或真人SIGINT可靠性。

我另以已教條件問：「limit2的一份body已完成但還未join，能否先補第三份？」答案不能，僅len2仍佔位；依據003-001/002/005/006的submitted-unreaped規則。題目由我加，不能算作者本來有一份額外實驗；撤去題目，作者正文與圖②→③的解釋仍足。用途由原上限需求支撐，不是題目補教才得出。

## 獨立實頁／圖／fold檢查

本人建立只含v11七頁的獨立preview；從frozenBODY重建頁面，刪除重複H1，對映本snapshot的8SVG絕對路徑為previewassets。沿前次本人9v6preview的同theme及其已封存CSS，explicit extra_css啟用 `.md-typeset details { font-size: inherit; }`；沒有讀取live11正文／作者站點修改。build strict config exit0僅證明該重建preview能建，不能代替閱讀判斷。

本人親看 **78張不同screenshots**：desktop1280×800、mobile390×844，Chromium，DSF1、字型ready。組成28張七頁entry/end、32張八圖原位context/element、18張九處展開fold。初始fold依稿閉合；視覺展開是完整BODY/source讀完後的獨立步驟，不回算盲讀。前輪24張確認親看，最後8張context工具輸出曾被截斷而沒有計入；恢復後小批補完54張，含重新開啟這8張。具體圖hash、screenshots及viewlog保存在本輪work。

| 圖與原位置 | Q1／3／4所需對應與本人畫面結論 |
| --- | --- |
| future-lifecycle，001-004 | 呼叫得future／await執行body／取得100與未await丟棄分開，箭頭是本例次序。圖與文字鄰接；desktop/mobile labels可讀。不能由圖推出await必suspend。 |
| task-result-identity，002-006 | Id→input對映、正常(Id,Output)與JoinError.id兩路最終回originalinput；身份與完成結果不會混成同一編號。圖／caption／source一致可讀，不把Id0等同input0。 |
| bounded-refill，003-006 | ①兩項佔位，②回收取得report後的method內部狀態，③填新job後返回；圖顯示補2時0仍在，文字交代②/③時點。這個關係實際回應不等最慢同批的用途；未當吞吐量圖。 |
| timeout-scope，004-008 | 同材料30+30、整段50與每步50，第一條第二步取消／completed1，第二條completed2；預算框覆蓋的對象不同。手工timer限定就近可見，單位/箭頭可讀，非實測。 |
| borrowed-join-wait，005-004 | 橙框drop的是借用JoinSet的等待future，藍框同一集合/task還在，綠框重新join得Value80；顏色/箭頭與caption指回本例順序，不能讀成drop整個set。 |
| stop-and-finish，005-006 | observe stop→abortall→join/drain→pending未提交→完整Batch；藍已提交與灰未提交不同職責，綠最終各input有分類。未把Cancelled統一覆寫已完成成果。 |
| http-response-boundary，006-006 | 預設可在headers/status後交report，只有body選項走下一步、另10秒；同GET不等於HEAD、不保證wire無body。圖原位caption說明程式完成條件。 |
| single-http-budget，007-004 | prepareClient/Url預算外，send→Response保存→可選text同虛線總10秒，ownedreport結束量測；兩份future不等於兩份budget。desktop/mobile都能定位不同框與箭頭。 |

七頁標題／入口問題／收束下一步可見，fold開啟後字級與normalcode一致（記錄12.8px），不會因完整答案整體縮小而另加閱讀負擔。Mobile程式碼長行保留橫向滾動，截圖只顯示當前橫向view，不聲稱每條長行已同時完整展示；全文已從frozentext讀過。展開fold的作用是檢查閉合位置／視覺字級與內容邊界，不是逐畫素檢查fold全部滾動高度。

少量context截圖的stickyheader標題處在滾動切換淡化時點，截圖上方還有上一段被裁切；圖本體所需文字沒有被遮住，完整element另親看。沒有據此報持續遮擋。圖稿字型大小不一，但本輪實際尺寸關鍵文字可辨，不能僅因未一律24px就判失敗。未看darktheme、其他瀏覽器、assistivetech、全站nav/部署或11.8。

## 官方契約、執行與限制

選讀官方來源是在完整主文觀察保存後的CONT補查，不能說成普通讀者只靠主文已讀取這些來源。固定Tokio1.53.3／reqwest0.13.5／Rust1.99來源存於本輪 `contracts/`，本人實讀以下selected位置：

- Tokio spawn.rs:23、174：不同步poll；Future及Output的Send/static。
- runtime/task/join.rs:16–36：工作無需先await才開始；destructor完成觀察、handle detach、borrowedawaitcancel-safe。
- task/join_set.rs:300–317、461–464、591–594：join_next_with_id形狀／取消安全、abortall不remove或wait、drop提出取消。
- sync/mpsc/bounded.rs:199–204：recv的cancel-safe確切是未取訊息，不是無副作用萬能保證。
- signal/ctrl_c.rs:20–34、60–63：Unix handler 行程級且drop不恢復，asyncbody首poll呼叫註冊；沒有把啟動工作當註冊證據。
- time/clock.rs:118–125：pausedclock current_thread／idleauto-advance。
- time/timeout.rs:205–221：此版本inner先poll，Ready返回Ok，後polltimer；有限同步例的結論不靠跑通替代理解。
- reqwestredirect.rs:131–140：`previous.len() > max`且首URL是初始，5次/需第6失敗與當前版本一致。
- async_impl/request.rs:521–525、async_impl/client.rs:2670–2694與3139–3144、async_impl/response.rs:36–45：send普通factory立即execute_request／建立總Sleep，Response接續同期限；不是每個text重設10秒。
- blocking/client.rs:1493–1527與blocking/response.rs:295–299：sendwaittimeout後Response保存duration，text另呼叫waittimeout；bodyfreshbudget與async不同。
- async_impl/response.rs:163–174：charset關閉時bytes→UTF8lossyownedString，符合本文並非stdstrictUTF8的限制。
- Rustallocstring.rs:2726–2727／vecmod.rs:3125–3126／stdstdio.rs:588–591與720–723：N1有效但未教的cross-typeoperator、N2receiver、I2現有stdout範圍。

本人本輪實跑僅**獨立preview build與Playwright截圖**，沒有跑全WIPworkspace、正常examples、expectedfail、SIGINT或公開HTTP。沒有自己的Ruststdout／失敗cause實驗可報，也不借root或作者PASS作本人probe。Negative機制及stdout預期由正文/完整source/selected契約核對，實際執行結果本輪未驗證。不會執行無限pending、deadlock或以公共HTTP回應湊固定答案。

未聲稱全面檢查全部第三方內部API／memorymodel、任意網路環境或所有HTTP協議情況。HTTP integration／controlledserver是11.8的明確下一步，只核其預告角色，不開啟正文或helpers來補11.1–7。真人目標讀者反饋本輪沒有；這是獨立非作者的expert CONT審閱。

## 封存

原判與同位元組initial副本、SHA sidecar保存後停止；peer cross／root裁定／11.8尾段尚未開放給本人。本報告2必要項保留對應入口及影響，2可選項保留理解收益及不升級理由；之後即使稿件修改，原位元組與分級不回寫。
