# 第 11.8 節技術原判：local-service v13

本輪只補齊第 11 章尾段的獨立技術來源初判。必要待修 1 項（N1：首次進入通知的等待沒有失敗出口），可選呈現改善 1 項（I1：手機介面表格將識別字拆斷）。正常 gate／HTTP／收尾流程仍有下列有效支持，不因錯誤路線缺口而否定已教部分。

## 版本、角色與恢復範圍

- 唯一教材來源：`/workspace/work/tutorial-implementation/frozen/ch11-local-service-prerequisite-v13`，**1 頁、12/12 BODY、9/9 archive 檔案**。archive 是 local-service 全 package 的 8 檔（6 Rust、1 Cargo.toml、1 README）與 1 SVG。
- manifest SHA-256：`63db3a3021e46efd6307627d764ab33c84d2774c52eda836a240e162c1c372cc`；原頁 SHA：`3dfdcba1969e907a28f9fcd6381463054bffe1bcaf29ed50e200aa7a9f835b03`；SVG SHA：`b33b6a0ae1defbb7080fccc07086fcf5feee592c92fd95735ecaa5da9d89ccfa`。所有 BODY／archive／預覽 SVG 指紋與指定版本相符。
- 我是 `/root/chapter04_traits`，未寫第 11 章；有 4.1–4.3、14.1–14.3／14.6 作者與先前技術審閱背景。不是基本 Python 讀者的盲讀／新首讀。前文僅回到已讀 **prefix v11** 的必要正文與 `ch11-http-single` 原 source，不開 prefix 修稿／新 gate。
- 原 prefix 技術報告及其 initial 均保持 SHA `9d3d071f09f59016cb058983b816763d2da71782cd2d5b22b704384047a9b72d`，未改寫，也沒有讀其同行／root 裁定來補本輪。
- 執行環境中斷後，先確認了自己的 `review-ch11-technical-tail-v13-author04/` 工作目錄：原 manifest、正式 API 副本、preview、own probe workspace／lock 與 **僅 no-run 編譯完成**的 log 仍在，當時尚無尾段封存報告或執行結果。最初過窄的檔名搜尋漏掉目錄，我曾誤報沒有工作檔，隨後明確更正；沒有將這次漏查改稱舊檔不存在。
- 沿用可核實的原工作檔；恢復後完整重讀指定十二單位與九來源、驗 byte 邊界，再實際完成兩項 selected 執行及局部視覺。沒有舊首讀筆記可證明中斷前每段理解，因此不回填一份虛構首讀／舊執行紀錄。既有 reviewer fault-injection 準備也屬可見的自身背景。
- 使用 clear-tutorial／review-protocol／rust-terminology，維持基本 Python 入口、實際前文先備、先解釋最低契約再用、機制／用途分判、有限證據與錯誤收尾範圍。未讀 peer11／first11／root findings／disposition／作者 notes／checks／live／12 或 14 稿；未修改教材、root Cargo／lock／nav／checker／CI。

## N1 必要：等待 entered 時，尚未觀察 client 已結束的錯誤

**原位置與引文：**`001-006` 正確說「Fixture仍持有state中的通知送端」，不會因沒有更多請求而自動回 None；`001-007` 也明說 ready 只是一次服務觀察，「不承諾它之後永遠不會失敗」。但 `001-008` 的 main 先 `fixture.next_started().await`，之後才 `request.await`；`001-010` 的 gates／timeout、`001-011` 的 cancellation、`001-012` 的練習／答案也先單獨等 entered。原 source 是 `src/main.rs`、`tests/http.rs` 與 `examples/closed_gate_solution.rs`。`001-011` 同時將 binary 描述為「真正的有限示範」，再交給沒有外層停止條件的 `Command::output()`。

**已教／已成立部分：**gate 的雙方向與通知意義已教清楚；`next_started` 沒有資料時應等待，而非冒充完成，是正確 helper 契約。reqwest timeout 可讓 client 工作完成為 Request Err，沿原 prefix `007-003..005`；spawn 的 task 完成與通知 channel 關閉也不是同一件事，沿 `002-003` 的 Sender 存活規則。handler 真的進入後，現有 gate 次序、pause／resume、abort＋await 與 server shutdown 能成立。

