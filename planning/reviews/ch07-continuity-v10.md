# 第 7 章原稿獨立銜接初判：v10

## 角色、隔離與判讀範圍

- 日期：2026-10-14；審閱者：`chapter02_ownership`，第 7 章非作者。
- 這是獨立銜接／先備核對，**不是 blind 首讀**：撰寫 9.2–9.4 時已讀過第 7 章若干實際先備，尤其 7.3、7.4、7.5、7.8。亦曾撰寫 2、4.4–4.5、5；本輪仍按指定實稿回查，不把作者記憶當作先備證據。
- 唯一第 7 章版本：`/workspace/work/tutorial-implementation/frozen/ch07-complete-prerequisite-v10`。manifest SHA256：`98c33ca8e367f9c46c8ac2d5c60e201f7fb2f2db35db1c1664798f241a0b6472`。
- 先按 `001-001.md` → `009-008.md` 讀完九頁、62 個 BODY 單位並留下當時觀察，之後才讀 archive 的 57 個材料（53 個非 SVG 材料、4 個 SVG）。正文展開的 tests、答案亦包含在閱讀中。
- 封存前未讀第 7 章同行 first／technical／disposition／review、作者 source-notes／ledger、live 第 7 章或其他較新 snapshot。疑點均在自己的 BODY／source 閱讀中產生；本輪沒有同行提示。
- 依 clear-tutorial 的 review-protocol 判讀；目標讀者懂基本 Python，必要新概念以正文和真正已教前文核對。名稱表、編譯成功及 source 存在不單獨算理解證據。
- 原逐段觀察留在 `work/ch07-continuity-v10/body-notes.md`，不以後來技術確認回寫當時的「待核」判斷。本初判記最後分級與補查證據。

**結論：2 項必要局部修正、1 項可選介面說明。** N1 是可重現的輸出成功判準問題；N2 是依專案嚴格先教後用規則判定的 Rust literal 首用說明缺口，嚴重度較低。其餘下列關係在本次實讀範圍內已有足夠說明；未執行整套 Cargo checks，不宣稱全章技術／發布驗收通過。

## 按正文重建的先備

按需回讀的 actual 版本為 1 `ch01-complete-source-v8`、2 `ch02-review-fixed-v5`、3 `ch03-review-fixed-v6`、4 `ch04-publication-target-fixed-v8`、5 `ch05-review-fixed-v7`、6 `ch06-review-fixed-v10`。以下列具體教學關係，不以章節標題代替：

| 當前需要 | 已教證據與最短銜接 |
| --- | --- |
| dependency 中 library 呼叫、integration test 的 crate 路徑與計價答案 | 1.9 `009-003` 實際 `pub`／module／`use` 與 `total_with_shipping`：空訂單 0，非空小計未達 300 加 20。兩筆 150 各自為 170，所以 7.2 的 340 來自原規則，不由被測實作倒推。 |
| bool assertion、數字及完整資料的比較 | 1.10 `010-002` 已教 `assert!`；4.2 `002-002..004` 分別教 Debug、PartialEq、Clone，`assert_eq!` 需要 Debug／PartialEq 且借用比較。7.3、7.5–7.9 在新型別首次用時另交代它們是否具有能力。 |
| `&mut` receiver、byte slice dereference、owned 容器／借用視圖 | 2.3 `003-002` 的 `&`／`*`／shared borrow，2.5 `005-002` 的三種 self，2.6 `006-002` 的 slice 長度／索引；3.4 `004-002` 的 String／&str ownership，3.6 `006-002` 的 Vec／push／as_slice。因此 7.4 的 `for byte in &buffer[..n]` 得 `&u8` 再 Copy 數字，不是暗中移出元素。 |
| 泛型、trait bound、標準 trait 的 selected 實作 | 3.3 `003-002` 明說型別參數不給 T 任意能力；4.1 `001-005` 明說 `T: Charge` 提供共同 method。7.4 新 Read／Write、7.6 `Into<OsString> + Clone` 各在使用前補當節契約。 |
| Display、From／Into、cheap borrow／owned conversion、`?` | 5.3 `003-002..003` 的 `&T: Display`／fmt 結果；5.4 `004-002` 的 From 推得 Into；5.5 `005-005..006` 的 AsRef／ToOwned；5.6 `006-003` 的同型別 From 與 `?`。7.4 首次給 `[u8]` 的 ToOwned → Vec<u8>；7.5 首次给 Path／OsStr 版本，沒有假定 Deref。 |
| lifetime 型別參數不是秒數、owned 模型不借輸入 | 4.4 `004-002`／`004-004` 的 elision／annotation 與借用有效性。7.7 在 `Deserialize<'a>` 首次需要處再說輸入借用關係；欄位都是 owned，所以結果可離開輸入 scope。 |
| iterator Item 與 argv move、filter／collect／fold／enumerate、文字行 | 6.5 `005-002`／`005-005..006` 教 next 與 owned/shared/mutable Item；6.6 `006-003..005` 教 filter、collect 的目標與 Vec 能力、fold 累積；6.7 `007-002` 教 scalar／char、LF／CRLF／lines。7.9 的 enumerate 新位置／tuple 契約當節先教。 |

