# 第 15.11 節 ORIGINAL TECH：B5 v14

本輪未確認必要修改項；有一項可選的入口規模口徑改善。報告映射、typed error、公開入口、真 CLI 與出口責任在已查範圍一致。這是 B5 的獨立原判，不是 whole15／發布驗收，也沒有與同行交叉覆核。

## 身分、隔離與版本

- 審閱者是既有 TECH15，曾寫第 1、8 章、11.4、14.4／14.5／14.7／14.8，沒有寫第 15 章。是非作者技術審查，有專案與 Rust 背景，不聲稱基本 Python 讀者的盲讀。
- 唯一新教材包：`/workspace/work/tutorial-implementation/frozen/ch15-b5-prerequisite-v14`。15.11 一頁、9 BODY、33 archives；27 個完整 byte classes，其中兩個是 generated Cargo.lock。連 assets 副本與兩份閱讀包 metadata，共 45 個實體檔案。
- page SHA：`5ecda9e6e446884107011967328651b3130eea63ba162063ace009604c98a511`。SVG SHA：`f60e2e1d1e8bcf996fcf094c4aa14befa560bda768d2ef425113dac12f5afa8b`。
- 先保存全部 45 檔 SHA，按順序讀完 BODY 與提示／答案，再完整核 unique source。收尾重算與初始全相同。沒有改學生稿、root Cargo／lock、共享站點或 staging。
- 沒有讀第 15 章同行紀錄、作者 notes／ledger／handoff、Root outcome／disposition／proof。Root 後來傳來共 cache 已釋放的調度訊息，當時自己的 probes 已完成；不採其 build 訊息作本人的驗證。
- A `042f134ed1b349a78ea5e2717b0b15c26d9197368b3c2b60089a0bbf614c3189`、B-prefix `467fe5f9207a38c6a4373e36aa82cd3cafc33a3c3089e340228f2ddb223cbd69`、B4 `88f48a3b819a3d5be292a08f648a90447d18f2c76e044877af7b0d2c3950459d` 原判 bytes／分級不回改。A 原先的必要項不因本輪 B5 沒發現必要項而消失。

工作證據：`/workspace/work/ch15-technical-b5-v14/`。`read-checkpoint.md` 先保存 001-001～003 的理解／未知，再追加讀完後的理解；後續 probe／視覺結果不回填成早期已知。

## 全文、完整 source 與 generated locks

9 BODY 全讀，包括每步判準、三層提示、完整答案、main／library 邊界與獨立使用說明。沒有只讀導覽或 snippet 當作完整程式。

| 材料 | 本人閱讀及 byte 證據 |
| --- | --- |
| 完成版 report.rs／app.rs | 兩個完整檔案，含所有五項 report tests、四項 app tests 及全部 helper |
| 完成版 batch.rs，26,001 bytes | 完整讀所有排程／signal helpers 與七項 tests；最後 mixed HTTP→Report 的新增斷言也讀，不沿用舊結果略過 delta |
| starter report.rs／app.rs | 完整讀三個 stub、提供的 render／write_output／run／validate/load 等正常能力 |
| starter batch.rs | 自行證明整檔是已全文讀過完成版 batch.rs 的 exact prefix；不是只比函式名稱 |
| 兩份 checks 整個 cfg/test module | 分別自行證明為完成版 report.rs／app.rs 的 exact suffix；imports、helper 與 tests 均在已全文閱讀範圍 |
| 兩份 main.rs、兩份 lib.rs | 逐檔完整讀；核 root pub use、private modules、help/version 輸出與 fatal 路線 |
| config/error/input/single/test_fixture | 各 unique 完整檔案都讀；六組相同 copies 以 own SHA equality 對映，沒有抽樣代替完整 source |
| 兩份 manifest、README、config integration tests、兩份 invalid fixture | 所有 unique 完整檔案都讀；相同 copies 由 byte map 記錄 |
| SVG | 完整 XML、intrinsic 360×780、文字與連線，再核兩尺寸真實頁面 |
| 兩份 Cargo.lock | 機器 TOML 解析與依賴核對，不聲稱逐行人讀 generated lock |

