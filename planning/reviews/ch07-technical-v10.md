# 第 7 章技術初判：凍結 v10

## 身分、獨立性與實際範圍

我是非第 7 章作者的技術審閱者，先前為第 12 章寫作與其他審閱使用過 Cargo、Rust I/O 及相關前文背景；**不是新的 blind 首讀者**。本輪沒有取得第 7 章 first、技術／銜接 review、處置、作者 notes／ledger、live 改稿或其他 snapshot。以下先自行重建正文、來源與先備，再保存初判；尚未交叉閱讀同行結果。

依 clear-tutorial、review-protocol 與專案術語指引，以懂基本 Python、已順序讀前六章的讀者為入口。具體錯誤、資料／狀態與能力的機制、採用理由、比較範圍、API 的首次需要都分開核對。完整程式、測試、helper、答案、manifest 與失敗材料採同一先備規則；不能以測試通過代替教學或契約充分。

唯一第 7 章 actual：`/workspace/work/tutorial-implementation/frozen/ch07-complete-prerequisite-v10`。

manifest SHA-256：`98c33ca8e367f9c46c8ac2d5c60e201f7fb2f2db35db1c1664798f241a0b6472`。

9 頁、62 個閱讀單位均已完整讀取：001 的 6、002 的 5、003 的 7、004 的 8、005 的 8、006 的 6、007 的 7、008 的 7、009 的 8 個單位。沒有把答案折疊當成未讀；本技術角色另外完整讀 archive。

以 manifest 實際計數為準，共 **57 份 archived source**：42 份 Rust、8 份成員 Cargo.toml、3 份可見靜態輸入檔、4 SVG。Rust 包含 8 個 binary、15 個 example、6 個 library 根檔與其 modules、5 個 integration-test 檔案；合計 41 個 `#[test]`，另有 tooling 的 1 個 doc test。沒有 archived compile-fail Rust 或 `.expected` 檔；JSON 的拒絕資料與 child 的失敗是另一種材料，不虛構編譯負例數量。

62 個 chunk、57 份 source 與 4 個原位 asset 副本均以實際 bytes 核對指紋。全部 manifest source 已讀，**但章內另需的 9 份靜態材料沒有進入 manifest 或 archive**，見 E1；因此「提供的全 scope 已讀完」不等於「所有材料已完整驗證」。

本輪證據保存於 `/workspace/work/tutorial-implementation/technical-ch07-v10/`：`source-fingerprints.json`、`setup-commands.json`、`probes.json`、`stdout-broken-pipe-probes.json`、`cargo-probes.json`。沒有修改教材、shared Cargo／lock 或 nav。

## 實際前文與依賴回讀

回讀以下必要段落；名稱出現、其他章大綱或 root QA 都不替代實際契約。第 6 章 v10 也包含上輪已親讀、與原稿核對 bytes 的先備，這項先前經驗保留，不重標為本輪新首讀。

| actual 前文 | 本章真正需要的能力 |
| --- | --- |
| 1 v8：1.3 `003-004/005`、1.8 `008-003`、1.10 `010-002/003` | workspace 的共同欄位／繼承、package 與 shared lock、`--locked`；loop／continue；assert、test／cfg(test)、同檔 module 與 crate 根的區別。7.1 的 path dependency 是當下新增而先解釋，不由 workspace 成員身分冒充依賴。 |
| 2 v5：2.6 `006-003/005` | slice 的完整／前段／後段 range、上界不包含、shared slice 每筆 reference 與 mutable slice 的修改；支援 buffer、writer、Command argument slice。 |
| 3 v6：3.6 `006-002/003` 的契約段 | Vec 的 new／push／len／as_slice／as_mut_slice，owned 元素與借來元素不同；索引仍須有效。u8 與重複 array 初始化則在 7.4 首次需要前教。 |
| 4 publication v8：4.1 `001-005/006`、4.2 `002-002/003`、4.5 `005-002/005` | trait bound、trait method 的名稱範圍、derive 的逐欄能力、Debug／PartialEq、模型持有 reference 與持有來源的區別。7.7 當下另解釋 Deserialize 的 lifetime 參數與 owned 模型，不假定所有輸出一定借用 JSON。 |
| 5 v7：`001-005`、`002-001`、`003-002/003`、`004-002`、`005-005/006/007`、`006-003` | Result 的條件式 Debug／PartialEq、type alias；Formatter／fmt::Result 與 Display；AsRef 借用轉換、ToOwned／ToString、for tuple／整值 pattern；From／Into 的 prelude 來源與同型別 identity；`?` 的成功取值與錯誤封裝早退。 |
| 6 v10：6.5 `005-002/003`、6.6 `006-004/005`、6.7 `007-002` 等實際片段 | Iterator／IntoIterator 的 next、state、名稱來源；Vec Debug／PartialEq 與一般收集；fold；lines 的 LF／CRLF／空行／尾端規則及字元／bytes 區別。enumerate、contains、ASCII lowercase 在 7.9 本身先教，不借用未教範例。 |

