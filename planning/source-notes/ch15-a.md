# 最終實戰 A：引導寫作與驗收來源研究

- 日期：2026-10-14；供作者與技術審閱使用，不作首次閱讀提示。
- 研究初期只拆分來源、引導與驗收。Root 逐節釋出 actual 前文後，15.1 共同入口、15.2～15.5 四份引導階段已實作；15.6 完整參考版也已完成作者自查，等待 root 讀實際正文與 source。作者自查不代表獨立首讀或正式發布 QA。
- 各階段均按實際依賴釋出；15.6 已實讀 actual A4／8.6／14.4 才寫。下方「尚等」「未開寫」記錄保留研究當時的階段，不改成早已取得後文。規劃標題不解除阻擋。
- 契約沿用 `curriculum-plan-v2.md` 第 5 節 A 與 15.1–15.6，不加刪除／同步／監控等功能。

## 實際重讀的中英文工具教材

| 來源 | 本次閱讀範圍 | 值得借鏡的引導關係 | 本專案的取捨 |
| --- | --- | --- | --- |
| [Rust Book：Adding Functionality with TDD](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html) | 失敗測試、最小實作、測試通過、整合執行與不同查詢結果 | 先用輸入與預期結果固定需求，看到失敗原因，再逐步完成；library 核心先測，CLI 最後接。 | 採小步回饋；測試只支持該情況，不把一項通過當成完整正確性。主線避免先給完整演算法讓讀者只抄。 |
| [繁中 Book：測試驅動開發](https://rust-lang.tw/book-tw/ch12-04-testing-the-librarys-functionality.html) | 核心函式測試與逐步補上搜尋能力 | 用真實材料、具體預期列出要完成的行為，再讓程式回應它。 | 借鏡敘述次序；不搬用其整份 `Box<dyn Error>` 介面，也不把外部章節當正文必要解說。 |
| [簡中 Book：測試驅動增加功能](https://kaisery.github.io/trpl-zh-cn/ch12-04-testing-the-librarys-functionality.html) | 失敗回饋、文字工作分步、最後完整執行 | 分辨未實作、編譯失敗與行為不符；每次新增一個可預測的工作。 | 明示起始版未完成哪些功能、哪個測試應失敗；不展示未教 placeholder macro。不要照抄翻譯中「函式不存在」但其實已有 stub 的不一致敘述。 |
| [Rust Book：Modularity and Error Handling](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html) | main／library 的責任、設定分組、可回傳錯誤的執行工作 | 讓讀者從變長的 main 看見責任分工的需要。 | A 的檔案收集、內容判定、報告、CLI 各有可單獨確認的行為；不是為了漂亮架構加通用框架。錯誤表示用實際已教的具體型別。 |

前三份 TDD 章節採用同一工具任務，並非三份互相獨立的教學成效證據。本次借鑑的是任務與回饋的安排，不宣稱已實測某種教法更容易學。

## Rustlings：自己動手、提示與參考答案

本次實讀官方 [Usage](https://rustlings.rust-lang.org/usage/)、[`tests2` 起始練習](https://github.com/rust-lang/rustlings/blob/main/exercises/17_tests/tests2.rs)、[對應解答](https://github.com/rust-lang/rustlings/blob/main/solutions/17_tests/tests2.rs)、[hint metadata 的 tests 段](https://github.com/rust-lang/rustlings/blob/main/rustlings-macros/info.toml)、[Contributing 的練習／答案約定](https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md)、[Changelog 的 solutions 段](https://github.com/rust-lang/rustlings/blob/main/CHANGELOG.md)。

可採用的安排：

1. 練習用清楚的 TODO 指出讀者需要改的位置，而不是把整份檔案丟給讀者猜缺什麼。
2. 先讓讀者保存自己的版本，執行測試取得短回饋；提示在需要時才打開。
3. 提示說明要用的能力與如何理解失敗，解答另外保存，方便比較。
4. 答案與練習保持相同任務、資料及檔名關係；修改題目時同步檢查答案。

`tests2` 是補 assertion 的小題，hint 引導比較相等／不相等及參數順序，答案提供幾項具體例子。它不是約千行工具教學的現成模板。A 需要自己設計階段、ownership 與錯誤資料流；不能只把函式挖空並列出答案。

Rustlings 的 hint metadata 也有過早使用 boxed error 等材料，**不整份複製其閱讀順序或提示寫法**。其 Changelog 還記錄移除尚未教的 `map`／`collect` 等修正，支持本專案必須逐項檢查練習與提示的前置。

## 引導階段：讓讀者完成的工作

每一階段的固定關係為：需求與可觀察答案 → 最低介面／起始檔 → 讀者實作 → 分層提示 → 驗收與理由 → 下一階段。這是作者安排，不將全部提示或答案放在題目前。

| 小節 | 當下主要問題 | 讀者自行完成的工作 | 階段結束能看到的結果 |
| --- | --- | --- | --- |
| 15.1 共同入口 | 完成什麼才算一個可用工具？ | 選練習副本與驗證路徑，讀懂需求及不完整狀態。 | 有一份明確的行為契約；知道工作版、完成快照與完整答案的不同入口。 |
| 15.2 A1 收集 | 哪些檔案可以納入，哪些問題要留下？ | root 判定、遍歷一般檔案、link／Unicode 政策、收集資料與項目錯誤。 | 顯示待判定檔案與錯誤，還不宣稱找到重複檔案。 |
| 15.3 A2 判定 | 候選縮小後，如何確定真正內容相同？ | 依大小與 digest 組候選，再比較 bytes，形成可能多個真內容群組。 | 空檔及跨 buffer 相同資料會成組，同大小但不同內容會分開。 |
| 15.4 A3 報告 | 掃描不完整時，報告怎麼仍說清楚？ | 群組排序、按路徑的位元組指標、錯誤與完整性、文字／JSON 同一結果模型。 | 相同掃描結果有固定順序；成功部分與未能判定的部分都被保留。 |
| 15.5 A4 CLI | 怎麼讓別人真的使用並判斷是否完成？ | 接已教 clap、stdout／stderr、ExitCode、fixture 與完整測試。 | 一個 root 參數即可執行；0／1／2 具有清楚意義，JSON 不混入診斷。 |
| 15.6 完整參考版 | 我的版本與完整答案差在哪裡？ | 對照相同需求與驗收；按需要查看完整可執行答案。 | 獨立 package，能建置、安裝、執行、測試；限制與設計理由可查。 |

每頁開始先指出現在已可用的上階段成果，再說新增需求。不在 A1 同時要求實作全部 hashing 與報告，不把完整答案放進第一段。

### 三層提示的分工

| 層次 | 提供什麼 | 檢查是否洩漏過多 |
| --- | --- | --- |
| 提示 1：問題與資料流 | 要保留哪些資料，哪個結果支持這項需求，卡住位置屬哪種工作。 | 不直接列完整演算法 body 或整個最終型別模型。 |
| 提示 2：已教的能力 | 指向已完成的 walker／Read／HashMap／排序等具體能力，指出本次輸入、輸出與 borrow／move。 | 不能只丟外部連結；核心必要能力正文已有解釋。 |
| 提示 3：局部結構 | 一個迴圈或分支如何銜接輸入與結果，仍保留當下主要判定讓讀者做。 | 不把全階段答案拆成三個折疊區塊，讀完等於照貼。 |

完整答案另有清楚入口，讓需要參考的讀者能查閱，不鎖住或要求通關後才能看。提示折疊只是避免不小心看答案；正文必需的概念不能藏在提示裡。

起始版的未完成範圍要明示。可編譯的 placeholder 仍須標示相應測試預期失敗及理由；不能用通過所有檢查的空結果偽裝完成。階段完成快照各自可執行，但行數只計完整答案一份，不累加快照。

## 從契約分解驗收材料

下列是驗收設計，尚非已執行的測試證據。fixtures 必須可控制、與本機環境權限無關；每個 helper 與測試 API 先在前文教過，不能靠未知黑箱建立場景。

| 契約 | 有區別力的驗收材料 | 需確認的結果／可抓到的錯誤 |
| --- | --- | --- |
| root 必須是目錄且非 link | 存在目錄、缺少路徑、一般檔案、指向目錄的 root link | 合法目錄開始；其他為致命設定錯誤。避免 `follow_links(false)` 仍跟隨 root 的漏洞。 |
| 遍歷一般檔案與子目錄，含隱藏資料 | 一般／隱藏子目錄、空目錄、一般檔案、目錄 link、file link | 只納入本次政策可處理的真實一般檔案，不遺漏隱藏目錄或跟隨 links。 |
| 大小篩選不能代替內容 | 相同大小不同 bytes、不同大小、相同內容不同名稱、空檔 | 檔名與大小不同角色；空檔可形成內容群組。 |
| hash 只縮小候選 | 控制候選桶含 A、B、B：digest 相同，A 與 B 內容不同 | 產生 B／B 真內容群，不把整桶當同組，也不只和第一個 A 比後漏掉 B。假 digest 不宣稱是真碰撞。 |
| 短讀取／跨 buffer | 相同內容左右 reader 每次長度不同；跨一個以上 buffer；相同前段、尾端不同 | 一次短讀長短不同不判為內容不同；尾端不漏、不使用 buffer 無效區。 |
| 讀取／metadata 錯誤繼續 | 已教自訂 reader 控制在第幾次讀取失敗；可控制項目 metadata／open 失敗 | 有動作與原因，保留其他可讀群組，完整性為 false。不能吞 error 後回報完整。 |
| 可觀察檔案變動 | 前後觀察資料可控制改變；同長度不同 modified、長度變化、modified 不可取得 | 變化／無法完成必要觀察列項目問題；不把測試未變動當成 snapshot 保證。 |
| Unicode 與原生路徑 | 可表達的中文路徑；在支援的平台建立不合法 Unicode 項目 | 核心保留 PathBuf；不支援項目有可辨識 debug 訊息，跳過並回報不完整；display 不能作 key。 |
| 穩定群組與指標 | 同一資料以不同收集順序提供；多群組、三個相同檔、hard links 的不同路徑 | 排序後結果固定；每群 `(路徑數−1)×長度`，與保證實體回收量分開。 |
| text／JSON 表達相同結果 | 同一個成功／部分失敗的結果分別呈現 | 群組、指標、完整性與項目問題一致；JSON stdout 沒有額外進度文字。 |
| 結束狀態 0／1／2 | 完整有重複、完整無重複、部分失敗、設定失敗、控制輸出失敗 | 0 完整、1 不完整、2 致命；沒有重複也可以完整成功。 |

平台無法建立的 link／非 Unicode fixture 明示適用條件，不要求管理者權限、不以 `chmod` 期待所有機器都讀取失敗。測試框架內部的可控制 reader／觀察資料／輸出失敗介面須先教並讓讀者可辨認其角色；不新增整套模擬 framework。

## A 的完整參考版邊界

預定總量約 800–1,200 行手寫 Rust，包含測試與 fixtures；不計 dependency、lock、生成碼、正文或所有階段快照的累積。目前已有共同引導、資料材料與A1收集階段；**不能宣稱已達完整工具的千行範圍或A全部功能測試通過**。

保持一個 root、同步處理、size → BLAKE3 → 真正 bytes 判定、多個真內容群組、Unicode 策略、可觀察 metadata 前後變化與讀取失敗、項目錯誤繼續、穩定文字／JSON、exit 0／1／2。沒有刪除、硬連結整理、監控、並行 hash、精確可回收量或一致快照。

完整參考版與引導版使用同一契約與已教 API，不另外改成陌生的高手寫法。若實際 API 需要新前置，先交 root 調整前文或選更小的已教做法，不在答案檔首次引進它。

## 圖解候選與未完成證據

- 保留第 9 章的候選桶／多真內容群與不對齊短讀取示意，A 以自己的具體資料回收，不重畫不同名稱讓讀者重新對應。
- 必要時加階段資料流 SVG：檔案資料與項目問題一路如何到最終報告；不用箭頭把 error 畫成被忽略的旁路。
- A1 沒有新增 SVG；15.1／15.2 作者已看實際桌面／手機頁面，範圍見author-ledger。後續與完整工具尚未驗收；作者研究與局部實頁檢查不能代替獨立首讀、技術及發布驗收。

## 15.1 開寫前的補讀與具體材料

- 再實讀 English／繁中 Book I/O Project 的任務入口與 TDD 對應章，採已知能力組合成工具、先保存需求／失敗原因／小步完成與真正執行，不搬用未教placeholder macro或Book預先使用trait object的安排。
- 共同頁只需已實讀7章（最新gate ch07-complete-prerequisite-v10）與9.1 tooling-v1；B只保留任務導覽，不用Tokio／Waker／unsafe或HTTP Rust API。
- 新建真實A材料六檔：三份Rust LF5bytes（包含一般與點開頭子目錄）、一份rust LF5bytes、兩份空檔。作者以原bytes驗證兩組與依路徑重複內容10bytes；不是已執行A掃描程式的證據。
- Repo外Cargo new首次實跑發現此環境沒有rustup default，repo內toolchain檔的選擇不延伸到外部資料夾。實讀 [rustup Overrides](https://rust-lang.github.io/rustup/overrides.html) 的command shorthand與選取順序，先教cargo +1.99.0再示範；外部new/check/test重試全通過，沒有修改全機預設。
- 15.1資料表、里程碑、Cargo命令與判斷折疊在作者scratch生成頁桌面／手機已親看；沒有SVG與A核心code，詳細範圍保存author-ledger。


## 15.2 開寫前的實際閱讀與取捨

- 實讀 root 的 `ch15-entry-prerequisite-v1` 全六 units；同一份六檔素材、workingcopy、里程碑與toolchain契約接到A1，沒有改題。
- 實讀 `ch09-files-prerequisite-v3` 全七 units與全部scan.rs；另讀對應8項tests。該gate未存tests本體，所以對照當下repo中root已核的tests/scan.rs，不把archive範圍擴稱為含tests。
- 另實讀8.4 actual `ch08-complete-prerequisite-v8` 004-001至004-004，核固定literal的 `&'static str`；不是只因9.3提名就算教過。A1無Box／dyn／Deref／Error::source等需要後續gate的caller。
- 再實開 [English Rust Cookbook Directory Traversal](https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html) 與 [中文Rusty Book目錄訪問](https://rusty.course.rs/files/dir.html) 必要全文，觀察走訪library分工、metadata與成功項目的關係。採 walker 管待辦、我們保存政策；不用filter_map(Result::ok)、直接unwrap metadata、lossy文字作身分、跟link或排除hidden等與本題不同的做法。中文頁舊error_chain框架不搬入。
- 再實讀 [walkdir 2.5 WalkDir](https://docs.rs/walkdir/2.5.0/walkdir/struct.WalkDir.html) description、IntoIterator、new、follow_links、follow_root_links：遇到Err仍能繼續，但打不開目錄不能得到其內內容；內部follow預設false、root follow預設true，需要分開控制。
- 再實讀 [DirEntry](https://docs.rs/walkdir/2.5.0/walkdir/struct.DirEntry.html) path／into_path／file_type／metadata必要段：path借用、into_path交owner；file_type不額外查詢，資訊不會固定未來。selectedAPI只用9.3已教部分。
- A1工作版不用未知todo!／unimplemented!，只用已教enum／Result／Vec與明示固定結果。驗收檔放checks不自動編入，正文先教此路徑契約並要求加到同檔tests；完整快照真的編入完全相同八項，不把zerotests聲稱功能完成。

### 承接7.5 flush修正

Root回報7.5真實no-LF stdout緩衝bug並已修：write_all完成後還須flush，flush失敗不能回成功。15A的A4／完整版將明確檢查flush，分類為致命輸出2，不依報告有LF；目前A1只提供固定資料的階段demo，沒有承諾finalCLI退出契約。需在A4正式開寫時實讀root修後actual，而不是把此作者訊息當正文先備。


## 15.3 寫前補讀（2026-10-14）

已實讀actual15.2 v2七units與9.4 v4九units、必要完整source/tests，明確知道元件第一次FileIssue返回後不交partialgroup，caller才負責保存原生offender與重建其餘桶。本stage採用同A1型別本體，外部chaptercrate只供來源比對。

再次線上實讀English Cookbook Hashing的SHA-256file例（Context update有效buffer前段、EOF），中文Rusty Book對應SHA-256filebody；只借鏡分批讀取與說明次序，不搬anyhow/error_chain/ring/data_encoding/BufReader等不必要依賴。兩份是同例的英中版本，不當成兩個獨立效果證據。

正式BLAKE3 1.8.7 Hasher new／update／finalize本體已重讀：update依序接受&[u8]、finalize借狀態交Hash，沒有消耗或清空。File.metadata官方method段已讀，回ioResultMetadata觀察handle對應資料；由9.4已有before/afterpath+handle檢查承接。沒有用未教update_reader/mmap/rayon或強調未測效能倍數。

新增helper只用已有FnMut與owned/borrowed基本能力，正文在首次source前解釋每份輸入／輸出／borrow／offender契約；受控 failed read 與 forced bucket 檢查不靠 sleep／權限或真實 hash 碰撞。


15.3作者實作與stage證據已保存author-ledger：A1同一型別來源、FnMut讓測試提供實際失敗reader、原生offender移除後從有限remaining重建、20tests與外部workingcopy一步步回饋、真正桌面／手機SVG與提示。作者自查不是獨立首讀或正式releaseQA；A3/A4/完整A尚未開寫。


## 15.4 寫前來源與選用契約

- 新實讀English Cookbook StructuredData（unstructured JSON／nested TOMLstruct）與中文RustyBook encoding/structured.html對應body。採先固定資料shape／回查實際值；不搬Value/json!、raw string、TOML、byteorder、endian或error_chain。首次誤開中文encoding/complex.html不可用，改正實讀structured頁，沒有把失敗頁算完成。
- 官方Serde data-model必要option/string/seq/struct段、Serialize的Option<T>/Vec<T>/u64相關實作重讀；serde_json to_string_pretty精確版URL首次InternalError，latest實際回1.0.154內容並已读ResultString與Serialize要求／errors。不是假定API名字近似就有契約。
- 快取精確serde_json-1.0.154/src/ser.rs選serialize_none/some/unit/u64再核：None到null、Some依內容、u64數字；普通模型走derive，不使用這些内部方法作學生API。Option<T>:Ord官方conditionalimpl已讀，實際ordering政策再核後正文先教。
- 7.7 actual重新讀owned模型／from_str/to_string_pretty／escaping／typederrors／unit tests；6.7再讀Ordering/Ord/sort/stable不等固定輸出的完整tie關係；9.4重讀metrics/hardlink/checked integer actual。7.5 v11 005-006正文與Stdoutflush source已實讀，支援A3固定示範寫出；最終CLI狀態仍在A4。


### 15.4 新選 Option 借用能力與作者成果

- Clippy 指出 manual_map 後，沒有直接套用未知介面。實讀 Rust 1.99 標準文件 Option::as_ref 的 selected body（https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref）：&Option<T> 得 Option<&T>，map 消耗含 reference 的暫時值，原值保持可用。正文在 issue-rows source 前先教此關係與 AsRef trait 的區別，再改兩份 helper。
- 先前 fresh 英語 Cookbook／中文 Rusty Book structured-data 段落用來安排固定資料形狀、typed model、編碼後讀回；沒有採用其 Value／json!／raw string 或舊 error_chain。官方 Serde 與 serde_json 1.0.154 支持所選 u64／Option／Vec／owned struct 能力。
- A3 stage 只增加 metrics／report、三個待做 body、同一份九項驗收與兩個唯一 example。作者實跑 baseline20→新增驗收21pass8fail→排序22pass7fail→報告27pass2fail→文字29pass，真六檔 text／JSON 都10bytes兩群。獨立讀者／技術／銜接審閱與正式發布QA仍由root另排。


### 15.5 尚等 A3 actual gate：先讀来源與必要前文

- 已實讀 actual7.6 clap-v7 全六units與7.8 fixture-v8 全七units：PathBuf positional／bool long flag／Parser try_parse_from 的原生序列契約／Error Display/use_stderr/exit_code/print；TempDir lifetime與Command真正child、Output/ExitStatus、env! target路徑、UTF8 borrowedview均是已有能力。A4選--json boolflag，不引入未教的clap ValueEnum。
- 新讀 English Book12.3 task拆分／typed Result替代輸入panic／smallmain（包含BoxdynError提前preview原文）與12.6以重導向揭示出口混入診斷，再將資料與錯誤分流。繁中12.3实际讀108–167與495以後對應段；繁中12.6第二次讀bodytimeout，改讀簡體對應，不能把只有URL/timeout算成功。
- 本頁此刻只研究與讀前文，沒有A4正文或code；完成15.4之後等root实际gate再開寫。


### 15.5 來源研究接到實作

- 簡體 Book12.6 已實際讀26–130全部對應正文，採重導向顯示資料／診斷分流與smallmain安排。繁中12.3對應選段與EN12.3實讀，不複製未教BoxdynError preview、process::exit或當時未知的unwrap_or_else。
- clap精確4.6.7 docs URL不可達；latest實返4.6.7 Parser完整selectedtry_parse_from契約。已再核快取clap_builder-4.6.7 error/mod.rs 218–241 use_stderr／exit_code與843–856 Display實作，Display寫formatted訊息，caller's write_output能接手write＋flush。沒有將failedURL算成功查閱。
- 本題另外明訂partial1與fatal2，stdout只交Report或requestedhelp，項目診斷保留在Report。Stdout實際flush與typedOutputfatal，不假定Book eprintln或process::exit能代做本題交付保證。
- 7.9 actual009-005/007 writer與source再核作controlledwriter先備；15.4 actual report-v5全八units再讀。A4沒有新增Rust／thirdpartyAPI；let _刻意忽略已致命診斷的Result在main source前解釋，並非忽略工作結果。


### 15.6 寫前 fresh EN／ZH API 研究

- English Book14.2 Exporting a Convenient Public API（97–221）與簡體對應109–236實際閱讀：借pubuse解耦caller路徑與作者檔案安排。Book範例仍pubmod；本題採actual14.4已教的private mod＋rootpubuse，並保留publicpayload型別的可使用名稱，不假稱舊stage公開module刪掉也相容。
- Actual14.4十一units與8.6六units已實讀，不靠header或名詞清單；Error要求Debug+Display與source借用／dyn lifetime、Someborrowownedcause、Display只目前context避免重複皆有正文支持。
- 最後toolscope不新增cratepublish/featureflags/async/hashalgorithm/delete；本題同一CLI公開metadata仍publish=false。只重整入口、必要StdError報告器和重疊驗收。

### 15.6 選用標準原因與完整答案

- 官方std::error::Error的source方法selected body已實讀，回Option<&(dyn Error+'static)>借thiserror既有原因。TryFromIntError官方頁的首輪find沒有命中，不算取得；後來以標準HTML方法擷取實際impl Error段成功，支持當下選用能力。最初可選HTML parser套件未安裝，改stdlib處理，不增專案dependency。
- serde_json Error頁首輪搜尋沒有取得StdError段，沒有把failedfind當完成；精確快取serde_json1.0.154/src/error.rs的StdError impl與Display再讀，Rust實際編譯source借出也通過。正文只選用共同Error/source，不教或暗用nightly sources、downcast、額外JSONValueAPI。
- A4 public module與最終private module＋rootpubuse是另一package的介面安排，沒有破壞已凍結stage。ownedReport、scan_report、payload名稱、私有callback違約與一般檔案partial的差別，在15.6正文各首次需要之前說清。
- 最終只一份可用答案：核心995行＋662行必要tests/fixture，共1657；stagecopies不累加。17Unix驗收、獨立literalcopy、realCLI／textJSON／輸出故障與實際桌面手機原位閱讀都有作者證據，範圍與限制在ledger。完整答案現在已實作；root前置核讀／獨立首讀／發布QA仍分開進行。
