# 第 11 章整章 CROSS／修後銜接覆核（v15）

本輪修後的必要問題已補足，未新增必要銜接或版面缺口。新增一項可選文字修正：local-service 的 README 型別表示少一個結尾 `>`；正文與實作正確。本結論限本輪凍結稿與下述實讀、契約、實頁範圍，不是整章發布或重新執行 Rust tests 的通過宣告。

## 角色、原判與閱讀順序

我是未撰寫第 11 章的 CONT 審閱者，先前有第 10 章作者與第 9 章 QA 背景。原 CONT prefix／tail 都已各自獨立封存；本輪是互看後的交叉與修後核對，沒有重新盲讀，也沒有把同行提示改成我的 original 發現。

- 原 prefix：`planning/reviews/ch11-continuity-prefix-v11.md`，43,412 bytes，SHA-256 `4529c857015481f5d31db4adbf31e44b03424b5a07bf1fbb2d9aa5d6c2bf9ad6`。
- 原 tail：`planning/reviews/ch11-continuity-tail-v13.md`，27,448 bytes，SHA-256 `015c3767166eed36c2f688cebcb70015bf7ffa9e2248090a8eedf1bbc166ea4c`。

兩份原稿的 bytes、分級與限制不改。依 clear-tutorial 的 review-protocol、術語指引核對；讀者仍只有基本 Python 背景，範例、指令、manifest、helper、test、閉合答案與 negative fixture 都算當下先備。名詞、目錄、能編譯或作者已跑 tests，不能證明教材已教。

先全文讀以下五份同行 ORIGINAL，保存自己的交叉判斷，再開 v15 BODY／source，保存修後與本人實頁判斷，最後才全文讀 ROOT disposition。少數大型輸出曾截斷，缺失範圍另開補讀；沒有以截斷輸出當全文完成。

| 同行 ORIGINAL | SHA-256 |
| --- | --- |
| TECH prefix `ch11-technical-prefix-v11.md` | `9d3d071f09f59016cb058983b816763d2da71782cd2d5b22b704384047a9b72d` |
| TECH tail `ch11-technical-tail-v13.md` | `fdefe62ca564961c7e9d2fe3b95c69b96776928c9af7468580f077acfabbfe9e` |
| FIRST prefix `ch11-first-reading-prefix-v1.md` | `7b4b240c3418ff2bb90968325484e131bb0cfe571f7d9aaf14e2a3286777489d` |
| FIRST middle `ch11-first-reading-middle-v1.md` | `2840c746ebe239364dff71780a37e3c9af01943035c45c1dc089c461f92d9831` |
| FIRST tail `ch11-first-reading-tail-v1.md` | `c8f1ef0667df646d8607f096867a456ee55ee9271378c3ec93305a8640f24373` |

FIRST 前兩份依各自 v7／v10 首讀路線；CONT／TECH prefix 原判依 v11，tail 三角色依 v13。位置和語義可比較，不能默認所有 BODY hash 相同。FIRST 未開閉合答案或全部 source，isolated SVG 也不是實頁；TECH 的有限 probe、本人原 CONT 的 source／答案與原位圖文證據各保留自己的範圍。FIRST 沒報必要缺口，不等於全部 API 首用、答案與清理失敗分支已由它驗證。

原 CONT 依賴背景仍限原報告實讀內容：1–8 的必要語法／trait／ownership、實際第 7 章工具、第 9 章原稿與第 10 章 actual gates；沒有宣稱重新全文閱讀所有最新 1–10。這次對修法另外重開 7.4 的 slice `is_empty`、ErrorKind／`io::Error::from`，並核 7.9 的 writer 交付／main flush 責任。第 11 章自行教 Tokio、reqwest、Axum 的 selected API，不把 std thread 的 handle 契約直接套給 Tokio。

## 凍結範圍、實讀與版本差異

