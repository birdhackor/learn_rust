# 第15章共同入口與實戰A作者紀錄

角色：作者自查，不等於獨立首次閱讀／技術／銜接／release QA。共同入口與 A 分步以 root 實際 gate 釋出，不依未完成章節名稱猜可用 API。

## 15.1 取得的前文與來源

- root 已提供 ch07-complete-prerequisite-v10 全9頁62units；本作者的7章全實作及相應 frozen 必要正文/source 已實讀，另核本次 v10 的009-007 writer補句，確認PermissionDenied分類及人工writer不依真實權限的角色。Root gate 不是完整發布 QA。
- 15.1 新補實讀 ch09-tooling-v1 全6units，重新核 workspace／繼承設定／features／profile／target 的實際範圍，沒有用9.3／9.4尚未釋出的caller能力。
- 寫前再開 English／繁中 Book I/O Project與TDD對應全文／必要段落，採任務先行、失敗可辨、分步完成與完整執行的安排，不抄 Box／dyn、super、未知placeholder macro。來源與取捨另保存 source-notes/ch15-a.md。

## 15.1 本頁的必要關係

| 需要 | 先備或當下解釋 | 材料與可預測結果 |
| --- | --- | --- |
| 重複由內容判，不由檔名／大小判 | 7.3／7.4 bytes；本頁先展示真實六檔材料 | Rust LF三路徑同組；rust LF同大小不同bytes；兩空檔同組。 |
| 按路徑算數字，非實體回收保證 | 開頭需求是報告，不加刪除；本頁給(路徑數−1)×長度 | 3路徑5bytes得10，空檔組0，合計10；不聲稱可回收實體10。 |
| symbolic link／Unicode／變動政策 | Path/PathBuf/OS文字來自7.5；link僅給基本入口角色與A1承接 | 本頁不呼叫symlink/metadata/OSextensions，沒有把預告當已教機制。 |
| A狀態不同於7章的用法 | 7.5 ExitCode角色；本頁依本題指定0/1/2 | 項目失敗1、輸出等致命2；不因已有群組說完整。 |
| B只是任務導覽 | 本頁補URL完整名稱與基本位置角色，不用網路Rust API | 重複網址仍是兩筆輸入，原行序保存；HTTP/runtime/取消細節待B1。 |
| 自有workingcopy/Cargo起點 | 1.2 actual cargo new/check/test，7.1 lock，9.1 workspace；本文新教每次命令的+toolchain選取 | 在repo外建立duplicate-finder，當下main仍是生成起點；0tests不是功能完成。 |
| 里程碑／分層提示／答案 | 本頁先解釋工作版、完成快照、完整參考版各自用途 | A1只收集，A2才判內容；不提前放完整實作或陌生API。 |

本頁沒有Rust source或A/B算法code，沒有新Cargo dependency/member/lock，也沒有新增SVG；真實material檔在examples/ch15-projects/materials/duplicate-demo。範例、操作、題目都只依前述已教或當下基本角色，不提前放Tokio/Waker/unsafe或檔案工程helper。

## 15.1 作者自檢與範圍

- 六份真實材料逐byte核對，三份Rust LF為5 bytes，different首byte改為小寫而仍5 bytes，兩empty為0；正確內容群兩組，按路徑重複bytes為10。證據 `/workspace/work/ch15-practice-guide-verification.json`。
- 首次在repo外plain Cargo new失敗：此環境沒有rustup default，且repo的toolchain檔不延伸到外部資料夾。已先查官方rustup overrides、在命令前教+1.99.0的角色，再於同樣repo外位置真正new/check/test --locked通過；沒有改全機預設。test確實是0項，正文不把它當A功能證據。
- strict scratch build `/workspace/work/ch15-practice-guide-render` 通過，不改共享nav／Cargo。沒有任何新增Rust教學source，所以不為此頁重跑全部repo Rust tests。
- 真正Chromium desktop1280×800／mobile390×844已親看資料表與相鄰素材、里程碑、Cargo命令與toolchain解說、判斷折疊打開後的答案。手機兩表寬340.83／343px，頁面scrollWidth375小於viewport390，內容可讀；命令四行可辨，答案操作正常。證據 `/workspace/work/ch15-practice-guide-visuals` 共10張圖與bounds。沒有宣稱正式共享nav／darktheme／獨立首讀／release QA；HTTP已關閉。
- 實看Cargo命令相鄰文字發現緊鄰中文標點的strong語法顯示literal星號，已加兩側空白、重建並只複看受影響命令／相鄰段落；沒有把source裡寫strong當成已正確渲染。
- 15.2等9.3 actual，15.3等9.4 actual，15.6等8.6／14.4 actual；15.1的生成起點不聲稱A參考版已存在或測試通過。


## 15.1 root integration

Root全BODY與六份材料逐bytes核對，入口precheckREADY；gate為 `/workspace/work/tutorial-implementation/frozen/ch15-entry-prerequisite-v1`，section1／6units，包含hidden與empty六檔。此為A1作者前置，不是獨立首讀或正式發布QA。

