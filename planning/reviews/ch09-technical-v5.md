# 第 9 章獨立技術初判：原始 v5

本報告是 `chapter10_concurrency` 的 **nonwriter TECH9 ORIGINAL**，日期 2026-10-14 UTC。目標讀者依使用者約定：懂基本 Python，依教材順序學 Rust；好懂好學優先，所有範例、helper、完整答案、測試、fixture、manifest 與指令只能使用當時實際已教的內容。

**此角色不是盲讀初學者。** 我未撰寫第 9 章，但此前已撰寫第 10 章，讀過它所需的 actual prerequisites，也做過第 1、2、4 章技術審閱。這些 Rust 知識是審閱者背景，不自動算目標讀者的先備。本輪用實際正文重建首次使用位置，沒有以課綱白名單、程式跑通或專家常識替代介紹時序。

本輪初判：**未確認必要技術／首次使用待修項；一項可選實頁呈現改善 I1。** 此結論限下面已讀、查證與親看範圍，不表示初學者首讀、銜接角色、根 workspace 全套驗證或公開發布已通過。

## 隔離、版本與實際覆蓋

唯一第 9 章材料是 `/workspace/work/tutorial-implementation/frozen/ch09-complete-prerequisite-v5`，依 `reading-order.txt` 實讀 4 頁、27 BODY units。全部 39 個 source archives 已讀，包含四個 package 的完整 Rust source、helper、tests、所有答案、compile-fail 及 expected、三份實際檔案材料、四份 package manifest、root 設定節錄與三張 SVG。先讀 BODY 再補完整檔案與答案，沒有用答案回填早期正文。

本輪封存前沒有讀第 9 章 FIRST、CONT、作者 ledger/source notes、其他 reviewer、root 處置、live docs 或未來修稿；沒有修改第 9 章、Cargo/nav/根 lock。環境重連後按自己的保存紀錄續查，未重新全文讀取已核同 hash 的材料。

manifest SHA-256 為 `c90b88a838d69a6ae1107656d49207bc98108c8a994b12ad63ba862a59f6b161`。首次及封存前指紋核對均為 27/27 chunks、39/39 sources 相符，沒有 hash failures。完整逐檔資料在 manifest；本人核對紀錄在 `/workspace/work/tutorial-implementation/ch09-technical-v5-probes/frozen-hashes.json` 及 preview 的 `visual-reviewed.json`。

| 原稿頁 | manifest 的頁面 SHA-256 |
| --- | --- |
| 9.1 Cargo 工程 | `e8b77ce34a23026585810500376fa7eff00af3eb03132a21e75d8ee45cfabace` |
| 9.2 build script | `83080a14ed8102b94fe71eae0c9f1494489591feaa0538fc8da3e4eef9b260e0` |
| 9.3 走訪檔案 | `81b1f3abdc2397a88fe0c9c649dac04c1ccbc7089bb35d3dbcab5b7e03c8ef4c` |
| 9.4 內容確認 | `acf84dff1c7842a13adf9cea394d2ac9cac2b9f0aa69879655c47d9a005410dd` |

依 `.agents/skills/clear-tutorial/SKILL.md`、`references/review-protocol.md`、`references/calibration.md` 與 `references/rust-terminology.md` 的通用標準判定；沒有加入待抓問題提示。英文核心術語與中文動作解說符合專案約定；沒有要求把 crate/package、ownership、reference、lifetime 改成全中文。

## 實際前文與首次使用核對

先前實讀背景：1 完整 v8；2 原始 v4 及局部 v5；3 v3；4 原始 v5、修後 v7 的 trait／lifetime 正文；5 修後 v6 的 5.1、5.2、5.6；6 prefix-five v4 與後續 Fn 名稱來源修正；8 ownership v3、dyn v4、interior v5、Any v6。這只是歷史實讀範圍，**不聲稱本輪重新通讀最新 1–8 全部內容**。

