# 第 15.10 節 ORIGINAL TECH：B4 v13

本輪沒有確認的必要修改或可選修改項。這個判斷只涵蓋 15.10 的 B4 凍結稿、其中完整 starter／complete 與有限操作，不代表 B 完整版、第 15 章或發布驗收通過。

## 身分、隔離與版本

- 審閱者是既有 TECH15；曾寫第 1、8 章、11.4、14.4／14.5／14.7／14.8，沒有撰寫第 15 章。這是非作者的技術審查，有 Rust 與本專案背景，不是假設只懂 Python 的盲讀首讀。
- 唯一新教材輸入：`/workspace/work/tutorial-implementation/frozen/ch15-b4-prerequisite-v13`。一頁、9 BODY、13 archive paths，11 個完整 byte classes；連 manifest、reading-order、原位圖副本，共 25 個實體檔案。
- page SHA：`cb0fc06a0a9abefc9f54ad9a387600db141a543840ee8e6bda0ce0ff1a48d056`。SVG SHA：`95ab1e126e46778b36a3f53ec4462e957af55547f4ffe5c7e070a5899ed290dc`。
- 先保存初始指紋，逐項實讀全部 BODY、unique library／helper／test／demo／manifest／README／SVG，再做必要先備與有限驗證。所有 25 檔在收尾仍與初始相同。沒有修改學生稿、共享設定、staging 或原判。
- 沒有讀作者 notes／ledger／handoff、Root gate／outcome／disposition、CONT／FIRST15、15.11 或最終 B package。沒有交叉覆核同行答案。
- A 原判 `042f134ed1b349a78ea5e2717b0b15c26d9197368b3c2b60089a0bbf614c3189`、B-prefix 原判 `467fe5f9207a38c6a4373e36aa82cd3cafc33a3c3089e340228f2ddb223cbd69` 本輪僅保護 SHA，原 bytes 與分級未動。
- 第一次讀取遇 transport 失敗不算讀完或寫入成功；真實 exec 恢復後，才保存初始指紋並接續。本輪沒有重跑先前 B1–3 probes。

工作證據在 `/workspace/work/ch15-technical-b4-v13/`；中途未知先存 `read-checkpoint-before-closure.md`，後續核對另存，沒有回寫成當時已知。

## 全文與 source 覆蓋

9 個 BODY 都完整讀過，包括主文、三層提示、完成答案與最後 demo。完整檔案不是用 snippet 或標題代替：

| 完整材料 | 實讀／同 byte 證據 |
| --- | --- |
| starter `src/lib.rs`，5,765 bytes | 整檔讀，包括完整公開介面、提供的 fill／signal helpers 與三個 stub |
| complete `src/lib.rs`，26,402 bytes | 整檔讀，包括排程、兩個 signal helpers、完整七個 tests 及其所有 helpers |
| starter `checks/stop-tests.rs`，18,687 bytes | 自行驗證為已全文讀過 complete library 的 exact suffix；不是只查 test 名稱 |
| complete `examples/b4_signal_demo.rs`，5,486 bytes | 整檔讀，逐條核 ready／close／run_demo／print_batch／main |
| 兩個 Cargo.toml、兩個 README | 四檔完整讀；普通 dependency 的 sync／signal、Fixture path、repo 外 toolchain 操作及階段限制都有核 |
| 兩份 input.rs、兩份 single.rs | 自行核與前輪已全文實讀的 B2／B3 frozen source 完全相同；input 的兩個新 copies、single 的兩個新 copies 分別同 byte |
| SVG 原始碼及實頁 | 完整 XML、intrinsic 360×760、title／desc／箭頭與每個文字標記均讀，另看兩尺寸實頁 |

`source-byte-map.json` 記 13 paths→11 classes 與閱讀依據；`copy-proof.json` 記 checks 的完整 suffix proof、input／single 的所有相同前文 copies。沒有以抽樣冒稱全 source。自己 probe workspace 複製的 local-service／http-single 支援包與前輪自己已讀的支援材料再核相同 bytes；不是讀新的未授權實作。

## 實際先備與首次需要的位置

先備採真實 BODY，不採章名或 Root gate 為已教證據。前輪自行實讀的 31 個必要前文單位，本輪驗 SHA 相同後沿用，包含 spawn／JoinSet／Id、channel／task ownership、Duration／timeout／pending、Client／GET／headers／redirect、Fixture 的 held gates 與 shutdown、B1–3 的 owned Input 和有限排程。