## 15.2 actual依賴與安排

Root9.3 gate已提供；本作者先讀全七 units＋fullscan source與當下8tests，並補8.4literal lifetime必要正文。EN／ZH目錄教材與官方walkdir selected契約再實讀，記source-notes/ch15-a.md。此階段只回收現成能力，沒有新的Rust API。

| 必要關係／能力 | 首次需要 | actual已教或本頁說明 | 可預測結果與範圍 |
| --- | --- | --- | --- |
| 六檔是待判定資料，不提前分組 | 開頭材料／驗收 | 15.1資料bytes；9.3僅metadata | 四份5bytes／兩份0都進files，different仍保留。 |
| root fatal vs逐項問題 | 第一介面／任務拆解 | 9.3全BODY typedRootError／ScanIssue，5章失敗政策 | 根不能開始=>Err；項目失敗保留且繼續，空rootdir為成功空清單。 |
| 原生身分／metadata／SystemTime | 模型與observe | 7.5 Path、9.3 001-003至005，8.4實際literal | PathBuf擁有路徑，不擁有／固定content；u64len不同usizeindex。 |
| IntoIterator ItemResult／ownership | work／wire／controlledfixture | 6.5–6.6完整gate、9.3walkerselectedmethod | borrowed判定後into_path，Err保存後continue；next以match不靠未選用Option::expect。 |
| nativePath排序與typedVeccollect | material test | 6.7 sort／mapcollect；9.3 PathOrd | 固定平台native順序；不用PathBuf::clone未知選用能力，先借記錄驗len再consume移path。 |
| checks檔不自動執行 | 第一次指示讀者拿驗收檔之前 | 本頁明示Cargo路徑契約與同檔tests | 手動加入後才8tests；缺材料zerotests無證據，7fail1pass不假完成。 |
| 獨立manifest／toolchain／lock | 第一TOML與操作 | 7.1range、9.1workspace、15.1+toolchain；本頁folder vsname | repo外起始copy可check；新增deps先解析再locked，不帶workspace繼承。 |
| 固定demo的CWD／非finalCLI | inventory命令之前 | 7.3／7.5相對Path；本頁固定cd入口 | 六真檔列出；沒有承諾A4參數或0/1/2，沒有提前藏全部答案。 |
| fixture控制與platform | 驗收材料／檢查表 | 7.8TempDir；9.3 create_dir/symlink/OsStringExt；9.1cfg | OkErrOk可控制，Unix三項有條件；modified不可取得分支未被fixture控制，據實明說。 |

檔案：docs/chapters/15-02-collect-files.md；examples/ch15-projects/a1-starter與a1-complete；共享Cargo／lock由root整合。新packages實際target存在後已notify root，rootofflinealltargetscheck PASS、member/lock已釋出。無SVG，本頁沿用9.3既有操作與資料表，沒有新的空間／buffer機制；實頁檢查另記，不拿HTMLsource當已驗。

## 15.2 作者執行證據

- package fmt、starteralltargetscheck、兩packageClippy alltargets -Dwarnings與complete8tests皆通過；checks檔另外以rustfmt edition2024整理，驗收材料與complete真正編入的tests逐字相同。先前用rustfmt預設edition使import排序不同，已明確2024校正；這是format差異，沒有改測試行為。
- Repo外真實workingcopy用literalmanifest＋起始source `cargo +1.99.0 check --offline` PASS；把同一份checks檔加到模組末尾後，test確實7fail1pass；目錄拒絕那項碰巧通過，不能當能收集的證據。只替換實作後，同一copy與同一驗收8pass，Clippy也pass。
- Repo snapshot的inventory從固定package CWD實跑：第一行6待判定／0問題，隱藏與sub路徑皆在，四份5bytes／兩份0。詳細5命令與stdout／stderr `/workspace/work/ch15-a1-verification/commands.json`。
- Strict作者scratchdocs build `/workspace/work/ch15-a1-render` PASS；共享nav未改。真Chromium desktop1280×800／mobile390×844已親看輸入政策、stub與未完成說明、手動加入驗收、函式任務、驗收政策、完成快照／固定CWD，以及三層提示與答案打開後內容。手機頁面scrollWidth375小於viewport390，沒有整頁橫向溢出；長Rustcode維持codeblock內水平閱讀。
- 初步實頁發現三欄責任表把函式名稱切開；改兩欄後真正複查仍拆字，rectCount assertion也失敗，沒有以改過source宣稱修好。最後改三條平行說明，兩平台實看與DOM確認三個名稱各一行；加入驗收指示同時避免把條件屬性首glyph單獨斷行。只重建／複查受影響正文，不為版面重跑Rust全套。
- 原始22張與最後局部6張screenshot、bounds／open狀態證據位於 `/workspace/work/ch15-a1-visuals`；最終`final-followup.json`保存責任名稱與pagewidth檢查。作者已親看必要截圖，不把全部自動capture列為都已親看，沒有聲稱正式共享nav／darktheme／獨立首讀／releaseQA。HTTP結束後關閉。