這些回讀確認必要依賴；沒有將第 8 章的 Box、dyn、Error、Deref、Drop 實作或未來 macro 內部當作本章可用能力。必要前文的實際讀取檔案指紋另存在 `prerequisite-fingerprints.json`。

## 四題核對與通過範圍

### 7.1，001-001–001-006：讓 Cargo 建立正確的依賴

1. **身分／關係：** registry 是下載來源，dependency 是 package 的宣告，API 是程式可用介面，workspace 成員不自動相依。BODY 在 TOML 之前交代 path 以所在 manifest 為起點及 `..`／inline table。
2. **機制／性質：已足夠。** dependencies／dev-dependencies／features 改變 Cargo 哪些建置需何種 library；version 要求和 lock 的實際解析分開，`--locked` 阻止需要改 lock 的操作。**需要／用途：已足夠。** 原問題是「Cargo 知道哪個程式庫、哪些功能、哪組版本」，這些宣告逐一回應，不把新版較好當作理由。
3. **操作：** path dependency 後 use 舊 pricing library；registry clap／tempfile 只建立宣告，介面在 7.6／7.8 才教。update 明指練習副本，非假定學生改教材 lock。
4. **例子／界線：** 正常 240 驗證可重用第 1 章功能；不證明 clap parser 已被使用。版本範圍／lock 能說明重現選定版本，不能單獨保證每個 OS 相同行為。此界線有明說，無必要缺口。

### 7.2，002-001–002-005：公開用法與獨立訂單政策

1. `///` 是公開文件、doc test 執行其中 Rust、integration test 是另外一個 crate；`crate::` 所指因此不同。新增配置／語法均在 first code 前。
2. **機制：已足夠。** 文件範例成為可執行檢查，外部 test 經公開 library 呼叫。**需要：已足夠。** 原問題是「別的 crate 能照公開用法使用，文件例子仍正確」；獨立訂單 test 讓合併小計的錯改露出，而非僅增加 assertion 數量。
3. cargo doc 建頁而不執行 test；cargo test 的 unit／integration／doc 路線區分。期望值用 actual 1.9 的業務規則算。
4. 150／150 分別 170 合計 340，對照錯誤地先合成 300 免運；240／440 文件案例保存既有公開結果。可以解釋設計作用，不只知道 compile。通過此公開功能／政策，不宣稱測到所有使用者。

### 7.3，003-001–003-007：先有完整 UTF-8 文字，再談檔案狀態

