# 第15章 A 篇 ORIGINAL 銜接初判：15.1–15.6 / v11

本次完整閱讀指定六頁、42 個 BODY 單位及 101 份封存附件，未確認需要阻擋目前 A 篇學習路線的必要問題。記錄一項可選的責任名稱修正 I1。本判斷只涵蓋 A 篇原始 v11，不能作為第15章全章、B 篇、執行測試或發布驗收的通過紀錄。

## 角色、版本與閱讀界線

- 角色：非第15章作者的獨立 CONTINUITY reviewer。我曾撰寫4.1–4.3、14.1–14.3及14.6，也曾讀過前章實際教材；這是審閱背景，不是本次讀者的已教依據，亦不稱新盲讀／基本 Python 讀者的 FIRST 紀錄。
- 唯一新教材：`/workspace/work/tutorial-implementation/frozen/ch15-a-for-review-v11`。六頁是15.1導覽、15.2 A1、15.3 A2、15.4 A3、15.5 A4、15.6完整參考版；未開15.7–15.11或 B 專案。
- manifest SHA：`b0943a259e99dc064a7789964b5b21152ea50e632d6a37c98f29bde0e93c8c62`。
- 實際42 BODY＋101 source byte 對映清單 SHA：`dc145bc52d73e5aa1c30d499f058b4c48f7fed21bf19c988b5cb5ce1beadd0d1`。演算法是按 manifest BODY 順序列 `BODY/相對檔名<TAB>sha`，再按原 source path 排序列 `SOURCE/原路徑<TAB>sha`，每列 LF；清單保存在自己的 `student-files.sha256-list`。此指紋包含全部 Rust、manifest、README、材料、checks、答案及三 SVG，不只是抽查檔案。
- 開始前全文讀取 AGENTS 與 clear-tutorial 的 SKILL、review-protocol、rust-terminology、calibration。原需／機制／用途分別判斷，不把術語或標題出現算作教過。
- 先依正文順序讀42單位，再核完整來源與必要前文。閉合提示及答案亦全文補讀；因此這不是一路不開提示的 FIRST 測試。必要能力以主文或真正前文定位，不以後讀答案補成原本已教。
- 初判封存前未讀第15章作者 notes、ledger、handoff、Root gate／outcome、TECH／FIRST 結果或同行問題；未以這些材料引導分級。
- 未修改教材、共享設定、Git index 或 staging；未執行 Rust、作者套件、故意失敗 fixtures 或公開 HTTP。下列測試判斷指完整來源的判準與銜接，並非自己重跑後的 PASS。

## 全文覆蓋與原始推論保存

42個單位均在自己 `scope-and-role.json` 記錄。101份 archives 對應63組完整檔案 SHA：每組先真正全文讀一次，再以完整檔案的相同 SHA 對映副本，沒有以同名函式、同樣節錄或抽樣取代全文。63組首讀 source bytes 合計209265，全部 `full_read=true`；空 binary 材料也核實為零 bytes。逐原路徑的代表檔、SHA與先讀組別保存在 `source-full-read-coverage.json`，63組清單在 `unique-sources.json`。

覆蓋四階段全部 starter／complete 的 literal manifests、lib/module、stub、已提供 helpers、完整 tests／checks、examples／main、README；亦完整覆蓋最終 duplicate-finder 的 src、CLI／public API tests、README及六檔材料。三張 SVG source 與原位 BODY 均已讀。改 package 名字後的 A4 main／CLI tests 因完整 bytes 不同，另外全文讀，沒有當成同 SHA 副本。

早期筆記保存於 `early-body-notes.md`、`early-source-notes.md`，保留當時未查／未讀範圍及之後解除候選的時點。曾暫記 `Option<PathBuf>` 比較、NotFound、String 上 contains／lines／fold 的先備待查；實讀前文後解除，未把待查問題算必要缺口。I1 的責任名稱候選在15.2正文初讀時已記，並非後見同行提示。

## 實際先備與首次需要

