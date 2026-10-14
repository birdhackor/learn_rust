# 第 9.2–9.4 節作者先備與驗證

作者：chapter02_ownership。這是作者自查，不是獨立首讀或發布驗收。原第 9.1 作者 ledger 保留。

## 9.2 必要概念安排

| 概念／契約 | 首次需要 | 實際先備／本文解釋 | 能判斷的結果 |
| --- | --- | --- | --- |
| host、target、cfg! | 分辨 script 與主要 crate | actual ch09-tooling-v1 全文；本節階段圖與 host 小節 | script 的 cfg! 不能冒充主要 crate 的 target |
| build.rs／build 欄位 | manifest | 本節先教 package 根路徑、script compile/run/crate compile 順序 | build.rs main 與 binary main 不是同一次執行 |
| env::var／VarError | script 輸入 | 本節執行時 Result、String／OsString variants 與政策；actual 7.5 OS 字串、5.1 expect | unset 可預設，NotUnicode 與非法文字都拒絕 |
| cargo:: 指令／change detection | script stdout | 全部四項在 source 前表格解釋 | runtime 直接執行不會 rerun；cargo::error 雖 normal main 回傳仍失敗 |
| env!／&'static str | library | actual 7.8 七 units、8.4 static 四 units；本節再次區分編譯／執行階段 | runtime 環境不能改已固定文字 |
| OUT_DIR／build-dependencies | 生成檔／依賴提問 | 本節先述各自角色，無未教 include!/C/cc code | 生成檔位置與 script／主要 crate 依賴分開 |
| CARGO_PKG_NAME／format! | 練習答案 | 本節題目先教 Cargo automatic variable；format! actual 5.3 | 不需為既有 Cargo 值另寫協定 |

source：`examples/ch09-build-script`，獨立於 9.1 package；root 已整合 member 與 lock。normal main、library、build.rs、integration tests 與完整 example 答案逐項核對同一限制；沒有後節檔案 API、unsafe environment setter、Command::env 或未教 grouped use。

實際驗證（2026-10-14）：`cargo fmt -p ch09-build-script`、`cargo clippy --offline --locked -p ch09-build-script --all-targets -- -D warnings`、`cargo test --offline --locked -p ch09-build-script` 通過，Linux integration tests 2 項。demo／practice 的 cargo run 分別印相應文字；最後 practice 成品以 `LESSON_MODE=demo ./target/debug/ch09-build-script` 啟動仍印 practice；完整答案印 `package=ch09-build-script; mode=practice; target=linux`。

負向驗證：wrong 文字及 Unix 非 Unicode byte 0xff 各由 build script 拒絕、建置非零；demo 與 unset 都成功。實際 stdout／stderr／exit code 存 `work/ch09-author02-research/build-checks.json`。這是 build-fail，不是 Rust compile-fail，不佯裝成 E-code fixture。先前 clippy 的 collapsible_if 已改用 actual 1.6 已教的 `&&`，再跑通過。

SVG `build-phases.svg` 有 intrinsic 340×570、24px 字、title／desc、四步相鄰正文。**目前未親看實際桌面／手機頁面，visual 待 root 原位檢查。**沒有執行跨平台建置或 Windows 執行，不以 host/target 範例證明那些平台已驗證。

## 9.3 必要概念安排

| 概念／契約 | 首次需要 | 實際先備／本文解釋 | 能判斷的結果 |
| --- | --- | --- | --- |
| symbolic link／root 政策 | metadata root 判定 | 本節位置表、最後項目 symlink_metadata 契約 | root link 拒絕，內部 link 政策略過；!is_dir 不等於一般檔案 |
| Metadata／FileType／len／modified | records/observe | 本節表格在 source 前；u64 與 SystemTime 選用能力先教 | bytes 與 usize 角色不同，modified 失敗不能假裝完整觀察 |
| WalkDir／Result Item／兩個 follow 開關 | scan | actual6.5逐筆 ownership；本節表格與 root 檢查 | Err 保存並繼續，兩開關對不同位置，無原子／快照保證 |
| DirEntry／Error.path | source ownership | 本節 path／into_path／Option 表格，actual7.8 to_path_buf | borrowed path 使用後才移走 entry/error；未知路徑留下 None |
| Option.is_none/map | Unicode 與 optional error path | 本節各最低契約，map FnOnce；actual6 closure | 不把 optional path 轉換誤當丟 walker error |
| native PathBuf Eq/Hash/Ord/Borrow<Path> | sorting／練習map | actual4.3/6.7/7.5；本節特定型別契約 | display 不作身分；borrowed Path 查 owned key，固定平台排序 |
| partial move | main／答案 | 在 main source 前教部分欄位與整份值的差別 | move files/path 不妨礙互不重疊 issues/snapshot，不能交出整份不完整值 |
| fixture create_dir／Unix symlink／OsStringExt | tests | 本節測試前解釋新 API；actual7.8 TempDir/fs.write | 受控 link/非Unicode材料與條件平台範圍清楚 |