後續gate：A2要等9.4 actual，A完整版要8.6／14.4；root另提7.5flushbug修後actual在A4再實讀，完整writer將write＋flush兩步都處理成致命輸出2。當前A1demo不是完整CLI狀態實作。


## 15.2 root integration與15.3開寫前安排

- 15.2 root全BODY／完整兩package／checks／demo precheck完；最小修明確artifact下載三份完整starter、eprintln!的格式／stderr／失敗panic、inventory以普通空白代替未教tabescape。actual gate為ch15-collection-prerequisite-v2，七units與完整stage/artifact/materials。本作者已實讀全七units及修後inventory，不改原review。
- 15.3先實讀ch09-content-prerequisite-v4九units與stream/readers/files/groups/candidates及18tests必要全文；兩SVG也讀source，真正視覺另驗。英／中文Cookbook hash必要範例與BLAKE3 Hasher selected new/update/finalize／File.metadata官方契約已重新實讀，筆記另存。

| 必要關係 | 首次需要 | actual先備與本文安排 | 預期理解 |
| --- | --- | --- | --- |
| size→digest→bytes分工 | 六檔的A2答案之前 | 9.4 001-001至003 | 四份5bytes縮為三Rust一组，兩空檔同組；摘要不等價判準。 |
| 同桶可有多真內容群 | refine工作與forcedcollision驗收 | 9.4 001-007/009，局部借用group | A/B/B得到A單筆+B雙筆，重複報告只保留B。 |
| 原生offender／有限重建 | recovery helper source之前 | 9.4 FileIssue左右確切path、Err不給partialgroups；6filter/collect | 每次移除確切原生path後從剩餘桶重建；保存問題、不把Err當false。 |
| 已教callback支援受控讀取錯誤 | selectedhelper介面與tests前 | 6.3 FnMut/callback/functionpointer；9.4 reader errors | hash／compare可替換已教測試來源，正式呼叫仍hash_file/equal_files；caller需指本次候選的真offender。 |
| 同型別由本package管理 | 首次import／stage移植 | A1 FileRecord/Snapshot本體；1.9crate/module | copied9.4 support改用crate::collection，沒有第二crate同名實體轉換。 |
| 起始 baseline 與新增驗收 | 第一Cargo操作 | A1 tests已通過；A2 checks同檔手動加入 | 不抹掉A1成果，新增A2需求先失敗；只有complete snapshot含完成stage。 |

此安排在15.3 body／source寫前先保存；共享manifest由root統一，局部package用literalfields。A3報告／A4CLI／full answer不提前放在A2。


## 15.3 A2完成與自查

- 新body為docs/chapters/15-03-confirm-content.md；stage為examples/ch15-projects/a2-starter與a2-complete，root已整合member／lock並alltargetsofflinecheck PASS。normalexample採唯一a2_groups，沒有重複A1 inventory名稱。
- 兩stage collection.rs逐bytes等於A1完成版，保留原8項；stream/files從actual9.4 adapted只改local crate::collection imports，沒有混ch09_files不同FileRecord實體或增加未教轉換。content.rs三個待做body完全隔離；analyze接線與所有helper輸入／結果／borrow／failurepath契約均在首次source前解說。source支持函式的selectedAPI仍只依actual6.3/6.5/6.7/7.4/8.4/9.3/9.4。
- Body先解釋root vsitem保留、單次Err沒有完整groups、必須移除確切原生path後有限重建，再給callback與panic契約。真FileIssue保存繼續；panic只防程式callback報非輸入offender而無進展，沒有把一般檔案失敗變panic。
- Complete新增12tests＋原8tests＝Unix20項PASS。普通非Unix預期17項，未在非Unix實跑；newtests含A/B/B forcedbucket、left representative readerInterrupted→R→PermissionDenied後B/Brecover、hashfailurecontinue、carryover人工ScanIssue、observablebefore/afterlenchanged、duplicatepath、positive3/7短讀跨9001bytes、tailbyte/missingtail、空/非UTF8digest、Rightreaderror。fixtures不猜權限、不sleep、不造巨大檔；原reader完整source可見，人工ScanIssue只驗ownership接線。
- Selectedclippy建議retain時仍未教此API；採actual9.4 filter/collect的new remaining binding，再檢查數量減少後assign，沒有新增retain或lint豁免。兩stage fmt與alltargetClippy-Dwarnings PASS。
- 真repo外workingcopy以literalmanifest起點check PASS，A1 baseline8pass；同一份checks手動加入後13pass7fail。依序只補digest/refine/recover，15pass5fail→15pass5fail（failure位置轉移）→20pass，最後Clippy PASS。author 驗證script曾誤全刪同名import造成tests缺FileCause，已改stub使用完整crate路徑避免專用import殘留，重新從cleanstarter完成同一路線；最終script也明確驗stdout中的行為FAILED，沒有把編譯失敗算預期assertion失敗。
- 8個真正commands含stdout/stderr保存 `/workspace/work/ch15-a2-verification/commands.json`。真六檔demo2群／0收集問題／0內容問題：三份RustLF5bytes、一群兩空檔0bytes，different不進重複群；已確認hidden/sub原生位置都有。
- Strictscratchdocsbuild `/workspace/work/ch15-a2-render` PASS，沒有改共享nav。新增recover-content-bucket.svg intrinsic340×680，所有文字18px以上，圖片大小在desktop1280×800與mobile390×844均真正測得340×680；來源需要的text全部在SVG邊界內。原位圖文＋standalone SVG已親看，a代表讀失敗與b2/b1保留對應清楚；箭頭文字有白底，沒有線穿glyph。
- 實頁另親看必要ownership／helpercontracts／手動加tests／任務／驗收表、三層提示與答案展開；mobile page無橫向溢出，長Rustsource是codeblock 內閱讀。證據 `/workspace/work/ch15-a2-visuals/manifest.json`與22張原位capture，親看選定必要範圍，沒有聲稱所有capture、共享正式nav/darkmode、獨立首讀／發布QA已驗。
- A2完整stage約1148行含全部A1+A2tests與demo，只有本階段功能；不累加starter/checks duplicate。後續完整A需另控主線core與重疊驗收行數，不能把stage現在數量當已達完整工具scope。