本次完整回讀9.3、9.4正文；其重用的檔案／stream contracts，再與 A 封存的完整 helpers／tests核對。其餘前文只回讀實際需要的段落，記錄 source SHA 與局部範圍於 `actual-prerequisite-read-proof.json`，不宣稱重讀1–10全章或所有前章 sources。

| 首次需要／範圍 | 已教依據及最短推論 | 結論與界線 |
| --- | --- | --- |
| A1：三檔 library、`pub mod`、`crate::`、同檔 `cfg(test)` module | 1.9：「單純把一份 .rs 放在資料夾中……不會讓它自動加入」；1.10：「只有在測試建置時才編譯這個 module」。15.2 002-003／004另明說 lib 宣告與整份 checks module 的加入方式。 | 能從 starter 操作出真正驗收；不把 checks 目錄本身算 Cargo test target。 |
| A1：literal dependency 要求、dev-dependencies、自己的 lock | 7.1：版本字串是相容範圍、lock 保存實際解析；15.1 001-004先教 `+1.99.0` 只選本次工具鏈，15.2先 check解析，再 locked test。 | 不把 repo 外的副本假定繼承 repo toolchain／workspace／lock。未自行下載、build這些副本。 |
| A1：Snapshot／SystemTime、Metadata、DirEntry、WalkDir error、native Path能力、Unix fixtures | 9.3逐項給接收者、結果、能力及政策：時間用Debug／Clone／Copy／PartialEq，Path按原生位置提供Eq／Hash／Ord與Borrow，walkdir error有Debug／Display；觀察 root與處理後續 item失敗分開。 | 不是只引用9.3標題；對照 A1 provided models及八項完整 collection checks。 |
| A1 checks：`Option<PathBuf>` 的 assert_eq | 4.3：「若內部型別可以比較，Option也可以比較」並分Some／None；9.3的Path比較能力＋4.2 derive／Debug即可支持本題。7.3另明教ErrorKind::NotFound。 | 解除原先待查，不要求等15.4才使用 Option比較，也不為已教能力重開一章。 |
| A2：blake3 Hash、tuple key、Read短讀／Interrupted、File觀察、左右錯誤、callback FnMut | 9.4已選 Hash 的具體能力、大小＋摘要tuple分桶、按完整 bytes確定同群、來源失敗保留哪一側，helpers在封存來源可完整讀。15.3 003-003新增移除確切原生 offender→重建與有限進展。 | 新政策首次實作前在主文解釋；false與Err不混同；不聲稱 digest相同就是內容相同。 |
| A2：借用 ContentGroup→owned DuplicateGroup、重新借同一 FnMut | 4.5借用模型不延長來源；6.3重複 mutable使用 FnMut；15.3 003-004／006在題目前分開 borrowed members與owned path結果、callback借用與每次move。 | source中的temporary closure、filter／collect等用既教能力；不是答案先給才能知道該留下什麼。 |
| A3：Ord／cmp、stable sort、Vec Debug／PartialEq、String／str methods | 6.7說Ordering三variant、Ord／文字順序與sort callback；6.6逐元素Vec能力、FromIterator和fold；8.3說String有`Deref<Target = str>`且method查找沿Deref；6.7有lines，7.9有contains。 | 完整 report／CLI tests的比較與文字檢查有來源。15.4當地另教Option Ord與as_ref，未偷用未教複合能力。 |
| A3：JSON／Serde選用模型能力與錯誤 | 7.7教derive、deny_unknown_fields、from_str／to_string_pretty；15.4 004-004補本次u64／bool／String／Vec／Option與null的契約。 | 新增 Option欄位表示不靠7.7「JSON還有null」的名詞一句；先教能力再展示derive。 |
| A4：clap Parser／attributes、原生 root、help／version、ExitCode與Write | 7.6具體教Parser、try_parse_from完整arguments、clap Error的Display／exit_code／use_stderr；7.5分Stdout緩衝與flush，7.4教Write／write_all／flush與不能撤回部分bytes。15.5 005-002先固定本題0／1／2。 | 不把前工具的狀態政策自動套本題；完成掃描與成功交付分開。 |
| A4：TempDir／Command／Output／env CARGO_BIN_EXE | 7.8先教TempDir owner、current_dir、單筆arg、output外層Result vs status、CARGO_BIN_EXE限定當前package的integration target。15.5 005-005再次定位兩份checks加入位置與starter／complete不同binary名字。 | 全部child helpers／checks能沿前文理解；需要改名字處已在用前解釋，不是測dependency binary。 |
| final：private module＋pub use、非窮舉錯誤、Error.source | 14.4實讀：「Options仍是同一個型別」「外部也沒有因此取得……私有路徑」及enum non_exhaustive的crate內／外區別；8.6明說source外reference借self、dyn原因型別符合static。15.6 006-003／004先回收目的，再教本次三種標準原因能力與wrapper分工。 | 能整理公有入口而不暗示刪舊pub module仍相容；沒有把source回傳值誤教成永久reference。 |
| final fixtures與有限證據 | 4.5借用模型placeholder、8.4 literal static；15.6 006-005在source前說Source借bytes、正width／Interrupted／fail位置。14.5實讀oracle與有限合法域：測試「不保證有限測試看過全部輸入」。 | A用可控制reader／writer和固定tests，沒有未教proptest語法，也沒有以通過數當所有檔案變動或平臺的證明。 |