1. 原需要是讀整份筆記、保留不存在／非法 UTF-8 原因。BODY 先固定工作目錄與材料，再教 read_to_string、io::Result／Error／ErrorKind，File owner 與 mutable Read 推進。
2. **機制：已足夠。** File 的位置推進與追加 String，使 added bytes 不等於結果總長度；create 清空指定輸出，write! 的 File 出口得到 io::Result 而非 fmt::Result。**需要：已足夠。** 保留來源失敗，且專用輸出避免破壞教材輸入；讀取／寫入／回讀各是不同成功條件。
3. prefix 既有 10 bytes，加 9 bytes 得 19；ErrorKind 有比較能力，完整 io::Error 沒有 PartialEq。測試不是直接比較整個 Result<_, io::Error>。
4. 兩份文字都成功才合計；第二份非法 UTF-8 不交局部合計。可控制已有材料解釋流程，不以權限假設製造錯誤。正文／source 的關係通過；沒有被封存的 raw error fixture 另記 U1，不當作已實測。

### 7.4，004-001–004-008：短讀／短寫與完成位置

1. u8 範圍／Copy／Debug、`[Copy值;次數]`、byte slice 與 Vec，在第一個 buffer code 前給出。Read 的 n 表示有效前段，並非一次固定填滿；Interrupt／EOF 各有來源與條件。
2. **機制：已足夠。** read_all 每次只借四格、只加入 `[..n]`，持續到有限來源／非空 buffer 的 Ok(0)；write_all 反覆提交剩餘，WriteZero 防無進展。**需要：已足夠。** 原問題是分批讀非文字、不要把殘留資料當新資料；全文 bytes 先接合後驗證 UTF-8，避免拆開多 byte scalar 就誤判非法文字。
3. ShortReader 的 position／max 正數／短讀 n、InterruptedThenFailure 的三事件及 ErrorKind::from 均在 helper 前。ShortWriter 的接收上限／拒絕資料、flush 的 io::Result 契約均先教。`FromUtf8Error` 的 Debug／Display 在診斷前介紹。
4. ABC／DE 加 buffer 舊 C 的圖能定位每次有效 prefix；受控 reader／writer 不靠不確定 OS 權限。read_all 保存全部 Vec，正文已限制「分次正確接合」而非總記憶體固定。統計練習保留 n／retry／EOF，不保存全文，是同一核心關係的遷移。此節通過；7.5 忘記回收 flush 是 N1，不回判 7.4 未教。

### 7.5，005-001–005-008：原生名稱、資料、診斷與結束狀態

1. PathBuf 是 owned 位置資料，不等於 File 資源；Path 借用，display 為借用的顯示 wrapper，to_str 的 None 不代表該路徑不可用。OsString／OsStr 的 OS 表示不與檔案內容 UTF-8 混同。
2. **機制：已有足夠基本說明。** args_os 移交 owned OsString，首筆慣例是程式名；parsefn 檢查缺少／額外輸入並 move PathBuf。stdout／stderr／ExitCode 令 caller 分別取得產物、原因與結果。**需要：大部分已足夠，成功判準待 N1。** 原需求是由 caller 選檔名、成功交出原內容，失敗交診斷／狀態；名稱轉字串僅供診斷，真正讀檔仍用 native Path。
3. current_dir 的 io::Result 與 CWD 起點、join／push／as_path、AsRef<Path> selected 契約、into_string 的 Err 保留原 OsString，在 first code 前。ExitCode::from(u8)、writeln!、格式化、Bash 重導／狀態觀察均先給操作含義。
4. 參數 parser 練習不是額外提前做檔案 I/O；空白與非 ASCII 名稱不被 shell string 拆開。來源正常輸出例以換行結尾，仍不足支持任意原文字的 output success：N1 用沒有 LF 的小變化確認這一點。

### 7.6，006-001–006-006：讓規則、欄位與顯示請求對應

