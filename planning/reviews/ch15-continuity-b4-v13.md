# 第15章 B4 ORIGINAL 銜接初判：15.10，v13

本輪結論：**必要問題 N0、可選建議 I0**。指定 B4 的停止時點、每筆分類、批次理由、有限觀察與 owner 收妥已有足夠銜接；這不是整個 B 專案、15.11、執行結果或整站驗收。沒有修改學生來源。既有 A 與 B-prefix 原判、分級及未知不因本輪結果改寫。

## 角色、版本與隔離界線

- 審閱者 author04，未參與第15章寫作；曾寫第4章部分與第14章 macro／工具，亦曾做第11章技術審閱。本輪沿自己已封存的 A／B-prefix 完整閱讀背景，是 ORIGINAL CONTINUITY 延伸，**不是無背景 FIRST**，也不是作者自查。
- 唯一新稿：`/workspace/work/tutorial-implementation/frozen/ch15-b4-prerequisite-v13`。一頁 **9 BODY、13 archives**，涵蓋 B4 starter／complete 的 library、全部 checks、demo、manifest、README、Input／single 與一張 SVG。
- manifest SHA：`5f0dcdb5346281a01f5ce2a5a252bef83d841ebd8bc305be8ba5499a3f99a850`。完整學生來源指紋清單 SHA：`53ba779f60aad6c2ae17499b82c54657e7dd5ad5110c9557718419b4d3f2b28f`；清單依 manifest 的 BODY 與 source 次序列出每份 SHA，保存於自有工作目錄。
- 13 archives 為 **11 組不同完整 bytes**。新 library／checks／demo／兩份 manifest／兩份 README／SVG 均實讀；兩組 Input／single 先核對與已 FULL 讀過的固定 B-prefix 完整 bytes 相同，才對映。complete library 前277行實讀，其後完整 tests suffix 與本輪已 FULL 讀的470行 checks 完全同 bytes，不以相似片段代替。
- 回讀固定 `ch11-review-fixed-v16/005-001.md` 至 `005-012.md`、固定 B-prefix `003-003.md` 至 `003-005.md`；其餘必要 B1–B3 契約沿自己上一輪已 FULL 讀的固定稿。另局部回讀實際學生正文7.9的輸出／flush／退出邊界，沒有把目錄、作者摘要或自己的專業知識算作已教。
- 未讀任何15同行報告、作者 notes／ledger／handoff、Root outcome、15.11 或 final packages。沒有 cross。沒有執行 Rust、真 Ctrl+C、公開 HTTP、作者 tests 或全 workspace checks；程式預期結果只按本輪來源判讀。
- A 原報告 SHA 保持 `4a99ca927223e9bdc5e46b92ccf13cc03cb489f1f1615fb83bbd880e690b52ff`；B-prefix 保持 `7fa44c40873c102f14b044a0cc75c3fef7bb1cc9721a4bf0281079a842c82cf2`。原 B-prefix 的必要／可選項未重新裁定，也未假定已修。

## 從實際正文重建依賴與主要通過證據

下列每項分開核需要、機制、操作與證據限度。「已足夠」只指當前閱讀與指定材料，不代表我已執行相應行為。