本輪為第 9 章另實讀以下 actual prerequisites：7 v13 的 7.1 相依設定、7.4 byte/Read/short read/錯誤事件、7.5 Path/PathBuf/OsString 與 native identity、7.8 相關 TempDir/fs::write/env! 單位；5 v6 的 5.3 Display/Formatter/write!/format!、5.4 From/TryFrom；6 v10 的 6.6 adapter/collect/型別路徑語法、6.7 Ord/cmp/sort；8 完整 v8 的 8.4 literal `'static`／borrowed 有效性。7.8 尾部不是全部重新實讀，本輪也不以未重讀的尾部作新的查證依據。

下表是由正文與 archive 實際使用重建的依賴，並非先從大綱取名詞再宣稱覆蓋。

| 必須能理解／判斷的關係 | 最早使用前的實際已教位置／本節解釋 | 核對結果與界線 |
| --- | --- | --- |
| 根設定、member 採用、相依不是自動取得 | 1.3 workspace/package 繼承；7.1 TOML、path dependency；9.1 `001-002` 在節錄與 member manifest 前說 root 定義、member opt-in、path 起點 | `workspace.dependencies` 沒被說成讓所有 member 自動 import；節錄明說不能覆蓋完整 root manifest。 |
| profile 改編譯設定，target 指平台，release 不換 edition/OS | 9.1 `001-002` 先教 dev/release、opt-level、root-only、overflow；`001-004` 在 cfg 平台例前教 host/target | Cargo flags 的角色分開；只說可能變快並要求同工作實測，沒用設定當效能證據。 |
| feature 列表與編譯條件的不同用途 | 1.10 已教 `#[cfg(test)]`；9.1 `001-003` 先教 `[features]`、default、三個 flags；`001-004` 再教 attribute 排除 vs `cfg!` bool | 真正使用前給出規則；library、binary 的 grouped block/use 與 integration tests 用同一條件；負例故意保留普通 if 的名稱檢查。 |
| script 的 stdout 是給 Cargo 的交付，不是 CLI 結果 | 9.2 `002-002` 先教四階段、env::var 的 Result/VarError、四種 cargo:: 指令及有限模式驗證，再放 build.rs | 沒把任意環境文字直接交付 rustc-env；root/host 執行與主要 crate/target 編譯分開。 |
| 執行時 String vs 編譯時 `'static` str | 7.5 OsString；7.8 env! 角色；8.4 literal lifetime；9.2 `002-002`/`002-003` 在程式前補 env::var、env! 缺值/Unicode 失敗及結果型別 | script 的 env::var 是 script runtime；產品函式的 env! 已嵌入，不因產品每次呼叫重新取環境。答案 `format!` 依 5.3，`CARGO_PKG_NAME` 在練習前說明。 |
| Bash 當次環境、成品直接執行、失敗來源 | 9.2 `002-003:41` 在 NAME=value 指令前說當次命令與子程序；同 unit 解讀 wrong 配置、-vv/cache | 指令有 package、`--locked` 和根 cwd；錯誤是 cargo::error 而非 Rust type error；直接 binary 使用本 repo Bash/Linux 示範路徑，不冒稱跨平台通用 shell。 |
| metadata 描述外部資料，PathBuf owner 沒有固定磁碟檔案 | 7.5 native path ownership；8 的 owner/有效性；9.3 `003-003` 在 derive/source 前教 Metadata/FileType/u64/SystemTime、Snapshot 有限意義 | `SystemTime` 的 Clone/Copy/PartialEq/Debug 在 derive 前給出；沒有把 Rust 借用保證延伸成 filesystem snapshot 保證。 |
| root link 與內部 link 是不同位置政策 | 9.3 `003-002` 先給 symbolic link／工作政策；`003-003` 先教 symlink_metadata 查最後項目；`003-004` 在 builder 前教兩個 follow 開關及不同預設 | source 同時 validate_root 與兩個 false；前後檢查的 gap 也明說，不承諾 hostile replacement 安全。 |
| 每項失敗仍可回報，固定輸出不能倚賴 walking 順序 | 5.3 typed Display；6.5 IntoIterator/Item；6.7 sort_by/cmp；9.3 `003-003`/`003-004` 先教 walkdir::Error traits、Option<PathBuf>、Item=Result、目錄失敗界線 | `collect_entries` 收每個 Err 然後 continue，沒有 `filter_map(Result::ok)` 丟錯；最後按 native Path 排序。 |
| DirEntry 的借用/轉移、native key、partial move | 4.3 Eq/Hash/Borrow 查鍵；6.3 FnOnce；7.5/7.8 path 複本；9.3 `003-004` 在 source 前教 path/into_path/file_type、Option::is_none/map；`003-005` 教 PathBuf traits/Borrow<Path> 及 partial move | helper、main、答案的欄位 move 正確：ScanReport 無自訂 Drop，移走 files 後可使用 issues；PathBuf key 仍保留 native identity，Display 不是再開檔 key。 |
| byte chunk、有效 prefix、有限來源 EOF 與 Interrupted | 7.4 actual Read 契約、array repetition、to_owned、Read helper；9.4 `004-003`/`004-004` 在程式前連到相同工作 | hash 只 update 有效 prefix；fill_chunk 只借剩餘段，Interrupted 重試，其餘 Err 不當 false/成功；正常例 source 的 max_per_read 正數限制已說。 |
| BLAKE3 狀態改變與 Hash 值能力 | 9.4 `004-003:26–31` 先教 new/update/finalize 輸入、receiver、結果與 traits，再用 Hasher | update 追加同序 bytes，finalize 不清空；Hash 的 Display/Copy/Eq/std::hash::Hash 能力先給出，不借機提前教其內部 unsafe/parallel API。 |
| 借來的分組 references、tuple key 與排序 | 4.4 單一 reference 來源/elision；4.3 HashMap get/get_mut/insert；6.6 collect/多層 reference/Vec::<T> 路徑；6.7 sort；9.4 `004-002`、`004-006`、`004-007` 先補 tuple Eq/Hash 與 callback `&&FileRecord` | 回傳 ContentGroup 借輸入記錄；sort 改 references 排列而非檔案內容；非空組 invariant 在 index[0] 前交代。 |
| 真實 File 與掃描記錄的前後觀察 | 7.5 File::open；9.3 observe；9.4 `004-005:7` 在完整 helper 前給 File::metadata 查 open handle 的新契約 | path 與 open handle 查不同對象，成功觀察都比 scanned Snapshot；觀察通過仍非原子／一致／永久內容保證。 |
| 測試所用 Unix、TempDir、fs::write/hard_link、assert 不是免查區 | 7.8 TempDir/fs::write/close；9.3 `003-006` 先教 Unix cfg、create_dir/symlink、OsStringExt::from_vec 與非法 UTF-8 材料；9.4 `004-004` 先教 assert!、`004-009` 先教 hard_link/u64::MAX | tests/helper/答案逐檔核對；Unix-specific 例沒有假定其他平台也有該 std module。沒有 super 路徑或其他未教 macro 的新增暗靠。 |
| 計數與 byte 長度不同整數，溢位不是 wrap | 1.4 有限整數；5.4 TryFrom 與 2024 prelude；9.4 `004-008` 在 MetricError/source 前教 TryFromIntError/Debug、checked_mul/add、path-based metric | path_count<2 先返回避免 usize 下溢；u64 轉換、乘法及加總都保留 Err；空內容/硬連結界線先說明。 |