15.3可交root作者前置precheck；A3仍等本段actualgate，A4再讀7.5outputflush修後actual，15.6仍等14.4/8.6actual。原review未改。


15.3實頁追加：source-visual-followup.json另存desktop／mobile各三張，確切定位fixture、material test與recovery test的真實codeblock。已親看desktop/mobile fixture與desktop recovery／mobile material必要程式，避免只看正文或最早capture的瞬間空白區域就聲稱source頁面可讀；DOM source文本確在pre中。HTTP已關閉，沒有新增Rust修改或重跑全套。


## 15.3 root integration與15.4寫前安排

Root已完整read BODY、兩stage/support/stubs/20tests、SVG source、完整artifact，一樣bytes核對與20tests/真六檔demo親跑PASS；actual ch15-confirmation-prerequisite-v3八units全artifact已凍結。本作者再讀八units；root原位圖批次與獨立releaseQA尚未完成，不混成發布通過。

A3先補讀actual7.7七units（ownedSerde與selectedJSONAPI）、6.7 Ordering／三sort／完整tie break必要units、9.4 metrics／hardlink／MAX actual，另讀修後7.5 v11 005-006的write_all＋flush支持stage示範真正輸出。新EN／ZH structureddata／官方Serde／當前serde_json及快取精確1.0.154 source重讀，記sources。以下安排已在15.4 body／source寫前保存；接續作者成果另記下方。

| 關係 | 首次需要 | actual／當下說明 | 可預測結果 |
| --- | --- | --- | --- |
| 兩種issues＋完整性 | 入口答案／Report模型 | A2保留的scan/file兩欄 | 有任一問題=>complete false；找到group不抵銷。 |
| nativecore與presentation分工 | 報告排序／模型轉換之前 | 9.3PathOrd、6.7StringOrd；本頁nativekey保存到排序完 | a/b vs a.txt不拿String順序替代Path組成順序。 |
| logical bytes 與 overflow | 第一計算例／metrichelpers | 15.1公式、9.4 TryFromIntError/checked/MAX/hardlink | 三份5bytes得10；空組0；溢位是致命建報告Err，不當partial成功。 |
| 新representation trait能力 | 第一derive／排序source之前 | 新選Serde的u64/Vec元素遞迴/OptionStringSome→value None→null；新選Option<T>:Ord | 所有欄位可編碼／讀回；None先Some，path/action/reason/debug完整ties。 |
| 路徑文字非核心identity | 第一group/diagnostic轉換前 | 7.5to_str/display、5.3Debug；本頁String Unicode group 與path_debug 診斷 | 原生path先排序；非Unicode只診斷，不能拿debug文字開檔或去重。 |
| text/JSON来自同一Report | renderer之前 | 7.7toStringPretty/fromStr、7.9Stringpush LF | schema/metric/group/issue一致；不手寫JSONescape。 |
| 控制characters字面意義 | text選擇Debug表示前 | 4.2Debug、7.7escape、7.9charpush；test以pushquote/LF而非未知escape造材料 | filename帶LF也在一行被辨認，JSON讀回字元不丟。 |
| guidedstage | 第一次Cargo設定 | A1/A2移植完整artifact＋baseline20；checks另加 | 三新待做body（build_report/order_issues/render_text）明示；JSON薄wrapper提供已教能力。 |

A3只新增report/metrics與兩stage唯一a3_text/a3_json示範，不先接完整參數或ExitCode；literalmanifest，root整合Cargo/lock/nav。A4/15.6仍逐節gate，不提前把未寫能力算此刻成果。


## 15.4 A3 完成與作者自查