| 本輪位置與原需 | 已教前提、機制與最低操作 | 用途、例子與判定邊界 |
| --- | --- | --- |
| `001-001`：「每份原輸入留下什麼：已完成、已取消，或根本沒有提交？」`001-002` 五份材料 | B2 已教略白行前的0-based索引與保留 original；B3 已教完整 Input 登記、jobs 名額與收取後補入。此處直接區分 gate0／1控制原index1／4，第二 held 進入證明3已收且4已補，沒有以 sleep 猜進度。 | 原需先於停止設計。index3保留HTTP200，1／4取消，5／6未提交；`/notes` 尚未交給 single，不能先分類為 Input 問題。重複 U 仍是兩份輸入。此材料確實使已收成果、已提交等待與 pending 同時可見，**已足夠**。 |
| `001-003`：「把整批理由與每筆分類分開保存」 | 11.5 `005-003` 已教 StopEvent、EndReason、訊號來源失敗及關閉；B3 `003-003` 已教 Check 與 JoinError 分層。正文在 enum code 前說 Cancelled 由 is_cancelled，Unstarted 由 pending，兩者沒有 Check／HTTP／elapsed。`original()` 只是沿既教 borrowed match 讀四個 variant。 | EndReason 解釋整批為何結束，Completion 解釋每份原行；Finished 不是每筆通過，Closed 不是 Requested。停止不是 transport failure 或 Timeout。四分類與 Batch 各自回答不同原問題，**已足夠**。 |
| `001-004`：「new只準備資料；收取只record；每輪先查停止，再決定fill」 | 與實際 B3 new 立即 fill／next 收後立即 fill 對照；11.5 `005-005`、`005-007` 已教先準備、try_recv 的 Empty／Disconnected、biased select 與不在 join handler 補工。此處 constructor 仍先拒絕 ZeroJobs，再 Empty，沒有 spawn。 | 提前排入事件可阻止第一批；收取後重新檢查可防先補後停。這不是虛構 B3 做不到停止，而是改變兩個提交時點以符合新增需求。限定「從觀察到停止起算」避免把 policy 誤讀成 OS 訊號瞬間的原子保證，**已足夠**。 |
| `001-005`：record、stop_and_finish、by_ref、into_batch | 11.5 `005-004`／`005-006` 已教 classify 不等待、Id 對應、abort_all 不等待也不移除、逐份 join 及完成競爭。B3 已保存完整 Input，故本次失敗仍能取得 original。**新 `Iterator::by_ref()` 在首次實作用途前**明說 mutable 借同一 iterator、取得 owned Input、保留欄位 ownership、原 pending 一起耗盡，並區分不是 iterator 複本。 | record 移除一次登記、保存實際 Check／TaskFailure／Cancelled，不補工；收齊 tasks 後才將無 Id 的 pending 保存 Unstarted；最後接手 owner 並按既教 sort_by_key 排序。既有索引空洞與原文保持。所需新能力有最低解釋，未要求讀標準 library 的完整泛型實作，**已足夠**。 |
| `001-006`、`001-008`：保存完整起點、循序練習 | 明確要保存 manifest、整個 src 與 checks，而非只複製 snippet。sync／signal 在普通 dependency 的理由接11.5；Fixture path 先換成實際絕對路徑才跑 Cargo，toolchain 沿15.1。三個 stub 及 allow(dead_code) 的各自範圍在 code 前交代。 | 零 test 只表示起點可編譯；追加 checks 後1通過6失敗，先 record＋stop 應2／5，再 run 應7／0。checks／答案順序支援自己邊看邊寫，三層提示與完成快照預設關閉；主要任務規則沒有只藏答案。這些數字是**來源所列預期，我未重跑**；銜接已足夠。 |
| `001-007`：「測試自己的清理不會把產品忘記等待的錯碼補成通過」 | BODY 先說 finish 保存 tasks.len／inputs.len 快照，再後備 drain、Fixture.shutdown，最後才 expect／assert。checks 23–43逐行一致。drain 僅 abort_all＋join，不呼叫會 fill 的方法；make_scheduler 設定失敗先收 server。 | 若產品漏等，即使測試最後替它收回 children，原快照仍非零而不能補成綠燈。`Result<EndReason, Elapsed>` 區分測試期限與批次理由，**已足夠**。備援清理不替錯碼補登記／造報告，也不宣稱任意 panic 都可救回。 |
| `001-007` controlled_stop、合成材料；`001-008` 實際檢查 | 本文在 helper 前說同一 task 的兩份借用等待、controlled branch 記 observed／送 Requested 後 pending，run 返回才結束 select；十五秒 timeout 丟借用 future，但 Scheduler、entries、Fixture owner 仍留在 helper。11.5 `005-002` 已教 select 同 task／取消等待非移除集合，11.4 pending／timeout 已在實際 B-prefix 回讀。 | two_entered 每份五秒，結果 guard 十五秒；失敗先收妥再斷言。合成 task 完成通知之後沒有另一個 await，current-thread owner 重執行時 body 已返回，故可安排完成尚未收取的 Check／panic。不是任意 send 都證明全域完成，也不是 HTTP 證據。同步 panic、不讓出 runtime 仍排除，**已足夠**。 |
| `001-009`：用真 Ctrl+C 替換受控停止來源 | 11.5 `005-010` 已完整教 ctrl_c 的 io::Result、signal feature、首次 poll 註冊、biased 先 poll 再 entered、pending ready 不選中、Unix handler 不隨 drop 復原。`finish_signal_controller` abort 後 await，bool 只是事件送入成功，不是批次理由。新稿重接這些具體契約再展示 code。 | demo 刻意改四份材料，以兩份 held 保持 READY 時可取消；gate0／1改對index1／3的原因明說，保留成果的路線已由前面五份材料說明。不是悄悄改編號或省去原承諾，**已足夠**。 |
| `001-009` ready／close／print_batch／退出與 owner 界線 | ready 先核對 gate pair，再 writeln＋flush；listener entered 有獨立五秒觀察。close 先 await Scheduler 收尾，再分別保存 listener／server 收尾，兩者皆 await 後才 `?`。observed 的 Err 保留診斷，None 不冒 READY。7.9實際正文已教 Write／flush 與 ExitCode 邊界，11.5 `005-011` 已教數字退出介面。 | 源碼與文字一致：收妥後才印報告；READY 後立即操作應 Requested／1、3 Cancelled／4、5 Unstarted／hits2／130，port與Debug可變；不按則每筆期限讓有限 demo 完成且因 `/notes` 回1，基礎或輸出錯回2。**這是條件式預期，未由我真 signal 操作證實**。一般 run_batch 自己擁有 Scheduler，drop 不等 children；與測試借用 run 後仍持 owner 可另收尾分清，已足夠。 |