唯一修後稿：`/workspace/work/tutorial-implementation/frozen/ch11-review-fixed-v15`，8 頁、81 BODY、76 archives。manifest SHA-256 是 `6ffcc35ddcc04c2268c2d319f060df91550a142876d90f76091bec43ddbd9eb4`。81 BODY 依 reading-order 全文實讀，包括所有完整答案、tests、命令、功能說明表與修後新增段；所有 BODY／archive 重算 hash 均符合 manifest。

相較本人已全文實讀的原 prefix／tail archives，66 檔同 hash、9 檔變動、1 檔新增；9 張 SVG 全部同 bytes。9 個改檔與新檔全部本輪全文讀：

- `ch11-http` 的 `src/lib.rs`、`tests/cli.rs`、`examples/status_line.rs`。
- `ch11-async/local-service` 的 `src/lib.rs`、`src/main.rs`、`tests/http.rs`、`examples/basic.rs`、`examples/closed_gate_solution.rs`、`README.md`。
- 新增 `ch11-async/local-service/tests/absent_entered.rs`。

其餘 66 檔包含正常 source、完整答案、helpers、Cargo／lock、negative source／expected 診斷及圖；沿原報告已實讀範圍與這次同 hash 證據，沒有冒稱再次全文重讀或重跑。另完整重開同 hash 的 local-service Cargo 與 CLI test，核對正文所列依賴、features、真 binary 與固定 stdout。源碼 imports 的排列不代替正文教學次序。

本輪不執行 Rust、公開 HTTP、無限 pending／deadlock 或整個 WIP workspace。新增 official read-only 核對保存到自己的 contracts：Tokio 1.53.3 `runtime/task/join.rs` 的無新 bound 方法／Future 實作及取消契約；本人取回 Std 1.99 `io::Error` Rustdoc，確認 Send／Sync。Axum 0.8.9、reqwest 0.13.5、Tokio pause／resume 與 IntoFuture 的必要固定契約沿本人原判所存原始來源，沒有把同行或 ROOT 的實跑聲稱自己的 probe。

## 交叉判斷與修後覆核

### 1. 三處文字比較：原 CONT 必要，修後已足夠

原 `006-009` scheme test 與 `006-010` 完整答案的兩處摘要比較，使用 `String` 與 `&str` 的異型 PartialEq。實際前文教 `.as_str()` 及兩種文字型別各自的能力，沒有教這份跨型別比較；byte array reference／byte slice 的特定比較不能代替它。TECH 未單列、FIRST 未開閉合答案，也不能推翻原需。

v15 在 scheme 與兩個 status_line 結果上先 `.as_str()`，正文同步說借讀再比較。兩側成為已教的 `&str`，未增加新的運算子契約。來源三檔與完整答案均已實讀，修後收益是排除暗藏先備，HTTP 分類與摘要練習不改。

### 2. Vec 方法：原 CONT 必要只限 7.9 入口，修後已足夠

`006-001` 明訂 Rust 前置為「7.9 及更早」。原 CLI test 的 `output.stderr.is_empty()` 不能借後面完整第 11 章路線的 `005-007` Vec 方法說明來補洞；當時 7.4 教的是 slice `is_empty`，3.6 教的是 Vec `as_slice`。不以未教的 Deref 或同名方法含混放行。

修後 `006-009` 與 `ch11-http/tests/cli.rs` 是 `output.stderr.as_slice().is_empty()`，明確組合已教 view 與 slice 方法。正常完整 11.1→11.6 的讀者原本已由 11.5 學 Vec `is_empty`，所以 FIRST 不報同一缺口與本限定不矛盾。原問題不擴成所有讀者都缺 Vec 能力。

### 3. flush：保留原分級差異，接受修法的責任收益

TECH prefix N1 列必要：從 7.9 的輸出完成分工及 11.6「輸出失敗回 1」重建責任，未明確 flush 留下泛型 writer 的完成邊界。其 std-only 人工 Write probe 接收 67 bytes、flush 呼叫數為 0，明確 flush 才交錯誤；這不是實際 Stdout 失敗。