- 最新actual依據為 ch15-confirmation-prerequisite-v4 全八 units；本作者再次實讀，對v3差異只有局部inlinecode，全部sources不變。7.7取ch07-review-fixed-v13七units、6.7取complete-source-v6的Ordering／三sort／tie break、9.4 content-v4 metrics與hardlinks、7.5 output-fixed-v11 005-006已實讀。14.4gate目前READY，但本段沒有借用其尚未實讀內容。
- 新body docs/chapters/15-04-build-report.md；兩stage a3-starter／a3-complete，以及checks、兩READMEs、唯一example a3_text／a3_json。Root已整合members與lock；本作者不動共享Cargo／nav／lock。四份繼承A2模組collection/content/files/stream在兩stage均逐bytes等於A2完成版，沒有隱藏A4／fullanswer。
- Body首次代碼之前先教 Option<T>:Ord 的None先Some／Some比較inner、Option與Vec的PartialEq、Serdeu64／OptionString／Vec與owned模型。native PathBuf排序key保留到丟棄前，a/b vs a.txt圖與實測揭示String排序是另一規則，診斷debug只顯示、不拿去去重或開檔。
- Clippy建議使用未教Option::as_ref；已先讀當前官方selected body、在issue-rows之前解釋借Option<T>得Option<&T>、map消耗暫時Option而非原生路徑，並分清AsRef trait。沒有默默使用或豁免lint。fmt/check/alltargets Clippy -Dwarnings通過。
- Complete有原20加新9，共Unix29tests PASS；新checks與實際編入report.rs的module逐bytes相同。其他平台預期24項，作者未在非Unix實跑。新驗收涵蓋邏輯bytes／空與singleton／nativevsString順序／完整diagnostic ties／bothissues partial／Unicode控制字元與JSON整模型往返／乘法與加法溢位／Unix非Unicode診斷與hardlink路徑。
- Repo外literalmanifest真正starter check PASS，baseline20pass；手動加入同一checks後21pass8fail，order→build→text依序22/7→27/2→29/0，明確判stdout的行為FAILED，不把compile error當故意失敗。最後completedworkingcopy Clippy PASS。固定Overflow令溢位test碰巧pass，正文先解釋不是計算成果。
- 同一驗證共9命令與stdout/stderr保存在 /workspace/work/ch15-a3-verification/commands.json。真sixfiles text前三行complete=true／10bytes／2groups，JSON完整解析後五欄相符，0bytes2paths與5bytes3paths兩群、兩問題清單空、hidden/sub都有；兩examplewrite_all後flush，只依固定教材材料expect，未假裝實作A4退出政策。
- Strict scratchdocs build /workspace/work/ch15-a3-render PASS，未動共享nav。新增native-report-order.svg intrinsic340×500，DOM在desktop1280×800與mobile390×844實測相同；所有SVG文字bbox在viewBox內。最初bounds script誤假設SVGRect有toJSON而失敗，改用x/y/width/height後真正通過，未把第一次失敗算證據。
- 已親看desktop與mobile原位圖文，手機Option借用段／手動加checks／fixturecode／驗收表／提示與答案展開。桌面必要source、接受表與提示另看下方追加；mobile整頁scrollWidth375小於390，長code在pre內閱讀。24張原位與1standalone、四折疊open狀態、圖bbox與頁面bounds存在 /workspace/work/ch15-a3-visuals/manifest.json；沒有聲稱所有自動capture、正式nav／darkmode、獨立首讀或releaseQA都驗。
- A3只是當階段快照，沒有要求重寫A1/A2成果。後續15.5必須等root實讀本段actualgate；15.6將另合併重疊測試／fixture／demo，最終約1000–1400手寫Rust incltests按完整artifact單份實數記錄，不能累加全部stage，也不能省掉fs/collision/exit合約來湊行數。


15.4 實頁追加：desktop的Option借用、手動checks/fixture、驗收表、三提示展開、固定CWD示範與預期結果已親看；mobile的最後驗收表、第三提示與示範也親看。standalone SVG親看，沒有文字出界、箭頭穿字或用String排序代替Path的對應混淆。精確 frozenA2-v4 的四份繼承source與兩stage再次逐bytes核對PASS；同一新checks與實際tests亦PASS。HTTP已關閉。


### 15.4 root precheck 最小修正

Root完整BODY/source核讀後指出兩份驗收裡未教的arrayref→slice隱式coercion與String/str交叉PartialEq。已把11個group呼叫明確寫成&array[..]，兩項paths[0]比較先.as_str()，維持整個checks module與complete實際tests逐bytes相同。affected fmt／alltargets Clippy -Dwarnings／九項reporttests PASS；原20項被filter，不宣稱這次重跑全29。沒有新增鏡像測試或重跑repo外階段路線，等待root凍結A3才開A4。


## 15.4 root integration 與 15.5 寫前安排

Root完整BODY/allsource/九reporttests/README/demos/Cargo/SVG讀完，核兩種超前簡寫修正、checks同bytes與support等A2；fixedgate ch15-report-prerequisite-v5 八units＋兩完整stage與材料。本作者已再讀全八units。Root起初--manifest-path從repoCWD跑fixture失敗，改正文固定CWD後text/JSON真正10bytes2groups完整無問題；不是演算法bug，亦未混成全releaseQA。