主要方法的最短用途推論：讀者原要完整交代每份原行；直接丟棄擁有 Scheduler 的整批入口只會提出 abort，不能交回清單，也未等 children 收妥。因此將已收成果放進 Scheduler、查停止移到每次 fill 前、收回已提交的實際結果、另遍歷無 task 的 pending，才同時保住資料身分與完成責任。這與取消一份借用等待、仍保有 owner 不同。此推論只用正文與已教11.5／B3，沒有由新 API 名稱倒推一個作者未給的用途。

## 完整 source／checks 覆蓋與有限證據

starter 是三個未完成 body；complete 對應提供 record／stop_and_finish／run，另有提供好的 Input、single、signal helper、constructor、fill、into_batch。完整兩份 README 與 manifest 亦已核對，不將 README 中15.11預告當成本輪已教的 CLI／JSON 或完整退出政策。

七項 checks 全部讀過，範圍是：設定錯誤優先；提前停止零提交；兩份真 held 開始後取消且保原文／duplicates；收取後補工再停止保留已完成HTTP；合成完成未收的 Check／panic 與其餘 Cancelled／Unstarted；來源 SignalFailed／Closed 分開；正常混合結果與原順序。HTTP／Input／Request 三類結果的區別沿 B1；沒有用合成 Check 當網路成功。普通混合資料預期 hits9，來自兩次ok、一次missing及loop六次 GET，invalid未送HTTP；這是已教有限材料上的來源推論，不是我本輪實測。

新增 helpers 的操作均可接已教範圍：assert_cancelled／assert_unstarted 是 borrowed enum match，assert_pair 是 slice 索引與布林或；兩份合成通知用容量2 sender clone，held 收端在取消後清理；普通完成案例 `_stop_tx` binding 保有送端，沒有誤 drop 為 Closed。未發現本輪再次加入未教 match guard；這不等於撤銷上一輪 B3 原問題。

只為本輪確切契約查了正式 registry **Tokio1.53.3** 固定 source：ctrl_c 初次 poll／Unix caveat、join_next_with_id cancel safety、abort_all 不移除／不等待、try_recv 的兩個錯誤。完整 source SHA 與實讀範圍見 `selected-official-proof.json`。這是本機正式 source cache 的查核，沒有冒稱 fresh 網路閱讀或執行。std by_ref 的本節用途已由正文及實際 ownership 流程足夠解釋，本輪未另下載 std 文件。