`source-byte-map.json` 記 33 paths→27 classes。25 個非 lock unique classes 由完整人讀或自己的 exact prefix／suffix／copy 證明覆蓋；兩個 lock 的 `full_read=false` 刻意保留，其機器驗證另在 `lock-analysis.json`。

兩份 lock 都是 v4，各 183 packages；clap4.6.7、Serde1.0.229、serde_json1.0.154、reqwest0.13.5、Tokio1.53.3、url2.5.8、axum0.8.9、tempfile3.27.0 與兩份 manifest 所選版本一致，own package 依賴名稱相符，其餘 packages 都有 registry source，沒有教材外 path 依賴。自己隔離 package 的 `--locked --offline` 執行也實際接受 supplied lock。

## 原學習問題與主要通過項

### P1：同一份 owned Report 保留原輸入與缺省資料的意義

原需是 001-001「讓別人真的拿它檢查一份網址清單」，以及 001-002 指定原行順序、duplicates、JSON 與退出政策。不能只因換了一份 struct 就說完成工具。

- 身分／操作：001-003 先分 Batch 的整批理由與各 Entry；Report.cancelled 只由 Requested 決定。Row 保 index／original／kind，HTTP 才有 status／final_url；未量測、未交 Check、未提交的 elapsed 為 None。
- 機制／性質：entry 被接手，Check.original／elapsed 與 Outcome move 到 Row；Cancelled／Unstarted 沒有 Check，使用空欄位初始化。entries.into_iter().map(row).collect 保序；沒有排序成另一份完成順序、重編行號或去重。
- 需要／用途：caller 可以對回原清單，也能從 kind 分類，不解析可變 detail；null 不會冒充數字0或已做過的檢查。
- 例子支持：六筆手工 index1／3／4／6／8／9 刻意包含同原網址的不同結果、Input、Timeout、取消與未開始。正文明說 12／1000ms 是手工映射材料，不是 HTTP 實測。本人 mapping test PASS；另跑新增 mixed HTTP→Report 部分，保留真 http／input／request 與資料狀態1。
- 界線：Report／Row 是 README 明說的被動可交換資料，公有欄位／Deserialize 不構成對任意外來報告的完整語意驗證。本節沒有作那種承諾。

### P2：文字與 JSON 借相同模型，表示規則不混成網路分類

001-003 在 render code 前教 u16／u128、Option<T> 的 Some／null，以及新選 serde_json::to_string 的借用、Result 與不 pretty 的角色。

- 機制／性質：JSON 用 to_string_pretty 加 LF；文字逐行，original／final_url／detail 透過 JSON 引用處理引號與 LF，Option 數字直接成數字或 null。
- 需要／用途：一份資料有兩種出口，不維護兩份分類規則；原文或診斷內的 LF 不會偷變成另一筆文字結果。
- 材料支持：codec test 以含引號原文、人工 LF detail 驗一筆一行；整份 JSON 讀回 Report 並比較全模型。本人此 test PASS。它不是一次真 HTTP 產生該 detail 的證據，正文有明分。
- 界線：固定的是欄位／順序／表示規則，不是遠端網路、毫秒或 dependency 詳細診斷的文字。u128 的 Rust／JSON 整數編解碼不代表每種外部語言都會以相同精度處理。

### P3：候選資料狀態、真正出口完成與 fatal 各有責任

001-002 先整列 fatal2 > cancel130 > itemfail1 > success0；001-006、007 再實作，原需要是在 stdout 不完整時讓 caller 知道失敗。

- 機制／性質：Report.status 先 cancelled、再查 passed；write_report render 後交 write_output，write_all／flush 都成功才回 Ok(status)。From<io::Error> 專指 Output，讀檔發生處明選 Read。
- 需要／用途：已寫入 buffer 的 bytes 仍可能 flush 失敗；取消資料不能令破損 JSON 仍回130。將出口責任放同 helper，main 與測試不各猜一次何時完成。
- 材料支持：ControlledWriter 分別有拒寫、拒 flush、bytes、flushed，沒有用環境權限碰運氣。本人 flush test PASS；starter 相同 test 真敗在 Ok(0)，沒有寫入／flush，不是任意非零被接受。另以真正 CLI stdout 導向 Linux /dev/full，資料本來應回1，實際 fatal2 並在 stderr 保存原因。
- 界線：write_all／flush 成功不保證遠端持久保存；失敗時可能已有部分 bytes。正文明說那份 stdout 不能當作完整 JSON。