本章實際用了 8.4 的 `'static`、先前的 ownership/Drop/resource 理解，而不是只有章末名詞預告。9.3→9.4 也沿用 FileRecord、原生身分、typed failure 與非原子觀察；不把 8→9 銜接誤寫成每個 8 章型別都必須在 9 重用。archives 沒有將尚未教的 thread、atomic 或 unsafe API 放入例子；第三方 crate 的內部實作不是讀者自行撰寫例子的先備。

## 主要通過項：機制、需要、操作與證據分開

### P1．Cargo 設定與 cfg（9.1，001-001 至 001-006）

原問題是「哪些設定共用、這次編譯選哪些功能，以及使用哪一組編譯設定？」（`001-001`）。`001-002` 的 root/member 關係、`001-003` 的 feature 宣告、`001-004` 的 attribute/macro 對照分別回答它，不由 280 的算例代答三種工具角色。

機制／性質已足夠：root 定義提供共用來源、member 明確採用；profile 控制編譯設定；attribute 會排除整個函式／block，cfg! 僅交付 bool。需要／用途亦足夠：維護多個 package 時在一處維護同來源，仍讓各 member 依賴明確；可選的兩筆函式在未啟用的建置中必須連呼叫一起排除，才完成同一 package 的兩種可編譯介面。