## 原位 SVG、桌面／手機與折疊

自有隔離 preview 僅組指定9 BODY，按原序去掉拆單位重複頁名，保留段落／code／fold。固定 SVG 原 bytes 轉成 preview 相對 URL；少數前文連結只在自有 preview 改成對應公開頁 URL，沒有請求這些外部連結或改學生來源。theme、Markdown extensions、CSS 與當前實際站點設定相同；`navigation.top` 未啟用。strict build 完成且無問題，不把 build 當視覺已驗。

之後用 Chromium 實際開 preview，**親看20張 PNG**：1280×900／390×844 各10張，位置為圖上／圖下、材料表、by_ref 介紹、starter入口、owner／快照清理、關閉的提示／答案、開啟答案、signal介紹、命令與最後結果。SVG載入為360×760；CSS經HTTP200實際取得並核同SHA；全部擷取點沒有整頁水平溢出，Top數為0。source 與實際 view 的對應如下：

- 藍框將「已提交1／4、已保存3、pending5／6」保留在同一初始狀態；橘框明記已觀察 Requested／不再 fill。向下箭頭是順序，不是新 task 或原文的資料 move。
- 第二藍框區分 abort_all 與逐份 join，保留 Checked／TaskFailure、實際取消才 Cancelled；灰框標 pending 沒有 Id。綠框按原索引列1／3／4／5／6，另列整批 EndReason。下方文字明說3的HTTP200沒有被覆寫。必要標記不用放大可讀，上下視圖補齊圖說與緊接著的提交時點轉折。
- 材料表在手機多行換行，仍能逐格追蹤 index、held gate與狀態，沒有切斷必要識別字；表下0／1→1／4文字可接。未新增可選呈現項。
- 三層提示、完整答案預設收合；開答案可見對應完成碼。機制與練習操作在開 fold 前正文已有，沒有靠展開答案才能知道規則。
- 手機長 code 在自己的 code 容器水平捲動，沒有擴大整頁；這次畫面不是宣稱每一長行都已在初始 viewport 顯示。全文 code 的閱讀證據另由完整 source 核讀提供。

這是上述受影響位置的 actual 兩裝置檢查，不是所有捲動點、深色主題、跳轉連結、網站全頁或最終 CLI 畫面的驗收。初次 DOM 工具呼叫因該 Python 環境缺 playwright 失敗；後改用離線 cached playwright 的獨立 uv 執行環境成功，沒有改專案依賴。此失敗不算教材問題，也不算一次成功視覺檢查。

## 問題、未知與封存

本輪 **N0／I0**，沒有需要修改學生稿的新增項。可預期的回讀是正常沿11.5與B3複習，沒有發現須靠後文補當下新能力的斷點。正文、七項 checks、完整答案與 demo 的讀者時序核對已完成；執行、OS訊號、真測試預期值仍未由此角色驗證，不擴稱 PASS。

15.11 真 CLI／穩定文字與JSON／最終退出政策／final packages **未讀、未驗**；它們在本輪僅是下一步預告，本輪 typed Batch 與停止材料不依賴其實作。沒有以預告替當下缺口辯護，也沒有承諾下一節已成立。

自有證據目錄：`/workspace/work/tutorial-implementation/review-ch15-continuity-b4-v13-author04/`。關鍵檔案是 `scope.json`、`early-notes.md`、`student-files.sha256-list`、`byte-alias-proof.json`、`source-groups.json`、`source-full-read-coverage.json`、`prerequisite-proof.json`、`selected-official-proof.json`、`preview-source-proof.json`、`preview-dom.json`、`visual-capture-proof.json`、`capture.py`／`capture.log`、`preview/build.log` 與20張 `views/*.png`。原判另存同 byte `.initial.md` 與 SHA sidecar，封存後停止，等待15.11新 gate。