15.5 fresh English/Chinese來源與selectedclap前文此前已重讀，現在gate齊再寫。選已教bool --json，不加ValueEnum。

| 必要關係 | 第一次需要 | actual／本節解說 | 預期判定 |
| --- | --- | --- | --- |
| Settings只持設定 | 第一呼叫方式與derive前 | 7.6 PathBuf positional／boollong／try_parse_from全原生序列 | root可有空白；--json選相同結果的格式，不改掃描。 |
| complete與partial狀態 | 第一介面表 | A3兩種問題合成complete；本節0完整1部分2致命 | 空群完整也0；報告partial也要成功寫出才1。 |
| write＋flush都是完成邊界 | 第一writer helper前 | 7.4 Write與7.5v11；本節任何寫入/flush Err是fatal2 | write_all Ok尚未確認stdout全部輸出；flush失敗不回0或1。 |
| typedRunError保留分類 | 模型／From前 | 第5章From/?，A1RootError、A3ReportError、7.7JSONError | 環境root/數字/編碼/output失敗可交上層，沒有panic取代政策。 |
| providedmain與替代輸出 | 首次run/write_report source前 | 7.9泛型Write；7.6Error Display/use_stderr/exit_code | main僅參數/出口/狀態；help/version顯示成功0，顯示輸出失敗2。 |
| controlledwriter與child為不同證據 | 首次checks前 | 7.4implWrite／7.9writer fixture、7.8Command/Output | 自己的writer控制write或flush失敗；真正child另外測出口與數字。 |
| independentintegrationchecks與targetname | 手動加驗收前 | 7.2testtarget＋7.8env!實際binary；本頁starter/finaltarget名稱不同 | checks不是自動target；加到workingcopy tests/cli.rs才跑child。 |

A4只完成接線/main/行程驗收，不改A3core或偷放15.6公開API重整。兩packageliteralmanifest加clap4.6.7 derive，root統一member/lock；source存在即notify。


## 15.5 A4 完成與作者自查

- BODY為docs/chapters/15-05-connect-cli.md，stage為a4-starter／a4-complete，兩READMEs、兩checks、真正main、app與SVG齊。Root members／lock已整合可--locked；作者最初在整合前叫Cargo fmt/check得到notmember101，未當sourcebug／通過。共享Cargo／lock／nav未由作者改。
- 六份core collection/content/files/stream/metrics/report在兩stage再與frozenA3-v5逐bytes核同；app-tests entiremodule與complete同bytes，CLI tests只換兩種crate／binary名字後同bytes。沒有同名demo target，也沒有提前放15.6公開API重整答案。
- BODY先由A3固定root/expect的缺口引入可用CLI；先給介面表，分清complete掃描與write＋flush完成，任何OutputErr fatal2覆蓋原先0／1。新圖Report→write_all→flush→兩Ok讀complete0/1、任何Err2，arrow與兩種role在相鄰文字解說。
- Settings／clap選用能力均7.6actual；typedRunError四類以第5章From/?保持cause，RootError的innerIO Display與文字分類在首次code前解釋；7.9W:Write回收於write_output，只有兩份待做函式，main已有參數/出口/ExitCode，沒有panic處理一般檔案錯誤。
- 原A3 29tests及newapp6＋真CLI6＝Unix41 tests PASS（35lib、0main、6integration）。後續兩處len>0用既有as_slice().is_empty()替代Clippy建議的未選Vec::is_empty，兩package最終fmt/alltargetsClippy-Dwarnings PASS，affected6app+6CLI真正再跑PASS；原29source沒有變，沒有為lint重跑整套。非Unix預期34，作者未在非Unix執行。
- ControlledWriter成功接收量、write/flush failures與次數在firstfixturecode之前教，人工Report不是磁碟證據；真CLI另外fixture建六bytes資料、child原生root／CWD、兩出口／status完整核對，含partial1原生nonUnicode診斷、rootlinkfatal2、usage/help/version與emptycomplete0。
- 真repo外literalstarter先check、baseline29PASS。初版驗證script誤以Cargo default test會執行所有target而預期10fail；Cargo在library6fail後停止，已據真結果於BODY補7.8已有--test cli，再執行同一CLIchecks得2pass4fail。補write_report後33pass2fail，補run後35lib+6CLI全41PASS，不把沒跑到的CLI算通過。固定Err讓部分root-negative-status或既有help碰巧pass，BODY先明示不能算scan成果。
- 外部copy最後Clippy遇到兩個len_zero後作相同已教expression最小修正，再ClippyPASS；行為路線不因lint重跑。13真正records含failedclippy、修後clippy、affectedtests、實際repo根sixfilestextJSON與Linux/dev/full輸出故障在 /workspace/work/ch15-a4-verification/commands.json。真文字前三行完整/10bytes/2群，JSON全五欄與0byte2paths／5byte3paths解析核對；/dev/full child真回2且stderr有輸出失敗，不冒稱這個裝置probe獨自證明lateflush（lateflush由controlledwriter分別控制）。
- Strictscratchdocsbuild /workspace/work/ch15-a4-render PASS，未改共享nav；report-delivery.svg intrinsic340×570，desktop1280×800與mobile390×844實測同大小，全部textbbox在viewBox。兩平台原位圖與standalone親看，兩Ok才讀completevs任一步Err關係可辨。
- 已親看desktop/mobile工作任務/stub與所需From/Error/Write說明、checks/source、三提示與answer展開、實際command/預期結果；補看main let_契約與firstsource、childhelper相鄰材料、兩接口表。mobile page scrollWidth375<390，長code維持pre內閱讀；必要兩表在viewport內，字段無整頁擠出。25原位+8followup captures與bounds存 /workspace/work/ch15-a4-visuals/manifest.json／followup.json。沒有說全部capture、正式nav/darkmode、獨立首讀或releaseQA已通過；HTTP關閉。
- A4已能使用，但本節作業只有兩份新增接線，不累加前面所有stage copies。15.6待root讀actual本節gate後才開，需另實讀14.4與8.6；最終參考版會合併重疊fixtures／tests／demos，真單份手寫Rust含tests另記~1000–1400，保持所有有意義fs/collision/exit/flush契約，不增刪除等feature。