### P4：真 CLI 沒把 library 的正常資料與致命錯誤混掉

001-004 在 Settings 前教整數 long 帶值、typed defaults 與正數驗證責任。001-008 先說 private modules／root re-export、run 的讀取與 listener 順序，才給入口 source。

- 身分／操作：clap 解析數字，0仍是可解析整數；validate 在讀檔前拒 jobs0／timeout0。help/version 的 Err 是成功顯示請求，use_stderr／exit_code 沿實際7.6。
- 機制／性質：run 載入小清單、建立 client／Scheduler，再 spawn listener、等待 entered，之後才提交；完成後 await controller，再從同一 Batch 建 Report。SignalFailed／StopSourceClosed／Controller 仍回 typed fatal，不假稱使用者取消。
- 需要／用途：使用者可以設定工作量與期限，也能從退出狀態區分「有報告但未通過」與「工具無法完成」。來源失敗不能改寫成正常取消報告。
- 材料支持：本人真 binary 驗三筆 invalid fixture 的完整 stdout JSON／exit1、help stdout／exit0、零 jobs 優先於缺檔／exit2；真受控 HTTP 的兩份 held GET 進入後才對 own child PID 送 SIGINT，回130，index1／3 cancelled、4／5 unstarted、原文及 duplicates 保留、整份 JSON 可讀且 stderr 空。這是新 main 的操作證據，沒有用 library test 代替 stdout／退出碼。
- 界線：SIGINT probe 僅 own Unix child；停止後本機 server handlers 另放行、shutdown、等完。沒有聲稱 client drop 撤銷 GET 或自動關閉遠端服務。

### P5：private 實作與可用的公有入口能對上正文

001-008 沿14.4明說 root pub use 提供穩定入口。完整 lib.rs 的 batch／report／single 等 module 都是 private；Batch／Entry 與內部 build_report 沒有被 root 重導出。公有 Settings、Report／Row、RunError、StopEvent、run／check_text／write_report 等可從 root 使用。

- 機制／性質：同一型別 re-export，不是公開整個內部 module；外部不需拿到 Batch 就能用工具。
- 需要／用途：caller 不綁內部檔案位置；具體分類與失敗仍由 returned Report／RunError 保留。
- 材料支持：自己的獨立 consumer 真正 import root API、建構與寫出 Report、使用 check_text 取得取消報告及 ZeroJobs；編譯執行 exit0。另一份 external example 使用 url_checker::batch::Scheduler，精確得到 E0603／module batch is private，不把別種失敗算邊界成立。
- caller 界線：check_text 的停止送端要保持有效，要求停止並 await；整份 future 被 drop 不能替代 children 的等待。run 的 body 同樣不提供任意取消時的自動 async cleanup 保證。這些本節先明說，不靠測試自己持有 owner 的安排蒙混。

### P6：三個小步可實作，必要概念沒有藏進答案

新題目限 build_report、Report.status、write_report。完整起點提供舊排程、單筆、signal、render、寫出 helper、Settings／Error；checks 是完整可追加 cfg/module，沒有讓讀者從 snippet 拼遺失 helper。

提示關閉時，主文已有每個動作的輸入、用途、狀態與預期失敗；完整答案是比較實作，不是必要概念唯一入口。starter build_report 原有來源失敗政策要保留，stub 空 items／0 明說不是已完成。source 可解釋 3/6→5/4→6/3→9/0 的里程碑，但本人只選一項 starter 負例，沒有執行整串 intermediate mutations／九項數字，這段不冒稱全數重現。

## 實際先備與固定官方契約