本輪另外實讀 actual `ch11-review-fixed-v16` 的 005-001～007、009、010；補核 7.5 的 stdout／ExitCode、7.9 的 main／flush、6.7 排序、8.3 method lookup，以及已讀 A1 的 eprintln 契約。逐檔 SHA 與範圍在 `prerequisite-fingerprints.json`。

| 此處需要的能力 | 真正已教位置與本節時序 | 判定與界線 |
| --- | --- | --- |
| select 同 task、多 future、biased 順序與取消安全 | 11.5／005-002 明教，再由 15.10／001-004～005 連到查停止、record 不補工 | 已足夠；不是為每個 branch spawn task，也不是全時原子阻止提交 |
| abort_all 不等於清理、join 保留真正 Output／JoinError | 11.5／005-004、006；本節 001-003、005 先教 Checked／TaskFailure／Cancelled／Unstarted | 已足夠；來源未提交時沒有 Id，不虛構 HTTP／elapsed |
| try_recv 的 Empty／Disconnected、buffer 先取事件 | 11.5／005-007 明教；本節 001-005 說操作，再於 001-008 提示加入 import 與實作 | 已足夠；來源關閉不等於 Requested |
| `Iterator::by_ref()` | 001-005 在 tests／完整答案使用前，說 mutable 借原 iterator、取得 owned Input、保留欄位 ownership、原 iterator 一同耗盡 | 已足夠；沒有把它說成複製清單，也不用先讀 library 內部實作才懂 |
| 排序與 method 查找 | 6.7／007-006 明教 sort_by_key 的 mutable slice／FnMut 契約；8.3／003-004 明教 Vec→slice 及沿 Deref 尋找 method | 已足夠；本節排序不重編原索引、不合併 duplicate |
| signal feature、ctrl_c 首次 poll、Unix SIGINT 行為 | 11.5／005-010 完整教；本節 001-006 manifest 及 001-009 helper 前重接 | 已足夠；建立 future 不是註冊證明，drop listener 不恢復預設行為 |
| io stdout、writeln、flush、ExitCode | 7.5／005-006、7.9／009-006；001-009 先說 ready／close／print 角色再給整份 demo | 已足夠；flush 不是永久儲存保證；正常回 main，不用 process::exit 略過清理 |
| eprintln | 順讀前文 A1／002-007 已明教相同格式介面、stderr、寫入失敗 panic | 順序主文已教；本輪沒有另驗完全略過 A 的 B-only 跳讀路線 |
| helper 與 caller 的取消責任 | 001-004 說一次 run／唯一遞增索引；001-005、007 先教 owner／借用等待；001-009 明說不要 drop 整份 run_batch 當 join | 已足夠；這是具體 API 使用契約，不宣稱任何 future 都能後備清理 |

固定原始 API 也獨立核過：Tokio 1.53.3 `ctrl_c.rs` 完整 listener 說明與函式、`select.rs` 的生命週期／同 task／biased／cancellation safety、JoinSet 的 join_next_with_id 與 abort_all、mpsc 的 try_recv；Std 1.99.0 的 Iterator::by_ref 選定文件。`selected-api-proof.json` 記路徑、URL、SHA 與實讀範圍。下載 Iterator 全 HTML 只當保存來源，實際閱讀是 by_ref 區段，不冒稱全篇已讀。

## 主要通過項：需要、機制、用途與例子分開

### P1：完整原行清單沒有被停止事件抹掉

原需來自 001-001「每份原輸入留下什麼：已完成、已取消，或根本沒有提交？」；不是由新 enum 的名稱反推需求。

- 身分／操作：001-002 五筆原 index 1／3／4／5／6，明說 gate0／1 控制原行1／4；U 的兩次出現是不同輸入。001-003 用 Entry、Completion、Batch 保存逐筆與整批兩層結果。
- 機制／性質：record 依 Id 取回完整 Input 一次，真正 Check 保留；取消／task 故障由原 Input 補回 original；pending 是未提交 Input，沒有可取消 task。
- 需要／用途：保留空白行造成的索引空洞、duplicates 與未解析 `/notes`，讀者可逐行對回原檔，知道尚未檢查而非誤判 URL 不合法或 timeout。
- 材料支持：手工五列、SVG 及 refilled-stop 測試使用同一份關係，3 已完成後補入4，停止後仍保3 HTTP200，1／4 Cancelled，5／6 Unstarted。本人有限實跑該 test PASS；不從它推出遠端 GET 副作用已回滾。