## 各節的首次需要、機制與用途

下表涵蓋全部 62 個單位及完整來源；「通過」限定該已教關係，N1 與 E1 的範圍另保留。

| 單位範圍 | 自行重建的技術與先備判斷 |
| --- | --- |
| `001-001`–`001-006` | dependency 連到被使用的 library；registry 的版本取得與 docs.rs 的公開 API 角色不同。path 的起點是成員 manifest；inline TOML、dependencies／dev-dependencies、dependency feature、default features、major>0 的相容要求與精確要求都先教後用。cargo add 是刻意修改設定，不能一面改 lock 一面要求舊 lock；update positional dependency 與 run 的 -p 角色不同。兩筆第一章計價得到 240，只證明 path dependency 被使用，沒有冒稱 clap 已被驗收。官方 Cargo 契約與本機 update help 支持這些說明，通過。 |
| `002-001`–`002-005` | 原需要是外部 crate 能否使用公開函式，以及文件例子是否仍符合每筆訂單的運費規則。`///`／Markdown／doc-test 外框先教；integration test 的另一個 crate、library 名稱與 crate:: 的不同根先教，再提供完整 source。240／340／440 的預期來自訂單分開計價，第二個反例能抓「合併後判免運」的錯誤，不只是編譯檢查。cargo doc 與執行 doc tests 分開，三種角度不是互相替代，通過。 |
| `003-001`–`003-007` | 先固定 cwd 和真正材料，再教 fs／io、read_to_string、io::Result alias、Error／kind、NotFound／InvalidData 與其他類別。完整 UTF-8、合法空內容、失敗原因不同。File open／資源 scope、Read use、method 追加文字與本次 byte 數，各先交代。前綴 10＋新資料 9＝總長 19，並未把回傳 9 說成最終長度。Read UTF-8 失敗不改 String，其他 I/O 錯誤可在部分讀取後發生；write! 到 File 與 Formatter 的失敗型別不同。File::create 清空既有內容、example tests 要 all-targets、ErrorKind 可比較而 io::Error 不可比較，都有明確首次契約。source 與說明一致；材料完整性受 E1 限制。 |
| `004-001`–`004-008` | u8 範圍／能力、array 的 `[值; 次數]`、buffer 與有效前段先於 Read::read。n≤len 是合乎契約的來源要求，沒有拿 safe trait 當 unsafe 安全證明；全部學生程式使用安全索引。短讀不等於 EOF、非空 buffer 的 Ok(0) 結束本輪有限讀取、Interrupted 重試，與 source 一致。ShortReader 的正數上限、position／copied、空 buffer 分支都可推得；3／2／0 與舊 C 的觀察確實驗收相關規則。String::from_utf8 接完整 owned bytes，與 I/O 原因不同。ShortWriter 的 write／write_all／flush、未寫後段、WriteZero 和部分工作不撤回都先教；無需尚未教的 BufReader／BufWriter。統計練習不存完整資料，也不要求 UTF-8；通過，固定檔案例的材料受 E1 限制。 |
| `005-001`–`005-008` | Path／PathBuf 與 OsStr／OsString 的借用／owned、非 UTF-8 表示、display 的失真界線、to_str 與 into_string 回原值、join／push 相對／絕對規則都明確。cwd、AsRef<Path> 讀取、args_os 的 owned OsString Iterator 與略過程式名、完整參數／兩種用法失敗先教。ExitCode 數字、stdout／stderr、writeln!、正常返回與立即 exit 清理不同都先教，shell 分流有解釋。正常資料與用法分流可由 source 重建，原生非 UTF-8 檔名的局部執行也成功；成功之前少查 flush 的技術問題列 N1。 |
| `006-001`–`006-006` | 原需要是兩項必要資料、開關、說明與版本的對應，不宣稱手寫 next 做不到。Parser derive 在編譯時生成介面，屬性／欄位規則先給出；query 是 String、filename 是 PathBuf，解析不開檔。try_parse_from 的 IntoIterator、Into<OsString>＋Clone 與完整 argv 契約明確。Err 中 help／version 是 stdout／0，真正用法失敗是 stderr／2；print 與 parse／exit、顯示失敗又分開。對應版本 API source 與原測試支持這些界線，selected help/version test 本輪實跑通過。flag 答案不提前執行搜尋，通過。 |
| `007-001`–`007-007` | JSON object／array／文字／bool、名稱／值／型別／i32 範圍、跳脫及拒絕註解／尾逗號先教。Serde 能力與 JSON 格式分工、Deserialize<'a> 與 owned 模型、derive feature 與 deny_unknown_fields 的政策各有用途。from_str 不等於 FromStr，編解碼回 Result；往返欄位不等於恢復文字排版。NotebookError 保存 io／JSON 原因、兩個 From 與 ? 的先後都沿用已教操作，沒有偷用 Box／dyn Error。已有 Notebook fixture 的 owned／round-trip test 本輪通過；其他靜態輸入的精確內容／分類未驗，列 E1。 |
| `008-001`–`008-007` | 先固定完整 CLI 介面，再選 tempfile 和 path dependency 重用。TempDir 管目錄、PathBuf 只管位置、path 借用、close 接手／可回報清理錯誤、scope 嘗試清理不回報錯誤均先教，不靠未介紹 Drop 語法做例子。fs::write／to_path_buf／try_exists、環境條件 expect 與工具錯誤分開。Command 配置、arg 一筆含空白原生資料、child cwd、output 等待／捕捉、Output／ExitStatus、編譯時 env! 與 CARGO_BIN_EXE、借用 from_utf8、starts_with 先於 helper／tests。缺執行檔是外層 Err，缺筆記是 Ok(Output) 中失敗；空資料成功又與無stdout的失敗分開。完整 source 的 6 tests 都可由正文讀懂；重用 main 的 N1 同樣適用。沒有假稱本輪執行了這 6 tests。 |
| `009-001`–`009-008` | 先固定整行／原行號／原序／ASCII 大小寫／空 query／零筆與 JSON 的答案。Settings、Match／Report 的逐欄能力、usize 平臺範圍、AppError 各層原因與 status 都先教。contains／lowercase／enumerate 的 Item、雙邊比較表示與原文字保留、空行佔行號、owned 報告的作用可由 source 重建；只轉 query 的不足有實際關係。render 格式與JSON全字段、String push LF、讀前驗 query、read／render／write 邊界各明確。RefusingWriter 的非空輸出保證真正call，與讀取失敗分類不同；Command helper 不含 argv0、完整 stdout JSON 的 child test各有用途。這兩個 selected tests 本輪通過；名稱來源的可選一致性補充列 I1，其餘局部通過。 |