本人原 CONT I2 列可選一致性建議：泛型 library 可以只負責交付 bytes，Std 1.99 的 Stdout 使用 LineWriter，而本例 writeln 每行有 LF；本人沒有重現「真 Stdout 顯示成功但寫出失敗」。兩角色對責任文字範圍與必要門檻的理由不同，不能互看後回寫成同一必要 original。

v15 `006-007` 在 code 前明訂「寫入與 flush 一起交給 render」，只有兩者完成才交 passed；實作末尾 `output.flush()`，run 包為 AppError::Output。`006-009` 在新增 RefuseFlush code 前說明 write 保存 bytes／回長度、flush 以已教 PermissionDenied 回錯誤，測試核錯誤與非空資料。用途接上「交付 bytes 不等於這份輸出已完成」，沒有把人工 writer 當磁碟、網路或真 Stdout 故障證據。修後責任明確、避免 caller 猜測，本人保留原可選分級。

### 4. pattern／Cargo／HTTP 表：局部最低說明有收益

FIRST middle 在原 11.5 完整 main 的單一 `|` 處有當下未知，從兩個分支共 handler 猜其用途，至 11.6 才獲明說；FIRST 沒回查全部 actual 前文，不能把未知回填成必要獨立重現。v15 `005-011` 在首次 match code 前給 `p1 | p2` 任一 pattern 共 arm，對應 SignalFailed／StopSourceClosed，並區別 boolean `||`。最低關係已足，不需搬整套 pattern 課程。

TECH prefix 的 manifest-path 為原可選名稱負擔。v15 `005-009` 在命令前明說指定檢查的 Cargo.toml，路徑以本節 package 目錄起算，選獨立 fixture；原已有獨立 workspace／lock／target 用意仍在。命令、negative source／lock 同 hash，沒有引入另一份 Cargo 設計。

本人原 HTTP 表 I1 亦可選：後面的 selected API、Body 保存 status 與診斷，在真正判 body timeout 前已釐清不同步驟。v15 `006-003` 第三列現在限定「等待最後回應超過期限」，所知是沒有 final Response；`006-005/006` 仍分開後續 text 期限與 Body 錯誤。窄表排除過早讀大的範圍，不代替完整 HTTP 錯誤分類。

### 5. Client 能力：原 CONT tail 必要，修後首次 spawn 前已教

原 Send／Sync 出現在後面第一份 clone 前，但 `008-008` 先有 owned Client 的 spawn 與 inspect 的 `&Client` 跨 await。ownership 保活不能創造 Send；10.1 的 shared reference 關係仍需要所選 Client 的 Sync 契約。TECH／FIRST 的正常流程 PASS 未單列此 foreign type 首用前提，不代表已由前文提供。

v15 `008-008` 第一段在首次 client task code 前明說 reqwest 0.13.5 Client 有 Send／Sync，owned move 不是自動變 Send、Sync 支援這份 `&Client` 的 Send，並給 Url／owned Output 的移交能力。`008-010` 才在首次 clone 前教 Clone 共享設定與 pool。沒有把 clone 換成獨立 pool，也沒有把一個可跑結果代替 trait 能力說明。原必要修妥。

### 6. chosen server future／IP／helper 表：保持各自可選或未知的範圍

本人原 ServeFuture 能力 I1 可選。原文沒有聲稱任意 IntoFuture 都能 spawn；所選 Axum 實作能轉成 Send／static Future，仍需就地接回 11.2 的 bound。v15 `008-003` 在 basic code 前限定選定 `service.into_future()` 的 Send／static 與 Output `io::Result<()>`，`io::Error` 有 Send，所以 owned 結果能移交；仍分 task 與工作兩層，不引入內部 BoxFuture／Pin／tower，也不泛化一般 IntoFuture。