沿自己已讀 A、B-prefix、B4 的實際背景；本輪驗相關 frozen SHA 不變，不拿 Root gate 或目錄作已教依據。`prerequisite-fingerprints.json` 記 64 筆 selected BODY 依據（包括本輪 fresh 補讀與自己先前完整閱讀的 unchanged 單位），不是聲稱本輪重讀全1–14。

| 首次需要 | 當下已教依據／本節本地先教 | 判定 |
| --- | --- | --- |
| clap Parser／args_os／use_stderr／exit_code、long、native PathBuf | actual7.6／006-003、004；本節001-004在code前新增數字值及default_value_t | 已足夠；default不是對Settings derive Default，也不驗正數 |
| Serde derive／deny_unknown_fields／owned model | actual7.7／007-002、003及7.9；001-003先補Option／u16／u128／str／to_string | 已足夠；code與new tests均在契約之後 |
| typed From／Error::source／dyn Error原因鏈 | actual8.6／006-002、003；001-003先說每層分工及Read/Output區分 | 已足夠；String signal診斷不捏造source |
| non_exhaustive／pub use／private module | actual14.4／001-003、006；001-008銜接到實際root exports | 已足夠；external match仍需其他分支 |
| write_all／flush、Stdout／Stderr、ExitCode、fmt及LF | actual7.4／7.5／7.9與自己前輪已讀 BODY；001-007先教本次helper責任 | 已足夠；diagnostic自己失敗仍fatal2，不panic |
| task／channel／timer／select／cancel-safe與owner | 自己前輪已實讀actual11.2～11.5、11.8及B1–4；新batch整檔另讀 | 已足夠；guard只守測試、caller責任不擴大 |
| 內置Fixture的新ready_response／Response::text | 001-004先教消耗Response、完整小body／Result、外timeout與失敗shutdown | 已足夠；只測ready，產品仍headers-only |
| 獨立目錄／manifest／lock／toolchain及新install --path | 前文15.1／7.1／9.1；001-004、009在命令前補獨立lock及安裝範圍 | 已足夠；--manifest-path不替程式換工作目錄 |

新選官方契約獨立核固定 cache：clap4.6.7 derive defaults／attributes、Serde1.0.229 str與Option實作、serde_json1.0.154 to_string／None→null／u128、reqwest0.13.5 Response::text(self)；另讀 Cargo1.99 install 文件的 local source、binary目的地與--locked選定段落。`selected-api-proof.json` 記 SHA／行範圍／URL。保存完整網頁不冒稱全文讀過；library內部的 unsafe 沒有被加入學生程式或在host執行UB。

## 本人的有限執行、cache與實頁

所有 source／manifest／supplied lock 放自己的隔離 copies；starter 只追加 exact checks。使用授權共 cache `/workspace/work/tutorial-implementation/release11-candidate/target`，環境 `CARGO_PROFILE_DEV_DEBUG=0`、`CARGO_PROFILE_TEST_DEBUG=0`、`CARGO_INCREMENTAL=0`。沒有建立新的大型 target、刪別人的 cache／proof 或改共享 manifest。真正 CLI binary 在自己 build 成功後複製到 own bin 並保存 SHA。

| 相稱驗證 | 本人觀察 |
| --- | --- |
| 新 mapping、codec、flush、設定優先四個 filtered library tests | 各 exit0，1 passed／15 filtered |
| 新 ready body＋mixed Batch→Report 的 focused test | 日誌1 passed／15 filtered；目的在Fixture與報告delta，沒有重跑不變B1–4 suite。這條shell後接自己的行數整理，未另保存Cargo獨立exit碼 |
| starter flush test | 可編譯，exit101；實際失敗訊息是預期 Output 卻得到 Ok(0) |
| public consumer／private module負例 | 正例run exit0；負例E0603，非任意非零 |
| 五個真CLI目的案例 | invalid JSON=1、help=0、零jobs先於缺檔=2、/dev/full輸出失敗=2、兩held進入後SIGINT=130 |
| isolated frozen單頁strict build | exit0，No issues found |
| 桌面1280×800／手機390×844 | 28張主要畫面＋2張補充，共30張本人全部親看 |