## 15.5 root integration 與 15.6 開寫前安排

Root全BODY＋App/Main/新12tests/README/Cargo/SVG讀完，核六core相同A3v5、appchecks同module、CLI只有cratebinarynames差異；無修法，actualch15-cli-prerequisite-v6七units/兩stage/材料。本作者已重讀全七units。作者實頁與rootsourceprecheck不是獨立完整releaseQA。

15.6先實讀actual14.4 api-v1十一units（pubuse/private/publictypedError/source/doctest/外部consumer）與8.6 whole8v8六units（Error/Display/source/sharedborrow/objectbound/finitechain）。新EN／ZH Book14.2 pubuse完整對應段落已實讀，官方Error/source與當前serde_json/Error selected能力再核。仍用literalmanifest；root只整合member/lock/nav。

| 必要關係 | 第一次需要 | actual／本文安排 | 能預測的範圍 |
| --- | --- | --- | --- |
| 合併是同一題目，不是新功能 | final入口 | A1–A4 actual完整功能；列stage→final位置 | 前段自己練習，最後reference只一份整體，不累加所有copies。 |
| privateimplementation／publicroot | lib.rs公有入口之前 | actual14.4 pubuse同型別、privatepaths與公有payload需重導出 | caller取得Report／Settings／RunError與必要payload，不依賴callbackhelpers位置。 |
| Result分类與StdError報告介面 | firstimplError前 | actual8.6及14.4；局部selectedserdeJSONError/TryFromIntError也提供Error且owned | TopRunError的source借出已有std原因；不必讓每個内部Result型別都實作Error。 |
| Display分工 | binary chain reporter之前 | actual8.6目前context與source不重複 | RunError來源為typed標準error時Display只說context；無下層時顯示完整leaf。 |
| 公有scan_report與owned結果 | firstdoctest/consumer前 | A4scan→analyze→build_report；已有Path/Result | 不需clapSettings也能借root取得Report；成功partial仍Ok，由complete讀完整性。 |
| programmercallback契約私有 | API說明／negativefixture前 | A2finite移除offender已教；14.4 private module | 真fsfailed保存issues；私有callback違約panic不是一般使用者錯誤政策。 |
| 縮減重疊驗收，保留關係 | finaltests導覽 | 各stage meaningfulcontrolledFS/collision/readchange/source/exit/flush | 每項core合同可定位，排除只是重複case或demo包裝，總handwrittenRust含fixture/tests實數記。 |

目標單份~1000–1400含必要tests，不靠minify或把測試藏另處壓假數字；若實際略超出據實記錄。A1～A4已凍結artifact不改。

## 15.6 完整參考版完成與作者自查