沒有靠本次14作者背景來補「讀者應會」；14.4的必要正文已實際回讀。14.5只核本題沿用的判準／證據界線，A內並未引入property library呼叫。未要求讀者為無用細節先讀整份標準 API。

## 分階段：原需、機制、用途與限定

| 單位與最短原句 | 原需→機制→能用來判斷什麼 | 完整來源交叉與限制 |
| --- | --- | --- |
| 001-003：「大小相同只是值得進一步檢查，尚未回答內容相同」 | Rust＋LF三份、rust＋LF一份同長但不同；空檔成另一群。先從真材料決定同bytes、多path、按路徑計重複bytes，再選階段工作。 | 六份真材料bytes／LF／空檔均核；10是路徑指標，不保證物理釋放、不提供刪除。 |
| 002-002／003／004：「collect_entries決定存記錄或問題」「checks不是Cargo自動辨認的tests目錄」 | 不能開始的root與逐item問題分流，完整scan後交files／issues；自己保存三起始檔，再附tests做第一個可判斷里程碑。 | 完整A1 starter三stub、完整collection模型／Display、checks八項與complete都讀。固定Err／空result能讓某負面case恰好過，主文明說不算已完成。受控Ok／Err／Ok驗收繼續收集，並非讓正式scan接受file root。 |
| 003-002／003：「若把這個Err當成『不同內容』……若整桶放棄……漏掉B／B」 | size濾候選，digest濾桶，byte判真群；失敗保exact path，移除後重建整桶，使已處理與後續有效記錄可再形成B／B。每輪候選數減少才有限。 | A2三待做函式、provided analyze、files／stream、十二新checks與complete全讀。人工同digest是受控桶，不偽稱找到真碰撞；singleton不hash，所以不聲稱讀完每檔。違約offender不在桶的panic與一般FileIssue分開。 |
| 004-002／003／004：「原生排序完才轉成文字」 | 一份owned Report供text／JSON；先按Path components排，再轉文字；diagnostic原生key＋各顯示欄位全比較，避免stable sort保留偶然到達順序。 | 全A3 report、metrics、九新checks／examples／完整JSON介面已讀。None排序及Option as_ref當地先教；path_debug只供顯示。模型Overflow是合法受控負面，不用真大檔假製造數字。固定平臺Path Ord，不承諾跨OS同序。 |
| 005-002／004／005：「兩步都成功才選0／1，任何一步Err都是2」 | 先約定caller的資料／診斷／狀態，再接write_all→flush→complete分類。writer的成功與Report.complete分別回答交付與掃描。 | 全A4 main、兩待做函式、provided Write helpers、六app／六CLI checks全讀。ControlledWriter不是實際buffer：記錄真正write／flush呼叫、各自可控拒絕；child驗stdout／stderr／status，不拿library return冒CLI。 |
| 006-003／004／005：「caller……不需要知道……callback放在哪個檔案」「這是另一個完整參考package的介面」 | 新完整版把A4收成可查閱的公有入口；scan_report只是薄三步，不迫caller學clap。borrow標準cause與typed外分類並存、Display context與source理由各一層。 | final七私有module、root reexports、Error／main explain、Source／八core／七CLI／一public API test及一doctest全讀。callback private限制外caller替換，不把一般讀失敗變panic；自身原因鏈無迴圈才有限，不泛稱任意Error鏈一定有限。 |