1. Parser／derive 的角色、command／arg attribute 與 feature，先於設定型別。String query、原生 PathBuf filename、bool flag 各有欄位來源與預設；當節保存設定，尚未搜尋。
2. **機制：已足夠。** 將位置參數／開關／help 規則集中到欄位生成 parser。**需要：已足夠。** 比 7.5 多一個位置參數、可選開關及 help/version，明說手寫仍可做，沒有歪曲原法做不到。
3. try_parse_from 接完整 argv、T 同時 Into<OsString>／Clone，結果 Err 有時是正常顯示請求。use_stderr／exit_code／print 的能力與 print 不自行退出先教；main 正常回傳與顯示失敗另判。
4. default／flag／help／usage 四類測試驗的是規則，正常 argv 再接 main；新增 flag 練習只保存設定。clap Error 不當普通業務錯誤直接 From 包掉。當前能力足夠；proc macro 內部生成合理待第 14 章，不是本節理解必需。

### 7.7，007-001–007-007：借文字，建立 owned 模型，再產生新表示

1. BODY 真 JSON 材料先於 Serde 型別：object／array／數值和文字區分／未知欄位政策。Serialize／Deserialize<'a> 是模型能力，serde_json 是格式工作；不是 FromStr。
2. **機制：已足夠。** from_str 借輸入而此 owned Notebook 不保存 reference；to_string_pretty 借模型建立新 String。**需要：已足夠。** 原需求是保存欄位供下次讀回，不是復原原始空白；deny_unknown_fields 的拒絕拼錯名稱政策，和自訂 Io／Json wrapper 的原因分流，均接到實際存讀。
3. selected signatures／Result／Error Debug、Display／欄位 trait 條件先給；owned 模型離開輸入 scope 仍可用。From 包裝原因先於 helper 的 `?`，沒有 Box／dyn Error／closure map_err。
4. 值往返比完整欄位，不比 pretty 的排版。語法／缺欄／未知／型別／range 對應模型契約；關閉 archived 練習只修改該 bool，不偷偷覆蓋來源。SVG 的三份表示／兩個借入關係與正文對得上。fixture bytes 的完整實查有 U1 限制。

### 7.8，008-001–008-007：資源 owner 留到真正 CLI 驗收之後

1. fixture 是受控材料與已指定答案；TempDir 管資源、PathBuf 只存位置。path 的借用、to_path_buf 的 owned 複本，與 close 消耗 owner／io::Result，各在 test 前。
2. **機制：已足夠。** 每個 test 有自己的目錄，Command 組呼叫，output 啟動／等待／收集。**需要：已足夠。** 原需求是可重複驗收真 main 三出口，避免共享材料互改；中文與空格檔名使「一個 arg，不經 shell 拆詞」真正發揮作用。
3. fs::write 接 str／byte slice，try_exists 區分 false 與 lookup Error；Command 的 builder 回傳 &mut、child CWD、Output／ExitStatus、env! 的編譯期值與 Cargo binary 名稱規則都先教。外層 io::Err 不存在 child status；內層 status 非零仍可能取得 Ok(Output)。str::from_utf8 借 Output，不等於 7.4 接手 Vec 的 String::from_utf8。
4. success 全內容／stderr 空／0；缺檔與非法內容為 child 作業失敗；缺程式是啟動失敗。owner 保留到 child 用完再 close，空內容練習排除「stdout 空等於失敗」錯推。首用 starts_with／Output Debug 亦有正文根據。此節完整 main 延續 N1，不將教學來源正確的 fixture 方法一起判失敗。

### 7.9，009-001–009-008：以有限政策組合完整 CLI