80×2=160，120×1=120，兩筆280的數字、main 條件輸出及測試一致。features 測試有不同單價/數量及零數量，並非把兩筆借同資料湊出結果。cfg! 負例的 E0425 支持名稱仍需合法這個機制；不支持 performance、全部 cfg 表達式或跨平台產物可執行性。根 profile 的引用按官方契約可成立，但本凍結只封存 root 節錄；本輪不宣稱另讀過完整當前根 Cargo/lock。

### P2．build script 的交付與建置識別（9.2，002-001 至 002-005）

原需要在 `002-001:3` 已獨立給出：「執行後都能回報同一份建置設定，而不是每次啟動都重新讀環境。」不是審閱者從 env! 產物反推的需求。

機制／性質已足夠：四階段圖及 `002-002` 說 script 先在 host 執行，將選定模式以 cargo::rustc-env 交給主要 crate 編譯；`002-003` 說 env! 固定文字，runtime env::var 才每次讀取。需要／用途已足夠：同一成品記住建置選擇；再次 Cargo run 改 mode 會重建，直接成品改 runtime mode 不改識別。圖中 LESSON_MODE→TEACHING_MODE 的換名有圖說和 source 對應，不靠相同字串猜兩者為同一變數。

VarError 分支、finite accepted modes、target 資料與 host cfg 的分工、錯誤先於 rustc-env 和 newline 輸入的因果均可從原稿與 source 核對。獨立 probes 真實支持「practice 成品不因 runtime demo 改寫」「invalid/nonUnicode/newline 模式在 build fail」，不代表獨立跑過 Windows cross-compile 或所有 Cargo cache 場景。

### P3．走訪分工、路徑身分與錯誤（9.3，003-001 至 003-007）

原需要是列出一般檔案（含隱藏目錄）、保留原生位置/byte 長度/修改時間，並避免把「沒有列出」誤當「不存在」（`003-001`）。政策略過與項目失敗先分開，root 無效則工作不能開始。

機制／性質已足夠：walkdir 保留逐層 walking 進度並逐筆交出 Result；reader 的工作仍是自行決定 root/link/regular file/Unicode 政策、metadata 觀察和資料/錯誤保存。需要／用途已足夠：把逐層進入交第三方，不改原需求；收 Err 仍 continue 讓後續成功與原因一起返回，最後排序才滿足報表順序需要。不是以「第三方必然更好」為理由，也沒把打不開目錄說成其所有 descendants 仍能被枚舉。

native PathBuf 的 owner、borrowed Path 查詢與 lossy display 角色分開；原生不同資料不因顯示文字相同而被重開為另一檔名。對非 Unicode 檔案的排除是明示最小政策，仍留下原生位置和 NonUnicode cause，不假稱完全處理所有檔案。fixture 實際內容為 5、5、6 bytes；最後一份藏在 `.hidden`，source 沒做名稱以點開頭排除。受控 walking 失敗後接兩個成功來源的測試與 collect_entries 的分工一致，但本輪沒有親跑該測試，不宣稱取得 runtime PASS。

### P4．大小、digest 與真正內容比較（9.4，004-001 至 004-004）