**仍缺的關係／實際影響：**呼叫者在首次 entered 等待時沒有同時觀察已提交的 client 結果，也沒有給這項通知等待自己的失敗出口。合法、尚未使用的 URL／gate 與曾成功 ready，沒有保證這次請求一定進入 handler。如果它在 entered 前回 Request Err／task 失敗，state 中的 Sender 仍由 Fixture owner 持有，既沒有訊息也不關閉。client 的十秒 deadline 結束了 client 工作，卻不能替正在 `next_started` 的 main／test 解除等待；後面的錯誤分類及 shutdown 根本還走不到。這不是要 next_started 猜完成，也不是指正常 gate 本身永不結束。

**相稱的受控證據：**沿用中斷前已準備、已編譯的自有 isolated workspace。原 local-service library 全 bytes 保持為副本的原樣 prefix，只在最後附 reviewer-only `cfg(test)` 故障實驗：先正常取得 ready，再取消私有 server task，對合法、未使用的 `/held/0` 送請求。request 的等待有兩秒 guard，實際得到 `Ok(Err(AppError::Request(...)))`、hits=0；接著用二十毫秒 timeout 包 `fixture.next_started()`，得到 Elapsed，沒有 Some／None。最後仍明確 shutdown，保存 server 取消的 Err。私有欄位故障注入是審閱工具，不冒充學生可用 API，不聲稱真實 OS 必然發生這個故障，也沒有 host 執行無限等待。

這項執行與 source 中仍持有 Sender 的狀態共同支持上述因果；二十毫秒只讓 probe 有終點，不用它證明所有環境的延遲或某種失敗頻率。原先完整成功流程沒有因此被改判失敗。

**分級：**必要，有限示範／測試的錯誤等待與收尾契約缺口。阻斷點在第一次尚未取得進入通知，影響 client 先失敗時能否讀回原因並結束；不是要求通用 production server 或硬切任意阻塞程式。

**最小修法：**在 caller 首次等通知的階段保留另一個明確出口，並在斷言／回報失敗前完成仍持有的 client 與 Fixture 收尾。可用前文已教的有限 timeout 包通知等待，再取消並 await 尚未完成的 request、shutdown；或同時觀察通知與 client 結果。若選借用 JoinHandle 作 select 分支，需先教其實際借用／取消安全契約，不能只把 `&mut handle` 塞進答案。不要只加一段「可能失敗」警語，或讓 timeout 包含的把手被直接 drop 而失去等待責任。healthy gate 的通知仍是到達證據，不改成 sleep 猜已到達。

**來源／處置時點：**本輪未受同行／root 問題提示；自身中斷前已留有此故障 probe 準備，恢復後按原 v13 正文／source 重核並實際執行。只有這些可核實證據，沒有虛構中斷前的完整判斷。提出待修，未修改教材／未作修後複查。

## I1 可選：手機 helper 表格保留完整識別字

**原位置與已教內容：**`001-006` 的「先固定helper介面」表格，第一欄包含完整呼叫 `Fixture::start(n).await`、`fixture.next_started().await` 等，第二欄已足夠說清六項操作的 owner／borrow／結果與等待界線。正文及後續 source 可以對回這些 API。

**實看與收益：**在固定 preview 的 390×844 手機原位，表格兩欄都在頁寬內，沒有遺失內容；但第一欄將新名稱拆成 `Fixture::star`／`t(n).await`，以及 `fixture.next_`／`started().awa`／`it`。讀者首次對照 API 時需跨碎片重組名稱，與語法自然邊界不同。桌面較寬時也有 `.await` 尾端換到下一行的例子。這是實際呈現的可改善負擔，不是新方法未教。

**分級／最小調整：**可選；目前仍能讀出完整呼叫且由程式確認，不列主要理解阻礙。可以減少第一欄重複的 receiver 前綴，或調整該表格的分欄／換行位置，保留識別字整段，再由鄰文／code 給完整呼叫。不要以使另一欄極窄、全頁橫向溢出或新增一大段解說來交換。此項來自恢復後親看原位表格，未改 CSS 或教材。

## 逐單位技術通過與界線

以下依讀到該處時實際已有的材料分判。機制、用途與證據互不代答；N1 所在第一等待階段不包含在正常成功路線的通過承諾內。