`probe/selected-probes.json`、`mixed-http-report.log`、`public-consumer/proof.json`、`cli-proof.json` 與 raw stdout／stderr 保存實際範圍。CLI local HTTP probe 只對自己的 child 設 loopback NO_PROXY／no_proxy；產品 normal client code 的環境proxy政策沒改。沒有安裝到全機、release重建、全suite、全workspace、17種CLI無差別重跑，或使用Root／作者聲稱的測試數作自己的執行。

實頁是真HTTP載入CSS與SVG，兩尺寸回應200且bytes吻合。檢查兩張完整表格、Option/null介紹、ready-body限定、候選狀態與出口、提示關閉／答案展開、private/public過渡與獨立使用／規模揭露。手機兩表自然換行仍能對列；圖的藍色資料箭頭與橘色fatal支線可辨認，0／1／130在write_all＋flush之後。必要圖解沒有只放在fold。

截圖限度另記：最初desktop SVG element圖上緣被固定頁首覆蓋；context和補充clip可讀其上半部，原element可讀下半部。補充clip實際360×710，被viewport裁到下緣，不聲稱一張完整無遮擋的desktop SVG；合併自己實見覆蓋全部必要標記，mobile element則完整。這是截圖方法限度，沒有由它推學生SVG缺字。

補充橫捲probe最初錯選pre，scrollLeft=0令自己的assert失敗；保留原script／失敗紀錄，改選實際code scroll owner後為clientWidth311／scrollWidth778／scrollLeft180、overflowX=auto，另親看實際畫面。沒有改CSS或用樣式修改遮掩問題；也沒有宣稱所有長行橫捲位置都已核。

preview只組合9個frozen BODY、去重H1、同byte本地SVG，將缺席15.10前文連結換成完整站點URL；沿自己既有theme、不含navigation.top。這是隔離預覽，不代表改了global或確認live／搜尋／深色模式／全站導航。

## 可選項 O1：統一入口與完稿的行數口徑

- 原句依據：實際15.1／001-001寫「兩題各以約800–1,200行手寫Rust為規模，包含程式與測試」；本節001-009明列核心825、測試898、本機Fixture198，共1921。
- 本人理解／已完成部分：本節已誠實揭露1921及只補三動作、不要求一次手寫全tests；核心825符合約千行方向。此處沒有隱瞞工作量，也不需為硬行數刪掉必要驗收。
- 剩餘關係與影響：入口的「包含測試」會讓讀者先預期800–1200總量，到完稿才換成核心約千行、測試另列。這是規模預期的一致性改善，不是程式缺陷或主要理解缺口。
- 分級：選讀／編輯改善，非必要。延續自己A original已有的同一入口口徑O1；本輪增加B的具體來源，不算同行重現或第二個獨立命中，也不回改舊報告。
- 最小建議：協調時將入口改為「核心約千行，另附驗收tests／Fixture，總量在各題完稿列出」，保留本節分項與三步練習的說明。沒有在本輪修改入口。
- own machine count：1921總行數吻合，含空白／snippet標記、不含lock或stage copies；把lib.rs的兩行test module宣告歸核心，得到825／898／198；若機械從每個cfg(test)起都歸tests，則823／900／198。這兩種分組差兩行不是新缺陷，總量不變，細節存line-count.json。

## 原判與未驗範圍

N=0，O=1。上述PASS限於實際來源與目的，不以零必要項代替whole15驗收。B4舊未知不抹掉：例如未按Ctrl+C的十秒demo路徑仍只讀source、本輪沒重跑；Windows訊號、OS真註冊失敗、所有中間錯碼、完全跳過A的B-only路線仍未驗。

B4原先對最終B／JSON／完整出口的未知，現在由新授權scope的完整source與有限證據補核，沒有回填成B4當時已通過。仍未執行完整16＋2 tests、所有CLI故障組合、安裝／release、遠端服務或全站發布；本節提到作者的CLI驗證不算本人的證據。先前A必要項與其他角色可能漏項不由本輪無cross判定。

封存本ORIGINAL的same-byte initial、sidecar SHA與own coverage後停止；等待Root明確交叉授權，不讀同行或擅自改學生稿。