## 必要 N1：Stdout 緩衝中的失敗可能被回報成成功

- **來源／位置：** `005-001:3` 的工作是「成功就交出原文字，失敗則回報原因與結束狀態」。`005-006:33` 承諾原 bytes 到 stdout，不增額外換行；`005-006:59/60` 卻以 `output.write_all(...)` 的 Ok 直接回傳 SUCCESS。`008-002:62/63` 與 archived fixtures main 重用同一做法。
- **已教能力：** `004-006:14` 已說 flush 要把中間緩衝資料推送到出口，回傳 io::Result；同節正常版也呼叫 flush。Write、io::Error、match、ExitCode 與 report_error 均已具備，不需引入新方法或新型別。
- **缺少的最小關係：** Stdout 有中間緩衝；短而無 LF 的 write_all 可以只完成把 bytes 交進這份緩衝。尚未取得可供程式判斷的 flush 結果，就選成功狀態，會漏掉稍後真正推送的失敗。正常 scope／行程收尾不是讓程式取得失敗結果的替代操作。
- **本輪可重現證據：** 用原 archived path-cli source 編譯，提供有效 UTF-8 文字，stdout 接「讀端在 child 啟動前就關閉」的 pipe，沒有競態或權限假設。4-byte `Rust`（無 LF）回 **0，stderr 空**；5-byte `Rust\n` 回 **1，診斷 Broken pipe**；8000-byte 無 LF 對照也回 **1**。pipe 從開始就沒有 reader，第一項沒有任何 caller 收到資料。輸入皆為審閱者建立，沒有冒稱教材原來提供這三份 fixture。
- **影響／分級：** 必要技術／輸出錯誤語意修正。原資料與結果約定沒有限定「檔案一定以換行結尾」；加一個 LF 也會改壞保留原 bytes 的規則。現有 newline fixture 正常通過不能支持這個更廣的成功／失敗邊界。
- **最小建議：** 在這兩份 main 的正常成功路徑，write_all 成功後再檢查已教 flush 的結果，失敗沿原「無法寫出筆記」診斷／FAILURE 路徑。用既有 match 即可，不必加入未教的 Result adaptor。可用已教 Write 實作建立 write 成功、flush 失敗的可控制出口驗收；不要求學生先學 Unix file descriptor。
- **對第 7.9 的界線：** 目前非空文字／JSON render 都在末尾加 LF；本輪沒有由此泛報它一定重現上述無 LF 失敗。它的 main／輸出契約若一併整理，仍應明確成功包含哪項推送工作；不能把本次實測範圍寫成所有輸出型別、平臺與場景的證明。
- **定位指紋：** `005-006` SHA `58bc881f617a32dc23d2aec5e49b283f55d9bb7fd33ef4ccfca44623d24bf9d6`；`008-002` SHA `df2d18c554b445c427c315fe8ac8ae25e44f337484b8f3f64ad5d597312d2a28`；path-cli main SHA `09b663127d9b09d7fb2834758ecb665da37e38db77b42f56d2525de4c1a05409`；fixtures main SHA `ec2cd380e46748aadc732f03a2b346e26a2bf9957e3a40fc513858f8be12129d`。