| 單位／首次角色與操作 | 機制／性質的來源支持 | 原需要／用途的連結 | 例子與有限結果 |
| --- | --- | --- | --- |
| `001-001` 任務／入口 | 固定200／404／redirect，已到達的逾時／取消及退出前server收尾；前置指明client、task/channel、Arc/Mutex/atomic。 | 公共網站無固定答案，自有fixture把資料與等待條件放入測試控制範圍；沿7.8材料owner與11.5收尾，不歪曲公共網站可以做的手動操作。 | 本機服務用來驗這組工作，不承諾通用Web框架或網際網路環境性能。 |
| `001-002` axum／route／handler／IntoResponse | handler的Send角色在使用前給出；server Response與client reqwest Response分開；StatusCode constants、`(StatusCode, &'static str)` 的轉換與content-type先教，再給welcome/get/route。 | 要得到自己指定的HTTP答案；Router只是配置，還需要接收連線，沒有把保存函式當已處理請求。 | UTF-8固定文字與200足以教最小回應；imports／server/client在下一完整示範，明標介面片段。 |
| `001-003` TcpListener／loopback／dynamic port／SocketAddr／IntoFuture／graceful shutdown／features | bind的io Result、地址Copy/Display等能力、同listener ownership先教；Serve可await卻不是spawn所需Future，先教IntoFuture兩個associated types；停止通知與兩層await分開。 | 保留listener避免查port後重綁空窗；把正常server關閉等到完成，不能由停止通知推全部收尾。 | basic沿prefix v11實際build_client/inspect；真200/body與另server完成驗角色，沒有把bind當handler執行證明。若handler不結束，graceful可等待，文字已限定。 |
| `001-004` State／Arc共享／hits／固定路由／Redirect | extractor在handler之前取state；Arc clone同一份state；S的Clone/Send/Sync/static及Receiver／Sender能力先交代。hits是進入工作handler數，排除ready。307保留method，本節GET。 | 要分辨發出一份client檢查與真正進入幾個handler；redirect→ok需兩次處理，故計數+2而非一個輸入+1。 | 原state未複製成獨立counter；load是快照，不證工作全部完成；Route/State/Response三種角色不混。 |
| `001-005` Path／gate／`Router<S>` | `{input}`的0.8 capture語法與usize解碼先教；短block取owned Receiver後guard結束，再await；gate缺失404、關送端503。`Router<S>`缺狀態語義在router function前說清。 | 需要「真到達但尚未headers」；entered由handler送，gate保留未放行使Response不能交出，避免固定sleep猜到達。 | 命中路由而usize解析不合法先400、不進handler；本節不是已headers後緩body。不存在或用過gate仍可報404，沒有要求每個URL都發entered。 |
| `001-006` Fixture六項介面／Option.take／shutdown | owner保存位址、state、端點、server；各helper receiver與Output先列，take替換None、取owned值，再做邊界檢查。shutdown先關gates，再通知、await server。 | URL只是位置，Fixture持有可結束資源；close gate讓等待handler可回503，才有graceful完成；caller另等client。 | 成功完成後read hits才是最後值；Fixture消耗後不能再release。next_started中Fixture持有Sender的限定本身正確，caller首次等待仍有N1。沒有Drop假裝await。 |
| `001-007` start／ready／啟動失敗回診斷 | build_client在spawn前、bind/local_addr失敗在spawn前；n=0通知容量仍1；真正ready GET/body200完成才返owner。ready Err先保存，再shutdown。 | 要證已能處理HTTP而非猜等幾毫秒；同listener從bind移交serve不中斷，且current-thread client等待時server能前進。 | 一次ready不是永遠成功；若ready分支實際失敗，程式有顯式清理路線，但本輪未故障注入各ready/bind分支，不冒稱全失敗驗收。 |
| `001-008` 入場／放行／client結果／server結果／圖 | owned Client/Url交task，inspect內再shared借client；entered→release→request.await→shutdown次序可從已教規則推出。 | 需要分別看開始、HTTP完成及服務收尾；send accepted不代替任一整份結果。 | healthy路線有完整預期輸出與原 source；首次entered前client失敗時仍有N1。本輪未另執行該binary，因此輸出主要由source／其他selected材料核對。 |
| `001-009` status/body/finalURL／is_redirect／同時保留ports | 200／404／500有Response；redirect到ok共5 handler（3固定+2轉向）；loop上限5follow共6進入。is_redirect在code前教。兩份live owner才比較URL。 | HTTP未通過與拿不到最後回應分責任；不因307中途有回應就捏造最後成功報告；動態port與資源存在一起驗收。 | 型別／所有比對／count與source一致；沒有用猜測public結果。這三項原tests本輪未重跑，不把閱讀算獨立執行通過。 |
| `001-010` Client.clone／gate次序／pause-resume | clone共用pool先教，兩通知可任意次序；pause/resume條件、不是暫停真I/O／std Instant先教。收到entered後才pause，deadline仍原十秒，取得結果後resume再shutdown。 | 先確定已到達，才改測試時鐘以驗headers等待的deadline；放行順序控制完成先後，不虛構async效能比較。 | 原headers-deadline test在自有fixed lock實際PASS，得到Request timeout與最後count1；不驗已headers後body逾時，也不斷言真實十秒。gates test source能驗release0用過false／超範圍false；它未另實跑，N1仍在入場等待。 |
| `001-011` client取消／server另收尾／CLI test | abort+await client後，fixture仍持server與gate；shutdown不把client斷線當全服務完成。Command/output/UTF-8/完整stdout沿前文，沒有未知socket helper。 | caller沒有由client取消取得server ownership，hits已加1也不回復；需分別等兩方。 | source固定取消/最後count1的正常受控條件成立；CLI source保留0、空stderr與完整stdout。總7 HTTP＋1 CLI計數正確，但本輪不宣稱8項都獨立執行，有限入口仍有N1。 |
| `001-012` 未放行shutdown練習／全答案 | drop唯一owned Sender使gate recv None→503；spawn shutdown消耗Fixture，兩份把手各await，不再向已移交Fixture release。 | 需要區分graceful HTTP503與client Request Err／task取消；server完成不代讀完client body。 | 全example和對應test用已教API，文字/body/passed/count一致。正常閉gate答案可由當下材料寫出；入場前失敗亦受N1影響，非從後文補答案。 |