source：`examples/ch09-files`（root 已整合 member/lock）；9.3 的 scan module、main、native key 答案及 tests/scan.rs 全部有限元件，沒有提前寫9.4或15A完整工具。個別原因保留具體 io::Error/walkdir::Error，未用 dyn Error 或 Box。

實際驗證：fmt、all-targets clippy -D warnings 通過；Linux/Unix八項 tests/scan.rs 通過（包含可控制 Ok/Err/Ok 仍收齊兩成功一失敗、兩種link位置、非Unicode原生資料保留、空檔/目錄及root失敗）。正常 main 列三份教材檔的6/5/5 bytes與掃描完整=true；完整答案以原生 borrowed Path 查到Some(&5)。先前 manual_map lint 改用本節 source 前新增完整 Option::map 最低契約後重跑；不是未解釋地照 lint 塞入新 API。

9.3沒有新增 SVG，政策位置與逐筆資料在相鄰表格和source表達。目前未親看其真實頁面，visual 待root原位檢查；沒有平台原生權限／Windows link 檢查宣稱。

## 9.4

root 釋出 `ch09-files-prerequisite-v3` 後，已實讀其七 units，再寫本節；獨立 package `examples/ch09-content` 依賴 `ch09-files`／blake3 1.8.7（由root整合member/lock），不修改9.3原manifest／source。

| 概念／契約 | 首次需要 | 實際先備／本文解釋 | 能判斷的結果 |
| --- | --- | --- | --- |
| 大小→digest→bytes分工 | 入口與size example | 本節表格及5/5/6實際材料；actual6.7、9.3 | 各步回答不同判準，大小單一桶省去內容讀取，不承諾測量倍數 |
| BLAKE3／Hasher／Hash | hash_reader | 本節全部selectedAPI與回傳角色先表格；英中Cookbook與正式BLAKE3已讀 | update按序串接有效bytes，finalize不消耗，digest同只是候選；同名Hash型別/trait分開 |
| short read normalization | equal_readers | actual7.4全文；本節fill_chunk＋4格示意／4096實作 | 左右positive short片段不必同步，填滿或EOF之後才比較同一位置 |
| byte slice PartialEq／CompareReadError | 比較source | 本節長度/逐格判準與Left/Right原始ioError角色 | 空/尾差/真正讀取失敗不混成普通false |
| controlled readers／assert! | main/tests | actual7.4原reader/error事件；本節構造契約與assert!bool政策 | 3+2／逐byte重接、Interrupted後仍保留PermissionDenied |
| File.metadata／前後觀察／FileIssue | 真檔wrapper | 本節handle/path來源、typedcause與offender前置說明；actual9.3 Snapshot | hash與compare讀取前後都查；false結果也postcheck；非一致snapshot/安全替換保證 |
| tuple key／DigestBuckets | hash_candidates | 本節Eq/Hash每欄關係與alias；actual5.2／4.3 | 確切(u64,digest)作candidatekey，partialmap不作完整成功 |
| true groups／native seen key／borrowed members | refine_bucket | A/B/B相鄰SVG與逐組代表推理；actual4.4/4.5單一來源、6.7sortcallback | fake同digest分1/2組；duplicate path不加倍；來源仍需有效 |
| singleton驗證／fail-fast復原邊界 | refine source及後文 | source前說新組完整hash讀取原因；source後FileIssue原生path政策 | FileIssue指出代表或candidate確切offender，caller移除後有限次重試，不丟整桶有效候選、不當false |
| checked算術／path-based bytes／hard link | metrics/答案/tests | 本節公式、u64::try_from、首次std::num::TryFromIntError／Debug/use、checked_mul/add／MAX、hard_link契約 | 0/1 path與空內容為0，溢位Err；hardlink不同path仍算，不等於可回收空間 |