原問題是「哪些不同路徑的檔案，讀出的 bytes 完全相同？」（`004-001:3`）。表格清楚說各階段排除什麼與仍無法判斷什麼。

機制／性質已足夠：長度不同排除；同長度按摘要縮候選；hash_reader 接各次有效 prefix；真正比較前把兩邊的短 read 累積到同容量或各自 finite EOF，然後比有效 slice。需要／用途已足夠：利用已得 metadata，跳過沒有同長度對象的獨一長度；digest 把仍需比較的工作集中到候選桶；但相同摘要仍要內容確認。這段明說「沒有測量或承諾特定倍數的加速」，不能把結構分工作為 host 上速度證明。

`fill_chunk` 用剩餘 slice、讀後累加 filled，不覆蓋前面已收 bytes。兩邊皆空則 true；段長／byte 不同則 false；Interrupted 重試，其餘觀察到的 I/O Err 區分 left/right 返回，不冒充正常差異。四格圖的 ABC|D|E 與正文最初概念上的 ABC|DE 不相矛盾：`004-004:9` 指出只剩一格限制下一次 read 至 D；圖的 bar 是 read 批次，不是內容 byte。4096 實作只改容量，沒有改相等判準。

source tests 換 9001-byte、多個容量邊界、非 UTF-8、empty、尾 byte／尾長度與中途錯誤，覆蓋選定機制；這是讀 source 得到的設計覆蓋，不是本人實跑結果。泛型 R:Read 的有限來源與符合 Read 契約前提仍在，不保證永久不 EOF 或永久 Interrupted 的來源會完成，也不以這種未承諾範圍報缺。

### P5．真實 File 的觀察與誤差邊界（9.4，004-005）

`004-005:7` 說「盡力而為的觀察」；`004-005:147` 說「沒有看見差異，不代表取得一致快照」。機制／性質已足夠：observe 查原生路徑，File::metadata 查 open handle，兩種成功觀察都與 scan Snapshot 比，前/後或可觀察差異保留 typed FileIssue。

需要／用途已足夠：9.3 的成功 metadata 並不保證後續能打開／讀完，wrapper 保留具體失敗並揭露可見變動，以免將失敗當「不是重複」；結果仍只服務穩定目錄的一般整理。原文自己給出 same len/modified、精度和替換 gap 的反例，不把借用、path owner、metadata 查詢或測試變動誇大成 race 安全。

兩份真實檔案例會得到相同 digest／same=true，與 6-byte 另一份比較 false；source 即使 equal_readers 回 false 也做其後觀察。測試「先讀完、再改不同長度、再呼叫 verify_unchanged」只支持那個觀察步驟可揭露差異；原稿也明說不是同步改每次 File read 的競爭測試。本人未獨立實跑此檔案 wrapper，不以 root 的成功紀錄替代。

### P6．候選桶中 A／B／B 與真正群組（9.4，004-006、004-007、004-009）

機制／性質已足夠：`(u64, Hash)` 兩欄都相等才同候選鍵；`refine_bucket` 只借來源，依各個非空組的代表比，全部不同才建新組，同 native path 重複輸入先略過。

需要／用途已足夠：same digest 仍可能有不同內容，所以不能只與候選桶第一份比較就判不存在其它相同者。`004-007` 明示 a=AAA、b2=BBB、b1=BBB 的順序；b1 第二次比 b2 才進 B 組，最終 A 單組與 B 雙組。這個材料直接讓必要第二次比較發揮作用，不需讀者自行選另一個情境才能看見理由。

圖和答案是手工同摘要鍵，不是假稱找到真 BLAKE3 碰撞。單例組仍呼叫 hash_file 完整驗證，原文說明獨立 caller 未必剛讀過的需要及多讀一次的代價；不把該摘要代替真內容判準。error 回傳沒有交付半成品群組；最終專案如何移除 offender/retry/完整 CLI 報告合理待教，當前 wrapper 與答案也未偷用它們。

### P7．計量不是刪除建議（9.4，004-008、004-009）