額外通過界線：

- `FixtureState.entered`容量等於gate總數（零gate時改1），每個gate Receiver只被remove一次，至多相同數量的held通知；shutdown前沒讀訊息也不會因buffer比gate數少而讓這些正常handler永遠卡在send。未提交的Receiver仍由state擁有，沒有把sender close誤稱所有map欄位立即消失。
- `release`先檢查index再take，沒有clone出第二個放行Sender；成功只證channel收訊，不證client body完成。shutdown的兩層Result與對stop notification的檢查明確分開。
- handler保存owned Receiver、shared state Arc，沒有Std MutexGuard跨await；與原prefix11.5已教契約一致。IntoResponse的library實作可用於介紹；不要求學生提前寫axum Body、HRTB或框架內部Service trait。
- axum0.8.9的graceful服務轉成Future後Output確是io::Result<()>；實作正常完成為Ok且不產生inner Err。正文僅保留signature的match分支與「本版正常關閉Ok」，沒有以它虛構某個已實測inner-error案例，本輪也未宣稱該分支故障覆蓋。

## 正式介面與執行證據

工作檔根目錄：`/workspace/work/tutorial-implementation/review-ch11-technical-tail-v13-author04/`。恢復前保存的 `official-source-manifest.json`、`official-web/manifest.json` 仍保留；恢復後驗其SHA並讀selected區段。不是讀作者來源筆記或據作者綜述說已tested。