沒有修改程式或先寫修後例子；這項發現是在本輪自行讀到出口契約後，查 Stdout 官方文件與原 source 局部實驗所得，未受第 7 章同行問題提示。

## 可選 I1：ToOwned 的引入方式可以更一致

`009-004` 的 search source 只引入 AppError／Match／Report，卻呼叫 `query.to_owned()`、`line.to_owned()`。7.4 的 `004-005:39` 與 5.5 `005-006:13` 先前明寫 `use std::borrow::ToOwned;`；4.1 也教 trait method 要在名稱範圍中。

這份 source 合法，ToOwned 確實由 prelude 提供；方法的 owned／borrowed 意義已在 5.5 充分教過，所以**不是本章偷偷引入新方法或 ownership 機制**。而且 6.6／6.7 的既有 actual 範例已省略同一 use，這不是 7.9 才首次發生；本輪不把前文這項呈現改成全章未知或重新裁定第 6 章。

可選收益是讓讀者重建完整 search module 時，能追蹤「這次爲什麼省 use」：在合適的名稱說明處簡記 ToOwned 也在 prelude，或沿已教方式明寫 use。只需這個最小來源關係，不列完整 prelude 或新增例子。先前教過 From／Fn／Iterator 等的 prelude 成員，不能單獨證明 ToOwned 是其中之一；若根本不處理這個不一致，也仍保留已教方法本身的局部理解。

這是本輪由原正文與實際前文所得的可選名稱來源補充，未讀同行結果；沒有為文句再跑一個同型實作 test。