用途判斷不止「介紹這個API」。各階段都能指出不這樣分工時會錯在哪個可觀察需求：root拒絕與item繼續、漏掉B／B、用文字key改原生順序、同位置診斷不穩、flush失敗仍假成功、caller被迫依賴內部路徑。測試對應這些關係，不只是鏡照程式內部呼叫順序。

## 引導、提示、答案與累積負擔

15.1 001-004把工作版、完成快照、最後參考版分開；自己的副本不被答案覆寫。四個starter的待做範圍均明確：A1三函式、A2三函式、A3三函式、A4兩函式。前階段成果、重用低層helpers、測試材料與新增責任各自定位。imports／模型／Display與低層read工作已提供，但其能力仍有前文契約，沒有把「已提供」當免解釋藉口。

三層hint先問資料去哪、再回收真正已教的能力、最後只拆一個分支。新增需求與必要型別能力已在閉合hint之外：A2 offender／有限進展，A3 Option Ord／as_ref與Serde複合能力，A4雙輸出成功與狀態，並非必須打開答案才知道題目。完整答案另有定位標籤與可用package；各階段實際source不帶未來能力回灌到前段。

新增tests位置與平臺邊界清楚：A1預期8、A2累計20、A3累計29、A4累計41皆是Unix下的來源預期，其他平臺會少指定Unixfixtures；不是我執行所得通過數。A4一般cargo test可能在library失敗後尚未跑CLI，所以另外明選`--test cli`。最終17是整合後8core＋7CLI＋1external API＋1doctest，不把41與17混成有遺失驗收。每一預期都有獨立材料與需求，負面stub恰好綠燈不是完成證據。

最終完整來源靜態逐行計數確認core995、tests／fixture662，共1657；包含註解／空行／文件中的程式，沒有計四份stage副本。15.1的800–1200是含tests的負擔目標，因此最終總量高於該估算；15.6如實揭露，核心約1000，且低層helpers和驗收已提供。此處不作必要問題或要求為湊行數刪檢查，也不聲稱目標總量已精準達成。實際新作業以少數待做函式逐階段負擔，A2重建是最重一段，已有有限小材料、SVG與分層提示支持。

最終README給複製哪些檔案、先check產lock再locked build／test、真材料與任意ROOT用法，並明確將保存的輸出放ROOT之外。正文在命令前區分repo根操作與自己的CWD；`--manifest-path`不是幫程式改CWD。沒有把這份命令、文件查閱或工具回傳值當成我自己已完成build／run。

## 發現與分級

### 必要：0項（限已查 A v11）

未確認先教後用、題目可操作性、主要方法理由或圖文銜接的必要缺口。這不是無條件證明所有 API／平臺／外部文件／網站已驗證；未查與未執行界線見下節。

### I1 可選：validate_root責任 bullet的成功主詞

- 原位置：15.2 `002-003.md`，三待做函式責任第一bullet；原頁 `docs/chapters/15-02-collect-files.md` 的「把工作版放進自己的資料夾」。
- 原句：「`validate_root`：觀察 root 本身，拒絕不存在、link 與非目錄；**scan 成功後才開始走訪**。」
- 已教部分：同單位稍後完整scan片段先`validate_root(root)?`，才建立WalkDir、最後`Ok(collect_entries(walker))`。9.3亦分先驗root與後走訪。root驗證政策和實際接線都已明確。
- 缺失關係：這個責任摘要把「驗證成功」寫成整個scan成功，字面上倒置scan完成與走訪的先後。
- 實際影響：讀摘要可能暫把「scan成功」讀成前置狀態；隨後接線足以消除，不會阻止依正文寫出正確流程，所以不升必要。
- 最小修法：只換成「**驗證成功後才開始走訪**」或「validate_root成功後才開始走訪」，不增加API、段落或先備負擔。
- 發現時點：獨立15.2 BODY初讀已記early notes；完整來源及前文核對後維持可選。未取同行／作者／Root提示。