| 固定正式來源／實讀位置 | 本輪契約核對 |
| --- | --- |
| [std1.99 IntoFuture](https://doc.rust-lang.org/1.99.0/std/future/trait.IntoFuture.html)：trait宣告、await desugaring、Future blanket implementation | await先轉IntoFuture；IntoFuture與Output是兩個associated type，Future亦可直接轉。 |
| axum0.8.9 `serve/mod.rs:97–156,263–302,332–350` | serve值、停止Future的Send/static bound、停止accept後等連線、IntoFuture→ServeFuture→io Result；沒有把Serve當Future提交。 |
| [axum0.8.9 Router](https://docs.rs/axum/0.8.9/axum/struct.Router.html) 的missing-state說明；`routing/mod.rs:408–415` | `Router<S>`是缺S，with_state轉到本例`Router<()>`；S的Clone/Send/Sync/static，不誤讀為裡面存的型別標籤。 |
| [axum0.8.9 State](https://docs.rs/axum/0.8.9/axum/extract/struct.State.html)；`extract/state.rs:301–318`；axum-core0.5.6 `extract/from_ref.rs`（恢復後另存extra SHA） | tuple欄位、從Router準備共享state；同型FromRef用Clone，Arc仍指同state。 |
| axum-core0.5.6 `response/into_response.rs:173–200,326–335`；axum0.8.9 `response/redirect.rs:41–49` | str轉UTF-8 text/plain、tuple覆寫status；307保原method/body。 |
| axum0.8.9 `extract/path/mod.rs:153–187,438–449` | Path tuple與DeserializeOwned；本例usize解析拒絕400而不進handler。 |
| Tokio1.53.3 `net/tcp/listener.rs:65–74,102–109,332–334` | bind0請OS選port，綁好可accept；local_addr回io Result SocketAddr。 |
| Tokio1.53.3 `time/clock.rs:116–180` | pause/resume前提、auto-advance、真I/O不被暫停；吻合先entered再pause的材料範圍。 |
| reqwest0.13.5 `error.rs:101–104`、原prefix保存的`async_impl/client.rs:79–95` | is_redirect對RedirectPolicy；Client clone內部Arc/pool而非新pool。 |
| prefix v11的`007-003..005`正文與原http-single整份source；`002-003,002-005,004-003,005-002` | 實際build_client／inspect borrowed/owned介面、request總預算、Request Err／Sender存活／abort等待／有限timeout與選擇等待，支援本輪新組合與N1。 |

自有 probe workspace 根只含 `local-service` 與原 prefix `http-single` 副本，與主workspace隔離。所有兩個package的Rust與受保護版本相同，僅local-service lib後面多 reviewer-only test；保留原lib全bytes prefix SHA `d92378da8834f64057240327ef888d2d368cdaa6e68982edda333910be638550`。自己的 manifests只精確pin原直接依賴；自己的 Cargo.lock SHA **`fb41dadae7dbd1511a73ff66091980127fdf401eae6f05abda96d09b0fbf6757`**，selected axum0.8.9／axum-core0.5.6／Tokio1.53.3／reqwest0.13.5。不是拿未封存的live根lock冒充v13。

保留舊 `probe-build.log` 的 no-run成功，**不把它算tests已跑**。恢復後真正執行：

- reviewer的entered前Request Err fault probe：1 test PASS，exit0；request正常結束為Request Err，hits0，bounded entered等待為Elapsed，顯式cleanup保server取消原因。
- 原 `a_request_reaches_its_deadline_while_headers_are_held`：exact選1 test PASS，6原HTTP tests被filter，exit0。

結果與完整命令分別在 `probe-early-request-failure.log`、`probe-original-header-deadline.log`、`selected-execution-results.json`。內部等待有明確期限，外層每項25秒watchdog；兩項均立即正常結束，沒有觸發watchdog。未重跑原prefix或allWIPworkspace、未請求公共HTTP、未host執行無限pending／壞協定／deadlock；其餘本節tests／binary／各啟動故障分支未獨立執行，不計runtime通過。

## 圖、表格與折疊的局部實看

沿用自己的既有 **exact v13 preview**，恢復後再次比對其正文是十二frozen chunks合併、僅去同頁重複H1與替換SVG渲染URL；沒有其他正文改動。只含11.8的nav，SVG bytes與archive相同。CSS與prefix既定preview相同，SHA `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`；未因審查改shared樣式。記錄在 `preview-transformation.json`。

恢復後用Chromium實際親看 **desktop1280×900／mobile390×844，1×CSS pixels** 的六張原位PNG：SVG各一張、helper表格各一張、打開完整答案的開頭各一張。`visual/`、`visual-capture.json`保存位置與鄰文，不是單獨放大SVG。

必要圖對應：同一Fixture從bind保listener與ready成功→送GET held0→server進入、hits+1→entered通知但仍無headers→release後HTTP200/body→另shutdown關gate/通知/awaitserver→最後hits1。三個來源訊息／client結果／server結果各有範圍，正文圖後直接指出，向下箭頭只為這份正常示範的等待先後而非耗時。兩種尺寸均能追對象和箭頭，無需點放大；小字箭頭限定另有相鄰正文支持。圖僅表示正常路線，不以它放行N1。

表格完整操作與結果仍可見，但有I1的識別字拆斷；scroll後的「回到頂部」浮動按鈕在手機表格底部有一次遮蓋，現有截圖不是全部scroll路徑證據，未把可移動的單次位置升級為持久遮擋缺陷。答案能打開、正文/程式碼保持正常大小；手機長code在自身容器內橫向滾動，截圖只看答案開頭，沒有聲稱一張圖已看完1142px高的全部展開內容。

本輪未驗全部details、其餘表格位置、暗色模式、copy按鈕、全頁滾動或實際GitHub Pages。局部圖表折疊檢查不稱全頁／全站視覺通過。

## 原判封存狀態

12 BODY／9 archive已完成，selected官方API、兩個有限執行與六張原位圖表/折疊實看有各自材料。N1待修；I1可選；上述主要方法保留有限通過。保存本報告、同bytes initial副本與SHA後停止，未cross／未讀任何新修稿。後續提示、裁定或修後核對另存，不覆蓋本次原判或原prefix初判。