### P2：查停止的時點能回應 B3 立即補工的問題

001-004 明說「new只準備資料；收取只record；每輪先查停止，再決定fill」，原問題是 B3 new／next 已先 fill 才能看見停止。

- 機制／性質：new 無 spawn、record 無 refill；try_recv 先查，Empty 才 fill，biased select 先列停止，收取後回圈頂端再查。
- 需要／用途：已排入的事件可阻止第一批與下一次補工，不必先建立工作再取消，仍保有原來 jobs 上限。
- 操作與範圍：停止保障從觀察起算，本文明排除訊號抵達瞬間的全程原子禁止；Finished 只表示工作圈完成，不是所有項目成功。
- 例子：提前 Requested 案例用非空有效清單與 duplicates／invalid 材料，驗零 GET、全部 Unstarted。本人的 complete filtered test PASS；starter 同測試確實敗在 Finished／Requested 差異，不是 compile error 或碰巧零工作造成假綠燈。

### P3：取消要求、結果保存與等待清理各自有工作

001-005「不能把集合內每份都覆寫為Cancelled」與 001-007 的 synthetic 材料，讓 abort 與實際結果競爭可見。

- 機制／性質：abort_all 不移除集合；逐份 await、record 到 None，才清 pending、形成完整 Batch。
- 需要／用途：停止時完成但未收取的 Check／task panic 仍有報告，不被改寫為未完成；已取消工作的放行收端也確實釋放。
- 材料支持：current-thread task 在完成通知 send 後不再 await，才直接 return／panic；通知 owner 因而能取得「body 已完成但尚未 join」的受控狀態。第三份待 channel、第四份 pending，四種分類各有材料。本人 filtered test PASS。
- 界線：synthetic Check::Timeout 是受控值，不當 HTTP timeout 證據；panic hook 不等於整個 test 失敗。不推論不讓出 runtime 的無限同步計算也能由 guard 切斷。

### P4：測試的後備清理不代替產品通過

001-007 在 code 前說明 drain／finish 的責任，先保存 tasks／inputs 長度，再後備 abort／join、shutdown，之後才 assert 原快照與結果。

- 機制／性質：Scheduler／Fixture owner 留在 helper 外層，timeout 與 select 丟棄的是借用等待；成果放在 Scheduler 欄位，不靠被取消 future 的區域 Vec。
- 需要／用途：故障或有限觀察到期時仍能回收資源，又不因測試替產品清理而把漏 join／漏登記移除的實作變成綠燈。
- 材料支持：完整七項 tests 與每個 helper 均讀；make_scheduler、two_entered、controlled_stop 的 Err／timeout 路徑先收尾後報錯。有限執行所選四項成功、starter 一項真失敗，與此責任切分相稱。
- caller 界線：run_batch 本體擁有 Scheduler，整份 future 被 drop 仍只觸發 JoinSet abort；普通 caller 要送停止並 await。測試特意直接持有 Scheduler，不能將測試 owner 安排概括成所有入口都自動等待完成。

### P5：真正 Ctrl+C 有註冊證據，也有 listener owner 的結束

001-009 先連回 11.5 的 ctrl_c 首次執行，再給 helper／demo。這不是以 job 啟動、建構 future 或 sleep 當 listener ready。

- 機制／性質：biased select 先 poll ctrl_c，Pending 才執行 entered 分支；後者 send 後 pending，不會選中而取消 listener。早期 Requested／SignalFailed 可以讓 entered=None，demo 從停止 channel 取得真事件，不把 None 冒充 ready。
- 需要／用途：READY 同時意味 listener 已進入及兩份真 held HTTP 已開始，使用者才知道此時按鍵能驗指定兩份取消。正常完成同樣 abort／await controller，避免留下等待。
- 操作：demo 明改成原行1／3／4／5、gate0／1→原行1／3，省略已完成 `/ok` 的理由已寫；close 保存 listener 與 server 兩份收尾結果，兩者等完才用 `?`。
- 本人實測：直接執行自己編譯的 binary child，以 pipe 讀到 flushed READY 才對該 PID 送 SIGINT。0.03 秒結束，exit130、Requested、1／3 Cancelled、4／5 Unstarted、GET=2、全部資源收尾文字及空 stderr 均吻合。
- 界線：這次真 signal 操作是 Unix 本機證據；SignalFailed 的常規 test 是注入事件，沒有聲稱重現 OS 註冊失敗。未按鍵的自行完成／exit1 路徑已讀完整 source，未另外等待十秒重跑。