1. 開場真四行材料固定 substring／一行只收一次／原行號／ASCII case folding／空 query 狀態 2／零結果成功。Settings、owned Report／Match、自訂 Read／Encode／Output Error，各有工作角色。
2. **機制：已足夠。** 比較表示與保存表示拆開，query 正規化一次、每行轉比較副本，保留原文；read → pure search → render → writer → main 分層。**需要：已足夠。** 原任務要找原整行且選文字或 JSON，分層讓搜尋不需檔案、排版不需 process，typed 原因讓 main 選正確診斷／狀態；不是從抽象分層產物反推需求。
3. contains／to_ascii_lowercase／enumerate 的 selected 輸入／結果先教，usize／Vec<Match> 的能力條件明說；空 query 在讀檔前驗證。String::push(char) 在 render 前；同型別 AppError 的 `?` 可回收 5.6。Rust `\"` 字面值首次出現在測試，N2 要補最小橋接。
4. 全行索引包括空行／CRLF；ASCII folding 不改非 ASCII、輸出仍原文；Report 離開文字 source 仍可用。writer test 確認 write 錯誤，不假借真權限；CLI 的整個 stdout 再解析為 Report 可抓混入診斷，fold 練的是行數而非片段出現次數。文字／JSON 本版非空輸出皆以 LF 結尾，**沒有執行證據支持把 N1 的 no-LF Stdout 故障直接移植成這個 CLI 的故障**；generic W 的 flush 責任可選補 O1。

## 獨立問題

### N1：選出成功狀態前，未確認 stdout buffer 的尾段已送出

- **版本／位置：** v10，7.5 `005-006`，重用於 7.8 `008-002`；archive `path-cli/src/main.rs`、`fixtures/src/main.rs`。BODY／source hash 均見 input-fingerprints。
- **原句／實作：**「成功時把原文字的 bytes 全寫到 stdout，沒有加標題或額外換行」；`match output.write_all(view.as_bytes()) { Ok(()) => ExitCode::SUCCESS, ... }`。7.5 `005-007` 又用 status 回答 caller 能否當成功。
- **已教／缺少：** 7.4 `004-006` 已明說 flush 要求中間 buffer 推往出口，回傳 io::Result；7.5 只說 Stdout 提供 Write，沒有回收其 buffer 和 flush 的完成位置。write_all 完成接受 bytes，不保證此類 writer 已將所有 buffered bytes 交至下層出口。
- **自己讀到的官方根據：** `https://doc.rust-lang.org/std/io/struct.Stdout.html` 說 handles 共用 global buffer；描述 terminal 的預設 line buffering，並明給手動 flush。沒有把文件的 terminal 限定擴張成跨平台／所有 stdout 的固定實作保證；下面是本環境實測。
- **相稱實測：** 在 Linux，以 rustc 直接編譯這份 frozen path-cli library／main，沒有引用 live source。stdout 接到啟動前就沒有 reader 的 pipe、stderr 擷取。有效 UTF-8 `R`（無 LF）回 status 0，stderr 空；`R\n` 回 status 1，診斷 `無法寫出筆記：Broken pipe (os error 32)`；空材料回 0。bytes／status 全保存在 `closed-stdout-probe.json`。因此 no-LF 的輸出下層已失敗，main 卻選成功，符合原任務範圍的實際反例。
- **分級：必要，技術／完成界線，增加理解負擔。** 不只是習慣上要呼叫 flush，也不主張學生必須先學 pipe。教材承諾處理輸出失敗並交正確成功狀態；不補此關係，學生會把 Write 接受完成理解成實際資料出口已完成。
- **最小修正建議：** 在 7.5 source 前補 stdout 可能暫存沒有換行的尾段；write_all 成功後用已教 flush，處理其 Err 再選成功。7.8 的同一受測 main 同步。相稱 test 可用已教 Write helper 控制 flush 失敗，避免把 OS pipe 操作加入讀者例子。正常 cleanup 會嘗試處理 buffer，仍不能代 main 已經選出的 checked 成功判準。
- **界線／處置：** 沒改教材，沒跑修後 code。只實測 path-cli；7.8 是逐字相同流程的 source 核對，不假稱另執行其 binary。7.9 本版逐行／JSON 均有 LF，和本反例材料不同。

### N2：Rust 字面值中的 `\"` 首次在 tests 出現，先前只教 JSON 格式的 escape