原稿先定义 path-based 的 `(path_count - 1) × len`，三份 3 bytes 得 6、兩份得 3，空內容得零。hard link 已給「同一實際檔案的另一個名字」，所以 path 指標不能改讀成實際磁碟可回收量。

機制／性質已足夠：`path_count<2` 先返回；u64::try_from 的錯誤型別及 Debug 在 source 前給出；checked_mul/add 成功 Some、超出範圍 None。需要／用途已足夠：9.3 u64 byte 長度與 usize 清單數不能暗自互換，應把計量失敗另存而不是 wrap 成似乎合理的報表。u64::MAX 測試材料先解釋 associated constant，不需要巨大檔案。此 source 支持 checked metric 的設計，不能據此授權刪除任何檔案。

## Selected 官方契約

查證的是原稿實際選用 API，不要求提前教完整標準庫或第三方內部。Rust/Cargo 官方頁與目前 pinned toolchain 為 1.99.0；walkdir 為 2.5.0，BLAKE3 為 1.8.7。下表的契約各已讀相關段落／實作；URL 存在本身沒有算通過。

| 來源 | 真正支持的範圍 |
| --- | --- |
| [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)、[specifying dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) | member 繼承需明確 opt-in；共用依賴不是每個 member 的自動 import。原稿沒用 1.99 新增的 default-features override，未把該進階差異提前塞入教材。 |
| [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) | root-only、dev opt-level=0、release=3、release 預設 overflow-checks=false；明設 true 超範圍 panic。優化程度不是速度保證。 |
| [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) | feature 限定於定義的 package；selected package flags 的作用；共用 dependency features 通常 union/additive 的設計。 |
| [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) | host script→main compile、stdout cargo:: 協定、rustc-env、cargo::error、rerun-if-env-changed 限 Cargo invocation 的外部環境、OUT_DIR 不自動清乾淨與分開 build-dependencies。cargo:: 與 error 的 MSRV 低於本 repo 1.99。 |
| [std::env::var](https://doc.rust-lang.org/std/env/fn.var.html)、[env!](https://doc.rust-lang.org/std/macro.env.html) | Result/String/VarError 與 compile-time &'static str、缺值/非 Unicode 失敗不同。 |
| [walkdir WalkDir](https://docs.rs/walkdir/2.5.0/walkdir/struct.WalkDir.html)、[DirEntry](https://docs.rs/walkdir/2.5.0/walkdir/struct.DirEntry.html) | IntoIterator Item=Result、inner/root follow 開關與不同預設、遇錯下一項／未能讀的子目錄界線、cached file_type、path/into_path borrow/owner。 |
| [walkdir Error 原始 source](https://docs.rs/walkdir/2.5.0/src/walkdir/error.rs.html) | direct Error API 頁取得失敗後改讀真正 source：Debug derive、Display impl、path Option<&Path>；沒有把未能取得的頁面算已讀。 |
| [Metadata](https://doc.rust-lang.org/std/fs/struct.Metadata.html)、[symlink_metadata](https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html)、[File::metadata](https://doc.rust-lang.org/std/fs/struct.File.html#method.metadata)、[SystemTime](https://doc.rust-lang.org/std/time/struct.SystemTime.html) | regular-file/len/modified 的結果及平台失敗；最後 link 查詢與 handle 查詢不同；所选 SystemTime traits。沒有任何這類 API 契約提供全段原子 filesystem snapshot。 |
| [PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html)、[Option::map](https://doc.rust-lang.org/std/option/enum.Option.html#method.map) | PathBuf Borrow<Path>/native Eq/Hash/Ord 與 source lookup；map 接 self、FnOnce(T)->U，None 不執行；本節 Option map 沒被誤當會丟 Result Err 的 iterator filter。 |
| [BLAKE3 Hasher](https://docs.rs/blake3/1.8.7/blake3/struct.Hasher.html)、[Hash](https://docs.rs/blake3/1.8.7/blake3/struct.Hash.html) | 普通 update 單 thread、追加 input；finalize 可重複且不清空；32-byte Hash 的選用 traits。此查證不引用 mmap/rayon 的 performance 來支持教材。 |
| [Read](https://doc.rust-lang.org/std/io/trait.Read.html) | read n 不超過 buffer、只前 n bytes 有效、短 n 非錯誤、Interrupted 重試；Ok(0) 也可能出自空 buffer／未永久結束的特殊來源，本例用非空剩餘段與有限來源。 |
| [u64](https://doc.rust-lang.org/std/primitive.u64.html) | checked_add/mul 的 Option 與溢位 None；TryFrom<usize>::Error=TryFromIntError、超範圍失敗。 |

重連後 selected 契約補讀保存於 probes 的 `official-resume-selected.json`。其餘 selected 頁面已在此角色前段讀取；本報告不聲稱保存了每個網站的完整鏡像，也不以工具輸出截斷部分算已讀。

## 相稱實跑：六項 own-lock probes

在 `/workspace/work/tutorial-implementation/ch09-technical-v5-probes` 複製原始 ch09-build-script source，建**自己的**最小 workspace、1.99 toolchain、獨立 Cargo.lock、CARGO_TARGET_DIR；沒有改根 workspace。完整 command/exit/stdout/stderr 在 `verification.json`。沒有重跑 root 全套來代替時序判斷。

| probe | 實際結果 | 能支持／不能支持 |
| --- | --- | --- |
| practice `cargo run -p ch09-build-script --locked` | exit 0；三行見下 | build.rs 選定模式並嵌入主要 crate；本機 Linux host/target。 |
| runtime demo 直接執行上述 binary | exit 0；仍是同樣三行 practice | 產品 runtime 環境不能改寫已嵌入的模式。 |
| wrong 模式 cargo check | exit 101，`LESSON_MODE 只接受 demo 或 practice`，build script logged errors | 原稿所說 cargo::error 建置失敗原因；不是 type error。 |
| 含 newline 的 mode | exit 101，同非法模式原因 | 選定值驗證先於協定輸出，沒有接受額外 rustc-env 指令；是本人額外小變化，不算原作者已實跑材料。 |
| 非 Unicode mode bytes `[255]` | exit 101，`LESSON_MODE 必須是可表示為 Unicode 的 demo 或 practice` | 已定義 VarError 分支選用；Unix 本機環境，不代表所有平台 bytes 輸入可用。 |
| 原始 cfg_bool_is_not_removal rustc fixture | exit 1/E0425，`cannot find function two_orders`，note configured out | 未啟用 batch 時普通 if 仍名稱檢查；只驗原稿預期 cause。 |

兩項正常輸出的 exact stdout 相同：

```text
建置時選的模式=practice
主要 crate 的 target OS=linux
script 的 host 是 Windows=false
```

這次 own minimal lock 沒第三方依賴，不等於驗過 repo 原始根 lock 中 walkdir/BLAKE3/tempfile resolution。9.3/9.4 normal examples、integration tests、Unix link fixtures、File races、release 功能矩陣與公開網站**本輪未獨立實跑**；對它們的判斷是實讀 source、材料與 selected 契約，不借用 root 的 PASS。沒有用測試跑通證明性能、race 安全或所有來源終止性。

## 本人實頁／圖核對與可選項

本人重建 **original-only preview** `/workspace/work/tutorial-implementation/ch09-technical-preview-v5`：按原 reading order 接四頁，僅去掉重複頁標題、映射凍結 SVG 路徑，保留折疊答案與相同教學內容；採本 repo theme/extensions，nav 只放這四頁。不是取 live docs 作修後預覽。

Chromium/Playwright 本人生成 1280×800 桌面及 390×844 手機截圖，**44/44 本人已透過 view_image 親看**：每頁入口及三個選定原位段落（32張）、三圖原位及 element crops（12張）。實際清單/每圖 SHA 在 `visual-reviewed.json`，位置與 bounding box 在 `render-checks.json`。這是選定段落與圖的抽查，沒有逐一捲完整頁每個位置，也未測深色模式、全部折疊展開 UX、公開 deployment。既有重建 HTML 與截圖確實可開；先前 build tool 尾部輸出截斷，**不把未保留完成狀態另稱本輪 strict build PASS**。

三圖在 desktop/mobile 原位均為 340px 寬；字、箭頭和圖說可讀，不需放大。本人核對的必要對應：

| 圖 | 本人親看與判斷範圍 |
| --- | --- |
| build-phases.svg | 第2框讀 LESSON_MODE／交付 TEACHING_MODE，第3框 env! 固定、第4框不再讀；箭頭是先後，不是「程式啟動四次都讀環境」。圖後文字解換名與順序；原位可連續追蹤。 |
| short-read-comparison.svg | 同一 ABCDE、左右 read 批次 bars、填到4格或EOF，再得到 ABCD/E 兩比較段。正文明確說剩餘一格改下一次read可收量，四格與4096只是容量示意差異，沒有不同算法。 |
| candidate-content-groups.svg | 上框是 forced digest 候選，中框 b1 先a不等再b2相等，下框 A單例與B雙例。圖前已有a/b2/b1具體內容和順序，圖說強調第二比較；不是依標籤A/B猜hash值相同就同內容。 |

### I1．可選呈現改善：兩處強調標記直接顯示星號

- **版本／位置／原句：**原 v5 `002-001:5` 的 `**Cargo 先執行一支 Rust 程式，把結果交給接下來的 crate 編譯。**這支程式叫 build script`；`004-001:3` 的 `**哪些不同路徑的檔案，讀出的 bytes 完全相同？**檔名相同`。
- **實際發現來源／時點：**本輪未看同行之前，本人 view 兩頁 desktop/mobile entry 時看到 `**` 原樣出現在正文，未成預期強調。畫面為 `desktop/mobile-09-02-build-scripts-entry.png` 與 `desktop/mobile-09-04-content-confirmation-entry.png`。不是 root 問題提示，也不是推測 Markdown source 必然 render 壞。
- **已教部分：**句子、前後學習問題與必要機制仍可讀；四階段圖、內容判準表和後續說明沒有丟失。這不構成 Rust/API 錯誤或當下概念缺口。
- **缺少／理解收益：**實頁未把本來的轉折／主要問題作預期視覺強調，且引入不屬教學符號的星號。修妥可讓讀者更容易辨認問題與新增步驟，減少純排版噪音。
- **分級：**可選編輯／呈現改善；未觀察到需要猜測核心關係或必須回讀才懂，不升為必要。
- **最小建議：**只調整這兩對強調 delimiter 的鄰接邊界，或改成引擎能正確解析的等價 strong markup；再親看這四張 entry 位置。不需改動 Rust source、新增 API、重跑 examples 或擴寫概念。
- **處置／修後：**本任務只審原稿，未改；交 root 依來源收益判採用／保留。沒有修後複查，本紀錄不可回填為已修。

## 未驗證與封存邊界

1. 此為技術角色的來源初判，沒有第 9 章新手逐段首讀身份，也沒有以本人能重建作者意圖推定基本 Python 讀者已學會全部設計。
2. 不在本輪讀取範圍的 1–8 新修全文、對應 reader/continuity 判斷與後續第10/最終專案接手，未宣稱本輪全鏈驗收；已教依據只取上面具體 actual 內容。
3. 非 Unix 執行、cross-target build、穩定目錄外惡意替換、安全原子快照與性能測量未做，也不是原稿的承諾。
4. original archives 的正常/答案/tests/source 已查，但除了六個 targeted probes 不宣稱親跑；並未為低風險措辭寫鏡像測試。
5. 本輪所有第 9 章發現於 peer 互看前保存。報告 SHA 另存 `ch09-technical-v5.md.sha256`，原初判封存後不覆寫；將來若獲授權 cross 或修後，另存 followup，保留此版本與分級。