### P6：可自己寫的工作量與答案入口

本節只要求補 record／stop_and_finish／run 三個 body，Input、single、有限 fill、signal helpers 與 checks 已提供。主文在提示關閉時已教狀態、步驟、結果與必要新 API；fold 中是提示及完成實作，不靠打開答案補必要概念。checks 是可追加的完整 cfg/test module，沒有要求讀者從節錄猜缺失的 helpers。

先完成兩個 body、再完成工作圈的里程碑可由完整 source 判讀；本輪沒有執行兩個 body 的中間 mutation，也沒有把正文的 1／6→2／5→7／0 整串數字冒稱全部重現。起始版尚未完成的行為明說，答案是可編譯的實作；階段 copies 不算重複作業。約千行的完整 B 最終負擔仍待未授權的下一節與 final package，這裡不宣稱已驗。

## 本人執行與實頁證據

所有 Cargo 執行在自己的隔離 workspace；B4 source 不變，僅兩個 manifest 的 Fixture path 改為自己的支援包，starter library 追加 exact checks。自己的 lock 離線解析，使用固定 `cargo +1.99.0`。沒有改根 workspace／lock，沒有重跑全 workspace。

| 有限驗證 | 本人結果 |
| --- | --- |
| complete refilled stop、uncollected Check／panic、prequeued stop、SignalFailed／closed 四個各自 filtered tests | 四項 exit0；每次1 passed、6 filtered out |
| starter prequeued stop，使用完整 appended checks | exit101；實際可編譯，斷言 Finished != Requested，符合尚未實作；不是接受任意非零 |
| build b4_signal_demo，真 READY→SIGINT | build exit0；child exit130；逐行分類與 GET=2 相符 |
| 自己 frozen 單頁 strict build | exit0，No issues found |
| 桌面 1280×800、手機390×844 | 26 張 context／element／fold 畫面全部本人親看 |

raw logs 與指令在 `probe-workspace/selected-probes.json`、`refilled-stop.log`；signal 原文在 `signal-proof.json`、stdout／stderr files。沒有另跑其餘三個完整 tests、Clippy、全部測試總數或所有失敗 mutation，不借用 Root 執行來補成自己的證據。

實頁檢查有真的載入 CSS 與 SVG：兩尺寸 HTTP200、回應 bytes 都與自己的 render inputs／frozen SVG 相同。不是只看 SVG 檔或無樣式頁。圖可在手機直接讀；橙框觀察停止、藍框已提交、灰框 pending、綠框完整 Batch，箭頭是處理順序。1／4、3、5／6 能回對前表，圖末明說是原 input_index 而非 Id。SVG 760px 高需要自然捲動；元素全圖與原位置均看，不靠放大基本字。

另看主材料表、gate／原行對應、no-refill、by_ref、後備清理前快照、guard owner、listener 註冊、caller await、提示關閉／答案展開、READY 操作及退出說明。手機表格自然換行仍能逐列讀；長 Rust 區塊沒有硬換行，未逐一檢查所有橫捲位置，不能據這次畫面宣稱每一長行都同時可見。

隔離 preview 沿自己的既有 theme、不含 navigation.top；只把缺席的15.9／11.5兩個前文連結轉為完整站點 URL，並把 frozen 圖絕對路徑換成相同 byte 的本地資產。轉換在 `preview/render-inputs.json`，沒有修改原正文或驗證遠端 live 連結。未驗全站導覽、搜尋、深色模式或 staging／正式發布。

## 原判與停點

目前必要項 N=0、可選項 O=0；上面每項 PASS 都有材料與限度，不以零發現數代替驗收。中途的 listener／API／source-copy／實頁未知已有本輪證據補核；以下仍保留未驗：Windows signal、OS 真註冊失敗、未按 Ctrl+C 的十秒完整路徑、所有中間實作錯碼、B-only 略過 A 的跳讀、15.11 最終 CLI／JSON／完整退出優先序、最終 B 行數與完整工具發布。

保存本 ORIGINAL 後停止，等待新的 B5 授權；不先 cross、不查看同行判斷，不將 B4 通過擴為 whole B 或 whole15 QA。