- **位置：** 7.9 `009-004` search tests 首用 `let contents = "Rust \"筆記\"\n".to_owned();`；render／CLI tests 後續亦使用。archive `search/src/search.rs:98`、`render.rs:29`、`tests/cli.rs:67`。
- **已教／缺少：** 6.7 已教 LF，7.7 `007-002` 明說 JSON 文字值的 `\"`，並特別說大括號是資料格式而非 Rust struct 語法。未找到 actual 1–6 或 7.9 此 code 之前說 Rust 的 quoted literal 用 `\"` 表示內容中的雙引號。
- **讀者影響／分級：** 僅缺一個跨表示的基本關係，Python 讀者可能熟悉相同寫法，主線不致停住；但專案明定全部 tests／答案也必須先教後用，不能把 Python 熟悉度或 JSON 格式規則算 Rust literal 已教。列為**必要的局部先備／時序修正，低嚴重度**，不是要求增加跳脫語法整節。
- **最小修正：** 在 search tests 首段前一句說明 Rust 字串 literal 的 `\"` 產生內容雙引號、`\n` 仍產生已教 LF；這裡是 Rust source 表示，之後 Serde 才另編 JSON 表示。無需改現有正確程式或重跑 suite。
- **發現來源／後續：** 自己核 archive tests 後回查實際前文，沒有同行提示。未做修正；不借 7.9 後面的往返 test 反補第一次出現時的教學。

### O1：generic `write_report<W: Write>` 的成功責任可再明確一句

- **位置／原句：** 7.9 `009-005`「先 render，再用 7.4 的 write_all 交給 W: Write 的出口」；source `search/src/lib.rs` 的 Ok(()) 只由 write_all 得到。
- **已教／可用：** 7.4 已區分 writer 接受資料與 flush；函式只借 W，caller 仍持有 output，故可以由 caller 決定 buffer 的完成時點。BODY 目前字面可讀成「交給 writer」，沒有明文承諾替任何 W flush。
- **收益／分級：** **可選，限定用途／API 契約說明。** 一句「本函式完成交給 writer，若 writer 有 buffer，由 caller 在需確認送出時處理 flush」可避免從 Stdout 範例推成所有 W 的硬保證；若作者選讓此 helper 負責 flush，亦需明訂且相稱測此契約。未把所有未 flush 的 library helper 當 technical bug。
- **範圍：** 本輪未為任意 W 擴寫／執行探針；其餘 writer error 型別與分流已有足夠說明。這項可與 N1 的完成位置一起審理，不能以位置相近計成同一 actual CLI 反例。

## archive 核對、執行及圖文證據

### source 覆蓋

manifest 的全部 57 個 source keys 均已讀：tooling 4、io-text 6、io-bytes 7、path-cli 7、clap 4、json 9、fixtures 4、search 12、SVG 4。涵蓋九節正常 main／library、所有 archive tests／helper、完整答案、Cargo、三份封存原始輸入和四張圖。沒有 `.expected`／compile-fail archive 項，未報此章 diagnostic-code fixture check 通過。

重點覆核包括：Cargo workspace 繼承欄位不引新配置語法；新 Error 的 Debug／Display／PartialEq 需求；u8／usize／Vec 的能力及重複初始化；`OsString → PathBuf` move 與 borrowed views；clap input 的兩能力／同型別 Into；serde owned模型及欄位條件；TempDir close 接手 owner／child 等待的先後；字串／byte slice 的 API 接受型別；`?` 的同型別及具體 From；normalT 與業務 status 分流。除了 N2 未見必須靠未教 Box／Deref／closure 等才能理解的 archive 用法。

### 原位圖文

完成 62 BODY 和 source 閱讀後，才以**相同 theme 的獨立 preview**重組指定 frozen BODY：只移除每 chunk 重複 H1、圖 URL 改成本機 copied frozen SVG，相鄰正文／圖順序保留。不是 live 第 7 章，也不假稱看過發布頁完整 nav。preview strict=false，因只複製此章材料；build 實際顯示 No issues found，這不等於 repo release build 驗收。