FIRST tail 的 IP 全名是原未知，loopback／port 的定位角色原已可理解；不升必要。v15 同段補 Internet Protocol 與 URL 目的位置／服務入口，沿已有 SocketAddr／保 listener owner 的用途即可。

TECH tail helper 表 I1 是可選呈現負擔：390px 原 `next_started` 等名稱被拆開，需拼回識別字。本人原 12 張 tail 畫面沒有截此表，不能借原 entry／figure／fold PASS 宣稱當時已驗此表。修後 `008-006` 操作欄保完整 method 名，包含新 next_started_within，方法角色另由 code／解釋完成；此次本人桌面與手機實頁確認完整名稱可辨。

### 7. 未收到 entered 的失敗出口：同行提示後承認原漏查，修後機制與用途足夠

TECH tail N1 指出我原判漏查的必要失敗路線。ready 成功只是一回 HTTP 觀察；即使 URL 指向合法未用 gate，client 也可能在進 handler 前已 Request／Join 失敗。FixtureState 持有 entered Sender，所以 next_started 不因 client 完成而回 None；caller 可能永遠卡在通知，無法走到後面的 client await／Fixture.shutdown。

TECH 原有限 probe 在正常 ready 後中止其私有 server task，合法 held0 的 client 回 Request 錯誤、hits=0；有限 observer 取得 Elapsed，再收尾 server 得到 cancellation。這支持原缺口，不是我原獨立發現，也不是本輪我重新執行。本人與 FIRST 原正常 gate PASS 的有效範圍是「請求已到達後的受控流程」，不能用它概括未到達路線。

v15 `008-006` 在所有 caller 之前保留 raw next_started 的語義並明教新 `next_started_within(limit)`：mutable 借 Fixture，以 timeout 等原 recv，交 `Result<Option<usize>, Elapsed>`；通知／channel 關閉／觀察逾時三種結果不同。取消 recv 借用等待不丟 Fixture、不取走未收到訊息，也不取消 client task。原有 11.4／11.5 的 timeout／mpsc 取消安全契約支持，不需新的 select 或 Future 內部知識。

同段先教 `finish_client<T>(JoinHandle<T>) -> Result<T, JoinError>`，只封裝 owned abort＋await；T 是 client 原 Output，沒有新 trait bound。Tokio 1.53.3 `impl<T> JoinHandle<T>` 與 `impl<T> Future for JoinHandle<T>` 支持這份簽名；abort 後已完成 task 仍交原 Output，不能把已發生的 Request 錯誤統一換名 entered timeout。

需要／用途的鏈也完成：因通知本身不能察知 client 已失敗，caller 給觀察 5 秒上限；觀察不是 HTTP 工作的 10 秒期限，兩份預算各自在自己的起點建立。等待未確認時，先收全部 owned client handles，再 Fixture.shutdown，最後才報通知／client／server 結果。這讓未到達路線可走進收尾，但不承諾任意機器 5 秒內成功或整份程式硬在 5 秒內停止。

實作逐路核對：`008-008` main、`008-010` 雙 client 的第一／第二通知失敗、pause 前通知失敗、`008-012` 取消示範、`008-013` 完整答案，均在異常通知分支先 finish_client 所有仍 owned 的把手，再 shutdown，最後 panic。雙 client 不是只收失敗那一個。正常 release true 才直接 await；release false 也收 client 後 shutdown。正常雙 client 所有斷言、已觀察快照與 Result expect 都在兩份 client 與 Fixture 收尾後；雙 Fixture 的第二次 start 失敗先收第一份，正常亦兩份 shutdown 後才 assert。