- BODY為docs/chapters/15-06-duplicate-finder-answer.md；完整package examples/ch15-projects/duplicate-finder，含literal Cargo.toml、README、公有入口、私有實作、兩種integration tests與六檔fixtures/duplicate-demo（hidden、sub及empty皆保存）。Root已整合member與lock並自跑alltargets check，本作者不改共享Cargo／lock／nav；A1～A4凍結artifacts亦未動。
- 寫前再讀actualA4-v6全七units、14.4-api-v1全十一units、8.6 whole8-v8第六節全六units。14.4初次輸出截掉最後三units後分別補讀，沒有把截斷輸出算完整閱讀。收到10.2 suffix-v13後亦只補讀001-005新增literal型別後綴段；final source沒有numeric suffix，不因補教改source或重跑tests。
- Fresh English／簡中Book14.2的完整Public API段已實讀，採caller路徑與檔案安排分離；本題private module依actual14.4，不宣稱刪除舊stage公有路徑保留相容性。選用Error.source、TryFromIntError及精確serde_json1.0.154 StdError能力再核，來源細節另見notes。
- Six核心collection/content/files/stream/metrics/report的演算法不改。逐bytes比對frozenA4-v6，移除snippet markers、embedded tests與舊stage加tests註解後完全相同，SHA保存core-source-provenance.json。最初normalization只除markers/tests，因content/report尚有stage註解而失敗；不是演算法差異。之後發現final collection仍保留過期A1加測試註解，已移除，最後六份真正相同；沒有把早先失敗當通過。
- 公有root重導出同一RunError／Settings／Report與payload名稱；scan_report是沿A4 scan→analyze→build_report的薄入口，partial仍Ok owned Report，不選格式或stdout。只有頂層RunError實作StdError，source借原有標準cause、跳過沒有Error介面的分類wrapper；Display當前context／無source leaf、sharedborrow與staticobjectbound在首次impl前解釋。main沿actual8.6有限source鏈借讀，不暴露私有callback供caller接錯。
- Root早讀app提出Count標準cause被Display與source重複的推測。作者再核實際ReportError::Metric(Count(_)) Display是固定「路徑數無法表示為u64」，原generic Report並未印出標準TryFromIntError原因；root已承認原主張未成立／作者核對否證。專用Count context仍採用為可選清晰改善，不能記成必要bugfix或失敗證據。未為平台自然不可觸發的usize→u64 Count硬加無關i32 constructor測試；source選取與已教共同報告器核讀支持當前分工。
- 真實單份physical Rust行數：核心995、tests與fixture helpers662、合計1657，含空行／註解／doctest，排除locks/build/generated與四stage copies。Root確認1000～1400只是早期estimate、使用者約1000不用精準；保留約千行核心及必要驗收，不minify、不藏tests、不刪邊界coverage。BODY與README均據實區分，讀者不一次重寫全部參考tests。
- 重疊fixtures/tests合併為8核心＋7CLI＋1外部公有API＋1doctest，共Unix17 PASS。仍保留forced bucket多真群、重複nativepath、精確left/right failedread、Interrupted後失敗→移除A保留B/B、3/7正短讀跨buffer、非UTF8／empty／taildifferent／missingtail、beforeopen與afterread變動、singleton免hash、scan與file两種partial、完整ties、native順序、控制字元JSON往返、兩種overflow、write與flush分別Err。真child保留hidden/empty/Unicode、empty/missing/file/rootlink、nonUnicodepartial1、usage/help/version、inner links忽略及hardlinklogical bytes3。非Unix預期14，未在非Unix實跑。
- 測試新Source借仍有效的bytes，BODY在artifact導覽前解釋4.5輸入lifetime／width正短讀／Interrupted與fail_at責任；用已教fixed array、tuple reference的field access、match借Option／解reference、顯式Path view，避免未知vec重複／letchain／Option Copy假設／arrayref coercion。首輪Clippy兩建議改回已教match與array repetition後真正通過，未allow lint或默默加入新語法。
- fmt、alltargetsClippy-Dwarnings、17tests、release與cargo doc PASS。repo外完整literalmanifest先建立自己的lock、同17tests與Clippy/release PASS；最初outsidecopy早於三項helper選用契約微調，已重拷當下src/tests/fixtures/README再跑17tests/Clippy/release，逐bytes核每個package檔完全相同。最後只移除過期A1註解與更新行數，無Rust tokens/tests變動；再核fmt、精確outsidecopy與strictdocbuild，依規則未為純註解再跑同套tests。
- 驗證13commands、stdout/stderr、真正6檔text/JSON10bytes2groups/complete以及Linux/dev/full輸出fatal2診斷保存在 /workspace/work/ch15-a-final-verification/commands.json。/dev/full probe支持真出口2，不獨自證明lateflush；controlledwriter明確令flush Err，覆蓋原0／1。summary.json有最新行數／13records／exactcopy；單份fixture全部bytes和原材料一致，公開doctest與API consumer不是零tests。
- Strict scratchdocs build /workspace/work/ch15-a-final-render PASS；final不重画已教演算法，只用責任對應與有意義coverage兩表。桌面1280×800與手機390×844原位公有入口、scan/doctest、Error.source／reporter、callback契約、reader fixture、表格與commands均親看指定screenshots；手机scrollWidth375<390、表寬343，code以pre自身水平閱讀，沒有整頁擠出。原code capture有一次lazypaint尚灰，等待render後重拍並親看，未把灰屏算可讀。正式nav/darkmode、獨立首讀與全releaseQA仍未由作者聲稱完成。
- 生成Rustdoc的root公有API另用實際HTTP頁核對／親看，不把HTML檔存在當視覺；初次file URL受browser administrator阻擋，改本機HTTP，未宣稱被阻擋頁驗過。實頁capture與bounds／pub API DOM在 /workspace/work/ch15-a-final-visuals/manifest.json及followup.json。最後HTTP兩server停止後交root完整BODY/source，不預先宣稱root actual gate已凍結。