## 親看實頁與視覺證據

自己以指定snapshot的42 BODY重組六頁；只轉換封存資產絕對路徑為預覽相對路徑，三 SVG從同份source archive複製。自己的預覽配置沒有B nav或其他未授權教材。使用專案同一CSS及完整相同theme／Markdown設定方向，未啟用navigation.top；strict預覽建立退出0，log為`preview/build.log`。

專案與實際HTTP載入的tutorial.css SHA均為 `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`；HTTP200、六頁與所有capture的`.md-top`均0，capture中實際stylesheet URL包括自己的tutorial.css。不拿另一套CSS或只有config文字冒稱實載。

以Chromium在桌面1280×900／手機390×844各選16位置，共32PNG，全部經`tools.view_image`親看，逐張SHA／尺寸／selector／fold state記`visual-capture-proof.json`。own工作目錄為 `/workspace/work/tutorial-implementation/review-ch15-continuity-a-v11-author04/`。

| 親看範圍 | 實際判斷 |
| --- | --- |
| 三SVG：recover、native-order、delivery；各desktop／mobile | 340px原生寬在手機保留字體與完整箭頭；失敗a與留下b1／b2能追蹤、PathBuf→已排原生→String有型別標記、write／flush兩Ok與任一Err分支明確。圖片不是主文外隱藏必需資料，正文亦解釋箭頭與結果界線。 |
| 15.1六材料表；A1 starter轉入與附checks說明；各兩尺寸 | 真材料、hidden／empty與路徑清楚；自己的副本與既成快照不同，checks非autotarget與同檔test加入位置可讀。手機表格在article寬度內，不需猜缺失欄位。 |
| A1–A4三層hint閉合；A2第二hint展開；各兩尺寸 | 初始皆閉合，summary標明層次；展開後必要泛型字樣如`HashMap<PathBuf, ()>`完整，不被HTML吞參數。hint只回收／拆分工作，不用藏起的內容補題目契約。 |
| A4雙驗收入口、CARGO_BIN_EXE改名／兩命令；各兩尺寸 | app checks同檔／CLI checks放tests、外binary name與固定檢查範圍連在一起；兩命令清楚，手機的長code在自己code block捲動而非整頁溢出。 |
| A4答案閉合與真正點summary展開；final Error過渡；各兩尺寸 | 答案是選用入口且兩份接線可定位；final新增公有報告能力在source之前有context、標準cause與self／static限制。展開答案親看包括已教程式碼，不聲稱全份長code同一畫面全部可見。 |

32個畫面的文件層級horizontal overflow皆false；手機長Rust／命令仍可在code區橫向捲動，未視為內容遺失。本次沒有操作全網站搜尋／所有links／深色模式，亦沒有逐像素驗收整頁每一段或GitHub Pages正式部署。僅能對上述固定稿原位、預設淺色、兩尺寸提出親看結論，不拿bbox、作者圖或他人的view紀錄代替自己的眼見。

## 保留未知與封存

沒有重跑Rust、Cargo dependencies解析、CLI child、平臺Unix／nonUnix tests、doctest或故意失敗案例。沒有公共HTTP／其他教材來源的新驗收，沒有驗證任意檔案競態／權限環境／超大目錄效能。作業所用source、答案、README與材料已全文核閱；實際執行能力與發布由獨立技術／Root檢查承擔，本判斷不補簽。

本報告是 A 原始銜接初判，保存相同bytes initial副本與SHA。封存後停止，不讀同行／處置／作者補記或B；將來B或cross另scope／另報告，原初判與早期未知不回填。