`008-011` 新 absent_entered 是另一份材料：保留一份 held gate，但把 client 送 `/loop`，所以不會送 held 通知而有真 redirect Request 錯誤。先用受 15 秒限制的結果通知等待真 HTTP Result 算出，只有收到後才 pause 測 5 秒 observer；resume 後 finish_client、shutdown，最後檢查 Elapsed、原 Request/is_redirect 與總 hits=6。通知 task 在 current-thread send 後直接回 result、沒有後續 await，沿 11.5 的具體安排；不是「任何 send 成功都表示 task 已完成」。此 route 是作者新增 regression，**不是 TECH 原 server-abort／hits0 的重現**，不宣稱驗所有 server 故障。新 API、條件與 test count 7 正常 HTTP＋1 absent＋1 CLI，在 code 前或相應段都有角色說明。

### 8. 取消 client 與 server 收尾、stdout 時點沒有混淆

`008-012` 明說 client abort 不回滾 handler 已進入／hits 已加的事實，沒有取得 server handle。Fixture.shutdown 先關仍 owned 的 gate，讓 handler recv None→503，通知 graceful stop，另 await server；server graceful 不是替 caller 自動解除 gate，也沒有以 Drop 假裝等待 async 清理。

`008-013` 練習 spawn shutdown、分別等 client／closing 把手後才判 503／false／body／總數1；服務完成與 client body 完成不能互代。正常 main 的 stdout 在資源收尾後才印，v15 `008-008` 就地稱為「收尾後列出的步驟紀錄」，第一行記先前收到通知且尚未放行，並非印出當下仍未放行。

### 9. ROOT 後見 custom assert 修法：不回填 original

ROOT 在授權本輪時已提示兩處 custom-message assert 首用未教，後來在 main 與 dual source 改成已教 if＋panic。本輪實讀看到改後程式沒有引入新的 assert 介面；定向 CLI／dual PASS 是 ROOT 的版本證據，不是我的 probe。這項發現不是同行五份原判、也不是我的 original 或本輪未提示獨立發現。

## 主要通過項：機制與原需分開，沒有擴大證據

除了上述局部修正，81 BODY 的主要通過理由仍各自連回當時原問題：

| 位置 | 機制／性質 | 需要／用途與實際支持的界線 |
| --- | --- | --- |
| 11.1 `001-002` 至末 | 呼叫準備參數／保存工作、推進本體、取得 Output 三時點；普通 future 工廠可立即準備；await 不一定暫停。 | 把呼叫工作與何時推進／拿結果分開，為後續組合保留進度。單份 visits／receipt 證據不支持排程公平或 async 效能較快。完整答案沿已教 borrow、as_str、Output Result。 |
| 11.2 `002-002` 至末 | spawn 交工作不等 started、通知不等 complete；ID 是 opaque identity，map 接回 input；join 的工作 Result 與 JoinError 分層，abort 還要 await；丟把手可 detach，丟 JoinSet 的 abort 不等 await。 | 多工作等待時可進其他份仍須追每份輸入／成果／清理。四份 gate 固定結果次序，不以 sleep 或 runtime 公平推順序；沒有從 std thread 契約偷借 Tokio 特性。 |
| 11.3 `003-002` 至末 | pending 尚未建立 task，先補容量、收回再補；tracked 包含完成但未收，len 不等活躍數。 | 限制已提交未回收的工作，可讓 input0 尚等時新 input2 進入；不是整份資料記憶體上限或加速實測。limit1 是可教的特例，不能包辦所有排程。 |
| 11.4 `004-002` 至末 | 外 timeout 與內 Result 分開，合作 timer 不硬切普通阻塞；paused clock 不改 std Instant；整段／每步預算算例分開。 | 等待有期限並知道它限制哪份工作，不假承諾全程在 wall-clock 時刻停止。50／30／20／10 是控制材料；不是性能數字或所有 await 都暫停的證據。 |
| 11.5 `005-002` 至末 | select 在同 task 選分支，bias 不是 rollback；取消借用 join 等待不丟原 task；stop 已觀察才停止補入，abort 後 drain 可留下已完成結果。 | 停止來源失效、stop_requested、未提交與已提交不同；追蹤全部 input 並收清把手，真 SIGINT 不以觀察到 ready 當整份完成。recv／join_next 的取消安全有具體物件，不泛化所有 Future；signal 全域 handler 的限制有就地說明。 |
| 11.6 `006-002` 至末 | URL parse／scheme、send Response／text body、passed／工作 Err／CLI ExitCode 分層；blocking send 與 text 各有其等待範圍。 | 從檔案改網址仍保輸入／作業／輸出邊界，404 是有報告的未通過、Body 錯誤保存已收 status。預設 GET 沒要求讀完 body，非沒有 body byte 已抵達。離線 tests 只驗輸入／表示／CLI／writer，不驗公共 HTTP／redirect／timeout。 |
| 11.7 `007-002` 至末 | Runtime 推進、Client 保存設定／pool、inspect 保存待做工作；普通 send 建立計時，send/text 共請求預算；報告 owned、elapsed scope 明確。 | 先固定一份 HTTP 工作的期限與分類才組合多份；單請求沒有其他工作交錯，不承諾改 async 更快。模型練習要求 HTTP 通過再驗 JSON，模型成功也不代替 HTTP 成功。 |
| 11.8 `008-002` 至末 | listener 是連線資源 owner、URL只是位置；State共享同counter、guard在await前結束；gate先有entered再release；client與server各自把手與結果。 | 為固定200／404／redirect、已到達未收headers的期限／取消與收尾建立受控條件。ready只一次觀察、counter只handler進入數；新失敗出口按前節處置，body deadline 仍屬介面契約而未由本章headers-held測試驗證。 |