## E1：凍結包的 9 份靜態材料尚未完整提供

這是**審閱輸入／證據完整性的限制，未據此判定 live repo 也缺檔**。沒有開 live 材料自行補入，沒有合成「應該長這樣」的檔案冒充原稿。

以下檔案被正文與完整 source 使用，卻同時不存在於 v10 manifest 的 sources 與 archive：

| package | 未封存檔案 | 原需要 |
| --- | --- | --- |
| io-text | `fixtures/empty.txt`、`fixtures/not-utf8.bin` | `003-002` 先宣告提供，read tests／text_io_solution／byte_count_solution 依賴合法空內容和 byte 255。 |
| json | `fixtures/broken-syntax.json`、`missing-title.json`、`unknown-field.json`、`wrong-value-type.json`、`number-out-of-range.json` | `007-006` 的拒絕政策 tests；files.rs 還要區分真正讀入文字後的 Json。 |
| json | `fixtures/quoted-title.json`、`not-utf8.bin` | 真實跳脫資料往返與文字解碼階段的 Io／InvalidData。 |

原 io-text binary 在 archive 工作目錄執行，note 仍是 9 bytes，但 not-utf8.bin 得到「找不到檔案」。原 `tests::absent_file_and_invalid_text_keep_distinct_kinds` 單項實跑退出 101，左邊 NotFound、右邊預期 InvalidData，直接確認當前包不能重現指定結果。這不是 UTF-8 API 錯誤，也不是 intended missing.txt 該被補建；missing.txt／missing.json 本來就應不存在。

Notebook fixture 的完整 source 與可執行 owned／round-trip 已驗；其餘 JSON fixture 的實際 bytes、單一失敗機制和跳脫內容仍未驗證。最小處置是補封存這 9 份**實際原材料與指紋**，再另存材料覆核；本初判保持原樣。雖然已完成所提供的全部閱讀 scope，E1 解除前不宣稱完整資料／fixture 驗證通過。

## 四張 SVG 的原位作用與驗證界線

| 原位／SVG | 親讀 source 後可以支持的關係 |
| --- | --- |
| `004-003`／buffer-prefix | 同一四格 buffer 的索引與 bytes、3／2 的新前段、舊 C 不屬第二次結果；有效／舊資料同時有顏色、樣式與文字，並非只靠相同數字認同一對象。圖在首次 read_all 程式之前，正文解釋 n 與列／格的對應。 |
| `007-004`／json-model | 上框借入 &str，中框 owned Notebook 的三欄，下框新 String；from_str 與 to_string_pretty 箭頭是表示轉換，不是所有權在原文字／模型之間來回 move。80／120／false 與輸入一致，排版不保證相同的說明正確。 |
| `008-006`／fixture-sequence | 操作先後與 TempDir owner 仍持有的範圍不同；建立→等待 child→檢 Output→close，外框沒有把保存 PathBuf 說成維持資源。與實際 close／Output 先後一致。 |
| `009-006`／search-boundary | 實線依序是 Settings／Report／String／stdout 的資料交接，虛線特指具體 AppError 從 read／render／write 回 main；help／version 的 parser 顯示另有文字。沒有把正常 help 的 Err 畫成 AppError 或讀檔路徑。 |

4 個 SVG source 都有明確 width／height（340×452、340×462、340×520、340×650），本文 source 字級為 18。原位與 asset bytes 已覈對，圖中的類型／狀態／文字承諾沒有另找出 source 層次矛盾。

**未 render SVG、未親看 PNG、未做桌面／手機實際頁面或 DOM 檢查。**因此不宣稱18px字在實頁可讀、不碰撞、縮放正常，或已驗證站點asset顯示。圖source覈對不代替visual QA，也不拿其他人的頁面檢查當我的證據。

## 正式 API 與實際執行證據

本輪直接查官方資料，只針對實際使用的契約，不把未教的 API 展開成學生先備：