逐API檢查normal library/main、四份正常／完整解答examples、streams與files_groups全部tests同一先備。無新Box/dyn/closure轉錯省略、Cow/lossy identity、平台time轉秒、memorymap／rayon／thread API。source以snippets引用，metadata helper、事件reader與答案也在當節先教。

作者實際驗證（2026-10-14）：fmt及offline/locked all-targets clippy -D warnings通過；`cargo test --offline --locked -p ch09-content --all-targets` 共18項通過（streams6；files/groups12）。正常main、size_candidates、file_confirmation、candidate_digests、collision_solution全實跑。digest分批/一次相等；5-byte真實候選桶2；真內容true/false；forced same digest A/B/B得到1/2兩組及按path3 bytes。TempDir隨機前綴只作位置，不當固定輸出。

重要負向／邊界：9001bytes跨buffer與不相同正shortread上限；empty/nonUTF8；末byte改變／少1byte；左右Interrupted→PermissionDenied；hash前/compare右方可觀察變動；handle打開後受控write再postcheck；missingcandidate明確offender；移除後其餘A/B/B成功重建；真hashcandidate partialmap不回Ok；相同原生path重複輸入不加倍；群組順序不依到達順序；u64乘/加溢位；Unix hardlink兩路徑。不是Rust compile-fail，所以沒有虛構E-code fixture。修改讀取後觀察helper的test只驗控制間隙，不宣稱真實並行每次read中變更。

兩張SVG有intrinsic340×455/480、font24、title/desc並相鄰正文。**尚未親看實際1280／390原位頁面；visual待檢。**未跑Windows、未量效能、未交付15A完整CLI；本節不宣稱獨立review或完整發布驗收。

### root source precheck 修後局部證據

root 親讀全文/source 並跑18項之後提出三項精確修正，全部採納：

1. 四格fill示意的左批次改成ABC|D|E（剩一格的read不能交DE），相鄰正文和SVG desc均解釋原因。4096實作不改，開場一次read3對1的動機保留。
2. tests第二份9001-byte array直接同值初始化，再改末byte；不依賴前文尚未教的array條件Copy。這是作者原先漏核的先備，不佯稱先前已教。
3. metrics source前補首次std::num::TryFromIntError的來源、整數轉換失敗意義、use、Debug能力。實際5.4只教自訂Count conversion，不能當這個標準錯誤的先備。

修後只重末byte/少1byte的同一測試（1項PASS）及fmt/alltargets clippy -D warnings（PASS）。沒有裝飾性重跑全部examples／tests；前述18項為修前全套及root全套證據，局部變更另明記。

### 作者原位頁面檢查（有限範圍）

隔離Zensical preview使用同份repo主題/Markdown extensions，複製9.2–9.4正文與assets、snippet直接讀repo source。嚴格build「No issues found」；不修改共享zensical.toml或root站點。Chromium/Playwright於127.0.0.1:8779真正開頁，viewport1280×1000／390×1000，先截圖再以view_image親看八張修後v2圖。

已實看：9.2 build四階段圖及相鄰前後正文；9.4修後shortread四格圖及剩一格說明；A/B/B候選桶與兩實際內容組、逐組代表說明；9.3 native path/借用key段落與source開頭。三張SVG於桌面/手機均保持340px原寬、24px可讀、箭頭框線字形完整，沒有縮成過小字或橫向截斷。9.3正文inline術語可換行，手機code以原頁面的橫向scroll呈現。

證據：`work/ch09-author02-research/*-{1280,390}-v2.png`、`visual-metadata-v2.json`、`visual-source-v2.json`。初次script於9.3 heading locator重複時中止，只留三張原desktop截圖，未聲稱mobile或全部完成；原圖沒覆寫，修後另存v2。這是作者同主題preview的局部原位QA，不等於root完整nav/release頁面、全正文/表格/折疊答案或獨立首讀驗收。上述visual待檢記錄是當時狀態，後續只在本欄追加真實完成範圍。