這些通過不是靠問答題名稱、method 清單或「有圖」成立。正文給需要、具體狀態改變與可觀察結果；圖與例子只支持其控制情境，沒有把通知／數值相同／單次測試當成一般 scheduling、網路可靠性、memory-order 或學習速度證明。原讀者未知有後文自然承接時保留當時未知，不回填成一開始就懂。

## 本人修後實頁：固定 bytes、兩裝置、實見範圍

自己的 preview 從 v15 的81 BODY依原順序重組，只移除後續unit重複H1、把固定絕對資產URI換成相對URI；9 SVG按原hash複製。theme／CSS沿本人原prefix preview，CSS只有 details font inherit，明設 extra_css；凍結 v15 本身不包含 CSS。沒有使用 live 第11章或 peer／ROOT 的頁面作自己的畫面。

真正 strict build exit0，0.53秒。最初 project venv 沒有 Playwright，exit1且沒有畫面，不當通過；改用環境已有的 system Python／Chromium，實際 capture exit0。54張選定畫面全由本人親看，desktop1280×800、mobile390×844；原40個target沒有整頁水平overflow，長code保留既有block內水平捲動，不能說每條長程式行在手機單張完全可見。

初次 table selector 抓到 URL parts（4張），pre 對齊抓到雙client block 開頭而非收尾行（2張）；這6張保留補充用途，另正確補拍且本人親看所需結果表／收尾line，沒有把錯誤target冒充所需位置。capture腳本、metrics、每張hash和本人view log可追。

實見支持：