- Cargo Book 的 specifying-dependencies、cargo-targets、environment-variables：major>0 默認相容範圍、dev-dependencies／examples／integration crates、CARGO_BIN_EXE 的完整 target 名和編譯條件；`cargo update --help` 確認 Rust 1.99 的 positional SPEC。沒有執行 cargo add／update 修改共享設定。
- std 1.99 Read 的 read／read_to_string：有效 n、短讀／EOF／空 buffer／Interrupted、Err 的邊界、追加計量、invalid UTF-8 不變；Write 的 write_all／flush 與 Stdout 的共享緩衝契約，支持 N1。
- std Command／OsString 的結果契約：output 啓動並等待／捕捉，Ok(Output) 可以含 child失敗；native string 的借用與失敗保留原值；Serde container attributes 的 deny_unknown_fields。
- docs.rs 的特定版本 clap／tempfile 網址在本次 fetch 返回 Internal Error，**未把網頁存在當作已讀**。改核緩存 registry 中 exact version 的公開 source／文件註解：clap_builder 4.6.7 的 `derive.rs:71`（輸入bound）、`error/mod.rs:218/233/268`（stderr／code／print），tempfile 3.27.0 的 `dir/mod.rs:434–481`（close／清理錯誤），serde_json 1.0.154 的 `de.rs:2709`（Deserialize<'a>），與 clap／serde features 的 Cargo.toml。它們支持所選API，沒有把內部 Box／unsafe 等實現帶進教材。

執行範圍與結論明確分開：

| 本輪實際執行 | 結果／範圍 |
| --- | --- |
| rustc 編譯原 archived path-cli library／main、io-text main／單項 test target | 使用 Rust 1.99.0、edition 2024；沒有換成 live source。 |
| 原 path-cli 接審閱者建立的非 UTF-8 原生檔名，文件內容爲合法 UTF-8；另無檔名 | 前者 code 0、完整9 bytes stdout、空 stderr；後者 code 2、空 stdout、既定用法診斷。前者材料在 Linux 以 bytes filename 建立，只支持本次平臺和路徑流程。 |
| 原 io-text binary 與上述單項分類 test | 重現 E1：UTF-8材料不在 archive，實際得到 NotFound，test 失敗；不是誤記爲程序應當通過。 |
| 原 path-cli 的 closed-pipe 三項對照 | 小無LF code0／無診斷；有LF與大無LF code1／Broken pipe；支持 N1。 |
| 隔離 Cargo 原 source：clap help/version test、JSON owned／round-trip test、search writer test、search JSON child test | 共4項 selected tests 通過；不是全41 tests或全23可執行targets。JSON child 確實解析整份stdout，writer 確認真的調用且原因是Output。 |

隔離 workspace只複製 frozen clap／json／search 成員，root metadata 採用 actual1 已教共同字段；獨立 generate-lockfile --offline，生成自己的 lock，與 shared root lock無關。使用本輪自己的 target 目錄，環境爲 `RUSTUP_TOOLCHAIN=1.99.0`。解析到 clap 4.6.7、serde 1.0.229、serde_json 1.0.154、tempfile 3.27.0；selected tests 對**這份獨立 lock** 加 --locked／--offline，不冒稱已覈對或使用shared root lock。

探查 harness 的兩次準備問題也保留：最初 stdout-only 的斷言漏看 panic 的 stderr，改爲合讀兩出口後保存實際失敗詳情；隔離目錄最初沒有選定 rustup toolchain，第一次 generate-lockfile 因環境失敗，隨後顯式選1.99而通過。沒有用這些準備失敗報告教材技術錯誤，命令／結果分別保存在記錄中。

## 原初判結論

本輪自行提出 **N1 必要輸出緩衝／狀態修正**、**I1 可選 ToOwned 名稱來源一致性補充**；另保留 **E1 靜態材料封存缺項** 的證據限制。其餘已列的技術與先備關係在本輪scope未發現新的問題，但完整通過仍不能越過 N1、E1 或未做的實頁檢查。

沒有重跑無改動的全套 check，也沒有沿用 root 舊 QA 或同行初判填充本輪記錄。現在保存原初判與SHA副本後停止，交叉／補材料／修後覆核須另存；第12.4仍等待 actual10.8，不在本任務寫新unsafe正文。