以真 Chromium／Playwright，1280×1000、390×1000 各四張 screenshot；八張全部經 view_image 實看。metadata 保存實際 bbox，四圖皆顯示 340px 寬，在兩 viewport 沒有被拉到桌面正文全寬；必要字／箭頭／圖說可在原位閱讀，不需放大。

| 圖／位置 | 實際圖文對應與支持範圍 |
| --- | --- |
| buffer-prefix，7.4 read n 後 | 第一回 ABC 對應藍前三格／n=3；第二回 DE 對應藍前兩格／n=2；灰舊 C 是同 buffer 殘留，正文明確不能加入 DEC。圖中合併 ABC+DE 是已處理結果，不是 buffer 永久存全部資料。 |
| json-model，7.7 owned 轉換後 | 上方借來 &str 經 from_str → 中間 owned Notebook；中間借模型經 to_string_pretty → 下方新 String。三框不能誤讀為三份共享 reference，箭頭借入／產出及欄位內容可定位。pretty 與原排版差別正文明說。 |
| fixture-sequence，7.8 child test後 | 外框 TempDir owner 包住建立資料→output 等 child→檢查三出口，close 才移除；下面 PathBuf 只留下位置資料。不是把變數 scope當 child運行範圍，圖說明它是操作先後。 |
| search-boundary，7.9 main後 | owned Settings → read_report owned Report → render 借報告產出 String → write_all 借 bytes；虛線具體 AppError 回 main／stderr。正常0／作業1／用法2與help0分開。parser階段包含 main 已教的 print，不將圖片簡稱當 try_parse_from 自行顯示的 API 保證。 |

截圖與 SVG/source 指紋在本輪 work 中封存。未逐頁實際點擊所有 source links／折疊答案，也沒做全站鍵盤／跨路由／Windows viewport 驗收；此處只支持四張必要圖和相鄰正文的觀察。

### 相稱執行與 unknown

- 自己真實執行：frozen path-cli 的 rustc library／binary 編譯，兩者成功；三份材料的無 reader stdout pipe 觀察如 N1。沒有跑第 7 章全部 cargo test／clippy／fmt，也不借作者或 root 的過往通過作為自己執行。
- U1：manifest/archive 未含正文引用的 io-text empty／not-utf8.bin 或 JSON error／quoted-title fixtures，只封存 note.txt／notebook.json／search note.txt。BODY 的預期／source 的處理邏輯可以查，無法從指定資料確認這些未封存 fixture 的 rawbytes，相關整套test未執行。此為證據範圍 unknown，**不等於教材已確定缺檔**；遵照隔離未去 live 找。
- U2：本 snapshot 沒有 root Cargo.lock；本輪核 manifest 與程式碼的 selected 能力，不獨立聲稱 registry 精確 resolved 版本／全部平台運行已驗證。沒有 Windows native非UTF8路徑／console 實驗，保留正文已寫的適用條件。
- 合理待教：TempDir 的清理契約當下已教，Drop trait 如何實現留8.2；derive 的 compiler/proc macro 內部留14；streaming無限輸入、Unicode完整 casefold、跨平台完整 native FFI 不為本章有限任務的必要先備。不能用這些未教內容解釋本章 source。
- 所有必要／可選項只提出修正建議，尚未修改或核修後版本。本 initial 封存後停止，等 root 開放交叉；不因同行可能已有結果改寫原始判定。

## 封存約定

本文件 SHA256 見同名 `.sha256`；`work/ch07-continuity-v10/sealed/` 保存 initial 文件與指紋副本，`seal.json` 列原 manifest、全部 source／BODY 指紋文件、實際視覺／執行材料 hash。封存時重新計算125個 snapshot 文件，須與讀前 input-fingerprints 一致；該檢查是資料未變證據，不是重新執行教學檢查。