- 11.5 manifest旗標、單`|`最低說明與程式相鄰；11.6窄表第三row全文換行仍分清finalResponse，flush責任與RefuseFlush解釋都在code前。status_line展開答案的as_str與固定文字可辨。
- 11.8 chosen serverFuture／io結果能力、ClientSendSync、5秒observer／10秒HTTP、cancel borrow／cancelclient、generic T保存Output，文字先於程式。型別的角括號正常render，沒有HTML吃掉必要能力。
- helper表桌面全表、手機context加element逐列，`next_started`與`next_started_within`都是完整識別字；async可另行。element截圖因工具捲位會讓sticky header覆蓋表頭，context表頭完整，兩份畫面共同支持，沒有只靠isolated element猜行列。
- 正常gate圖對到同一Fixture：①保listener／ready、②held0進handler/hits、③通知但仍沒headers、④release後client取得200與body、⑤另shutdown／awaitserver。圖前限定成功進入路線，新失敗出口已在之前教，圖後仍分通知／HTTP結果／server完成；不按框高推耗時。
- 另取原未變的11.7預算圖核主要PASS：build／parse在期限外、橘框圈send/text同10秒、elapsed開始／報告結束另有scope，箭頭對順序；兩張圖手機原位不需放大即可讀。未重看其餘7張未變SVG，不把此局部範圍說成9圖全重驗。
- 雙client失敗branch保兩份結果、aftercleanup以既有ifpanic判放行；新absent通知條件與測試時鐘限制寫在code前；client取消段仍另server收尾。CLI與closed-gate兩個fold、status_line答案共三fold在兩裝置展開，code font12.8px與普通範例一致，平常closed讓練習不先露答案。

本人本輪未看到新的必要視覺或圖文對應缺口；這是54張選定位址的實見，不是整章全畫面或使用者實測學習效果。

## 新可選 I-new：README 的泛型表示少結尾括號

發現來源是本人本輪全文讀 fixed v15 README，保存於讀 ROOT disposition 之前；不是 peer original、ROOT 提示或回填 original。

- 位置：`sources/examples/ch11-async/local-service/README.md` 第二段，原句 `以獨立5秒觀察預算取得Result<Option<usize>,Elapsed，與HTTP工作10秒不同`。
- 已教部分：正文 `008-006` 完整寫 `Result<Option<usize>, tokio::time::error::Elapsed>`，方法實作是 `Result<Option<usize>, Elapsed>`；三種結果與時間範圍皆清楚，程式沒有缺括號。
- 局部負擔：README的輔助型別名沒有閉合外層泛型，讀者對照方法簽名或抄名稱時需要自行補字。
- 最小修正／收益：改成 inline code `Result<Option<usize>, Elapsed>`，只補表示與辨認，不改API、正文、Rust行為或清理設計。列可選，不當必要先備或編譯錯誤，不需因這文字重跑Rust。

## 最後與 ROOT 處置比對及證據限制

在上述自己的交叉、81 BODY／76 archives與54實頁判斷保存後，才全文讀 `planning/reviews/ch11-root-disposition-v15.md`（25行、4,311bytes），SHA-256 `f3d024ff86a803f8e6a06a19e06d0df3c85abc2ff1564886f71940e60e562892`。

其採納修法與本人先存的判斷相容：flush分級各留理由、Vec只限7.9入口、FIRST未知不回填必要、ServerFuture只限選定型別、原private server-abort與新/loop分開、custom assert是ROOT後見。ROOT作者fmt／Clippy／tests與最後CLI／dual局部實跑都保其版本與出處；不是我新執行，未用它代替先備或機制推理。沒有因處置表列採用就刪掉原CONT對正常／未知路線的範圍。

本輪沒有Rust或真HTTP服務probe，沒有重跑未變全套，沒有看12peer／ROOT結果或擴到後章，沒有改student source、shared Cargo／nav／lock或任何原判。機制契約的支持範圍仍不包括：所有網路故障／任意負載、公平排程、真wall-clock硬截止、body階段timeout實測、任意writer的真外部故障。每份selected型別能力與owned結果只是本配置的必要契約，不是性能或memory-order安全證明。

本輪自己的詳細階段／指紋證據在 `/workspace/work/tutorial-implementation/ch11-cont-cross-v15/`：`peer-fingerprints.json`、`peer-cross-stage.json`、`cross-observations.md`、`fixed-scope-proof.json`、`body-source-read-stage.json`、`preview-input-proof.json`、`visual-view-log.json`、`pre-root-fixed-stage.json`、`root-last-stage.json`、preview capture及contracts。交叉同意與受提示後理解不補算original獨立發現；兩份CONT original SHA封存前後一致。
