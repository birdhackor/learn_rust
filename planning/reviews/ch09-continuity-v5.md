# 第 9 章 CONT 獨立初判：ORIGINAL v5

本輪未發現需要補教的必要先備、方法理由或概念銜接缺口。四頁／27 個 BODY 單位及封存 source 的主線可由實際第 1–8 章接續；三張 SVG 的原位桌面／手機檢查通過。另列兩項可選版面改善：正文中未生效的粗體標記，以及折疊答案的程式字級。這是第 9 章原稿的獨立 CONT 初判，不是全書首次閱讀、全面技術驗收或發布裁定。

## 角色、範圍與隔離

- 審閱者：`chapter03_data`，2026-10-14。未撰寫第 9 章；已有第 3、6 章作者背景，先前因作者 gate 讀過第 1–5 章較早凍結稿，也熟悉 Rust/Cargo/std。此背景先保存在 `/workspace/work/ch09-continuity-v5/background.md`，不能將本輪稱為零 Rust 背景的 FIRST 盲讀。
- 本輪依 clear-tutorial、review-protocol 與 rust-terminology 進行；只審閱，未修改學生文件、範例、shared config 或任何同行紀錄。
- 唯一第 9 章版本：`/workspace/work/tutorial-implementation/frozen/ch09-complete-prerequisite-v5`。閱讀四頁全部 27 BODY，並讀封存來源；恢復工作時保留原 `observations.md`，追加本次核對，不改寫較早觀察。
- 順序讀者的入口是實際第 1–8 章：`ch01-complete-source-v8`、`ch02-review-fixed-v5`、`ch03-review-fixed-v6`、`ch04-publication-target-fixed-v8`、`ch05-review-fixed-v7`、`ch06-review-fixed-v10`、`ch07-review-fixed-v13`、`ch08-review-fixed-v9`。以下按必要關係回查實際段落，沒有以大綱、作者記憶或選讀代替已教證據。
- 封存前未讀 FIRST9、TECH9、root 處置、任何 peer9、作者第 9 章 ledger/source notes；未取得第 9 章作者提示，也未讀後章或 peer10。恢復指令只交代進度及隔離規則，未提供預期結論。初判保存後 STOP，待 root 另開 cross。

## 指紋與實頁來源

凍結 manifest SHA256：`c90b88a838d69a6ae1107656d49207bc98108c8a994b12ad63ba862a59f6b161`。本次重新核對 27 BODY 與 39 封存 source，共 66 筆 SHA，全部相符；紀錄見 `work/ch09-continuity-v5/resume-integrity.json`（本段及下文的 work 均以 `/workspace/` 為起點）。

| 原始頁 | manifest 記錄的原頁 SHA256 | BODY 範圍 |
| --- | --- | --- |
| 09-01-cargo-engineering.md | `e8b77ce34a23026585810500376fa7eff00af3eb03132a21e75d8ee45cfabace` | 001-001–006 |
| 09-02-build-scripts.md | `83080a14ed8102b94fe71eae0c9f1494489591feaa0538fc8da3e4eef9b260e0` | 002-001–005 |
| 09-03-file-traversal.md | `81b1f3abdc2397a88fe0c9c649dac04c1ccbc7089bb35d3dbcab5b7e03c8ef4c` | 003-001–007 |
| 09-04-content-confirmation.md | `acf84dff1c7842a13adf9cea394d2ac9cac2b9f0aa69879655c47d9a005410dd` | 004-001–009 |

固定預覽位於 `work/ch09-continuity-v5/preview`：由通過 SHA 的 BODY 依 manifest 順序重組，第一個頁標題保留，之後移除閱讀包重複頁標題；只改寫圖檔 URL 為預覽相對位置。每頁將 URL 還原後與組合 BODY byte 相同，不宣稱展開 snippets 的 BODY 等同未展開的原始 Markdown。三張 SVG 也與封存的 `sources/docs/assets/ch09/` byte 相同。`preview/byte-integrity.json` 保存逐頁重組、URL 還原與資產證據。

預覽保留站點 theme/extensions，僅將 nav 固定為這四頁並取消章節排除。config SHA256：`01192cc6a296230d3cb7e0a67a6097c197132748dfeb346a5c206fc4ce53a2e2`。本次局部 `zensical build --strict` 通過；沒有改 repo config，也沒有讀當前可變的第 9 章正文替換原稿。

原有來源觀察 `observations.md` SHA256：`711313e6988e4df67a00d0967060eb7956ef73374c1bb386ef93388cf765a04e`。本報告另附 `.sha256` 並複製到自己的封存目錄，保留原初判，不用交叉閱讀回填獨立發現。

## 必要先備與首次使用核對

表中的「當頁補教」指正文在第一份需要該契約的程式前已解釋；單純 API 名稱出現不列為已教。

| 第 9 章需要的理解 | 實際已教來源／當頁補教 | 核對結果與支持範圍 |
| --- | --- | --- |
| workspace 根設定與成員採用、package/crate、library 路徑 | 1.3 `003-002/004` 實教兩種單位、成員 `.workspace = true`；1.9 `009-002/003` 實教 module、公開路徑與連字號／底線 | 9.1 比較根與成員 path 起點，再教 `[workspace.dependencies]` 與 dependency 明確採用；每個新 package 都先交代身份及指令起點。不是憑同 workspace 自動取得依賴。 |
| registry/version/features/dev-dependencies 與本機 path | 7.1 `001-002/003` 的來源、行內表格、版本要求、clap 功能與 dev-dependencies 契約 | 9.1 新增「自己提供 feature」的表、default、空清單與選項；9.3/4 新依賴 manifest 沿用已教來源規則，library 名稱先於 `use`。 |
| 既有數字工作與 cfg/test/integration test | 1.9 `009-003` 的 subtotal／運費；1.10 的 test；7.2 `002-003` 獨立測試 crate | 9.1 001-001 明說這次只算商品小計，運費不改；001-004 回收 `#[cfg(test)]` 後教 feature/item/block/cfg! 差別；001-005 解讀一項／兩項測試，沒有把 all-targets 當全 OS。 |
| reference、slice、Vec 的 owned／borrowed 分工 | 2.6 `006-002` 的 range slice、長度；3.6 `006-002/004` 的 Vec、push、as_slice/as_mut_slice、索引及已知邊界 | byte buffer 明寫 array→range slice，沒有省略一個尚未教的轉換；群組索引 0 的前提在 004-007 明說「每組至少一位」，空輸入則零組。 |
| derive 的欄位條件、map 鍵身分與 Borrow | 4.2 `002-002`；4.3 `003-002/003/005` 的 Eq/Hash/collision、借用查詢與 get_mut | 9.3 先教 SystemTime 的 Copy/Debug/PartialEq、walkdir 錯誤 Debug/Display；003-005 再教 Path/PathBuf Eq/Hash/Ord/Borrow<Path>。9.4 先列 u64 能力及 tuple 鍵逐欄一致性，沒有把顯示字串當路徑身分。 |
| Display／format、失敗政策、TryFrom／? | 5.3 `003-002/004` 的 shared reference 格式化、write!/format!；5.4 `004-002/004` 的 prelude、可失敗轉換與 associated Error；5.1/5.6 的 expect 與 Result 傳遞 | 9.2 先給環境缺值／無法轉換／錯誤模式政策，再使用 match/expect；9.3/4 先建立各層具體錯誤。9.4 `TryFromIntError`、checked_mul/add 的 Some/None 另在當頁教，未當成舊 Count 介面。 |
| closure、Iterator/IntoIterator、filter/collect、map iteration 與排序 | 6.3 callable 契約；6.5 的 next/Item；6.6 `006-003/004` 的 &Item、多層借用及目標收集；6.7 `007-003/004/006` 的 Entry、map 三種 Item、sort_by 的 &T 和 Ordering | walkdir Item 具體為 Result；Option::map 的 FnOnce 零／一次另教，沒有與 iterator map 混用。9.4 `&&FileRecord` 的排序參數在 code 前由元素 `&FileRecord` 接起。 |
| OS 字串、原生路徑、TempDir owner 與測試材料 | 7.5 `005-002/003` 的 PathBuf/&Path、to_str、OsString/OsStr、display 的失真限制；7.8 `008-003/005` 的 TempDir、fs::write、to_path_buf、env! | 9.2 的 NotUnicode 不等於未設定；9.3 的 NonUnicode 政策保留原路徑及原因。練習／測試維持 TempDir owner 至工作完結；新 create_dir/symlink/OsStringExt 在 003-006 教，hard_link 在 004-009 教。 |
| Read、短讀取、有效前段、EOF 與 Interrupted | 7.3 的 File/io::Error/NotFound；7.4 `004-002–005` 的 u8、重複初始化 array、read 契約、非空 buffer 的 Ok(0)、重試及 is_empty | 9.4 的 hashing 只用有效前段；equal_readers 先新增 fill_chunk 的共同段界，再比較新教的 slice PartialEq，不能以兩次 read 的 n 差異判內容不同。測試沒有偷偷使用 UTF-8 解碼或新 Reader API。 |
| 含借用模型、placeholder、'static 與 method 借用查找 | 4.4 `004-002/003`、4.5 `005-002/003`；8.3 的 Deref 借用／method 查找；8.4 `004-003` 的 literal/'static | 9.2 的編譯期文字與 9.3 action literal 接到既有 lifetime；9.4 ContentGroup 保存 FileRecord reference，輸出 lifetime 接唯一輸入 slice，不暗中複製 record 或檔案。9.3 partial move 是當頁新增，使用前有明確範圍。 |

除以上完整段落回查，也核對本文所引用的 assertion、trait scope、格式化、借用／Copy、map 的 get_mut／Option reference 比較等實際段落。這是針對第 9 章依賴的核對，不宣稱本輪重新審完全部第 1–8 章。

## 原問題、設計理由、例子與通過範圍

每項分開檢查身分／操作、機制或性質、需要或用途，以及例子支持的關係；通過項並未只用「程式可跑」代答理由。以下引用皆來自指定 v5 BODY，不含同行提示。

### 9.1：共同來源、此次介面與編譯設定

1. **001-001/002：共同依賴與 profile。** 原問題是「哪些設定共用、這次編譯選哪些功能，以及使用哪一組編譯設定？」共同 dependency 來源放 root、成員明確採用，改變的是同來源／版本的維護位置，沒有把所有成員變成互相依賴；兩種 path 起點及 member 設定連起實際操作。profile 是一組編譯設定，root-only、opt-level 與 overflow-checks 各自有角色。用途是選擇編譯成本／最佳化及保留整數檢查，而非保證實測加速。**機制／性質足夠；需要／用途足夠。**
2. **001-003/004：feature 與 cfg。** batch 是自己 manifest 宣告的功能；關閉時保留單筆規則，只移除批次公開函式。`#[cfg]` 決定是否編入，`cfg!` 只交 bool；普通 if 仍需型別／名稱檢查。正常有／無 batch 程式與故意失敗 E0425 fixture 使用同資料，真正隔離了「編譯排除」與「執行 branch」的差別。host/target 在平台 bool 使用前已教，release 不改 target。**機制／性質足夠；可選介面的用途足夠。**
3. **001-005/006：驗證與回收原問題。** 商品小計 160／280 有已教 subtotal 的來源，零件數變化得到 120，能排除誤用第一筆資料；沒有拿原運費例的 180／240 充當新結果。測試組合、release 輸出及章末三題回收共同來源／feature／if 區別。原句明說「沒有作速度測量」，所以例子只支持可建置與相同結果。**例子範圍足夠；效能實測不適用。**

### 9.2：將建置當下的資訊固定到成品

1. **002-001/002：新增工作的角色與協定。** 原需是執行後仍回報同一份建置設定，不是每次啟動重新讀環境。先教編譯 build.rs、host 執行、主要 crate 編譯、成品執行；script stdout 協定將資訊交給 Cargo，再供 env! 固定。向下箭頭有先後語意，LESSON_MODE→TEACHING_MODE→程式文字的資料關係也點名。`env::var` 的 owned Result、VarError 兩分類、模式政策與四種 cargo:: 指令皆在 code 前；`cargo::error` 正常 return 仍失敗也已解釋。**身分／操作足夠；機制與建置固定需求各自足夠。**
2. **002-002/003：host/target 與兩個時點。** script cfg 看其自身編譯目標，因此是 host；CARGO_CFG_TARGET_OS 由 Cargo 描述主要 crate target。來源用「平常碰巧相同、其他平台可能不同」限縮普通 Linux 輸出的證據，沒有藉它證明同義。兩次 Cargo 建置變更模式，及直接執行既有成品時變更環境，能觀察為何值固定；配置錯誤命令又顯示要停止建置的政策。**機制／性質足夠；讓回報對應該份成品的用途足夠。**
3. **002-004：OUT_DIR／build-dependencies 是相稱導覽。** 已說明生成檔案應寫哪裡、可能留前次內容、host 依賴與主要 crate 依賴不互通。本例不生成檔案、不新增第三方，當下沒有需要讀取生成檔案的 macro；不要求把 include 或生成器補進這個模式例。**目前角色足夠；完整生成機制合理待教，當下操作不依賴。**
4. **002-005：練習沒有折疊先備。** CARGO_PKG_NAME 的 Cargo 來源先教，沿用 env!、已教的 mode/target 函式及 format! 形成一個 String；答案只實作已給三步。章末收束「建置當下固定成品」，再將執行期的檔案位置／錯誤引向下一節。**練習與過渡足夠。**

### 9.3：從一個原生路徑擴至目錄樹，保留觀察失敗

1. **003-001/002：工作政策先於 walker。** 從 7.5 單檔輸入變成目錄樹，原需明列一般檔案、隱藏子目錄、原路徑、長度與修改時間；不讀內容、不判重複。「根設定不成立則整個工作無法開始；個別項目失敗則保留原因，繼續其他項目」給了獨立於 API 的答案判準。symbolic link 是指向另一位置的項目，循環由指回上層解釋；政策表分 root 與內部 link，避免「不是目錄就當一般檔案」。**角色與需要皆足夠。**
2. **003-003：metadata 與資料／錯誤模型。** 新 fs::symlink_metadata 不追最後項目，Metadata 的 type/len/modified 契約與 u64/SystemTime 能力先於 Snapshot；FileRecord 保存原路徑，沒有固定磁碟內容。RootError 與 ScanIssue 分開，避免根失敗被當空清單；path 可 None 的原因也在使用前教。observe 重新檢查一般檔案，回應先前觀察可能改變的需要。**機制／性質足夠；完整性與失敗報告用途足夠，非 atomic snapshot。**
3. **003-004/005：第三方分工、借用及正式工作。** walker 保存逐層進度，自己的元件保留政策／分類；原文沒說自寫 walker 不可能。Item Result 的 Err 仍可下一筆，但無法取得打不開目錄的內部，限定有直接用途。follow_links 與 follow_root_links 分別控制內部／root，配合 validate_root 完成政策。DirEntry 的 path/into_path、錯誤 path、Option.is_none/map、PathBuf Borrow<Path> 都在正式 source 前。partial move 在 main 前教，並讓 files move 後仍讀 issues。**機制／性質足夠；政策和 owner 身分的用途足夠。**
4. **003-006/007：成功不是沒有失敗。** 受控 Ok/Err/Ok 材料驗證中間錯誤保留且後續成功仍處理，不靠目前權限造假；Unix fixture APIs 先教且平台限定清楚。path map 練習要求查到 Some(&5) 仍回報 issues，沒有把局部成功當完整掃描。原生身分與供人閱讀的表示始終分開。**例子關係足夠；CLI exit policy 明確留完整專案，當頁只承諾元件資料。**

### 9.4：由候選縮小工作，再以 bytes 確認真正群組

1. **004-001/002：三種判準與先看長度的理由。** 原問題是「哪些不同路徑的檔案，讀出的 bytes 完全相同？」長度、摘要、內容各能排除／仍不能判定的表先建立答案範圍。利用已取得 metadata，例如唯一 6-byte 檔無同長度對象，不用為它讀內容；重工作集中在較小候選。不是先讓新結果反定義需要，也沒有測量或保證固定倍數加速。by_size 借用 FileRecord，u64 鍵能力、任意 map 順序後的排序、5/5/6→2 候選各有來源。**機制與用途足夠；實際速度未知且正文已限縮。**
2. **004-003：digest、BLAKE3 與有效前段。** Hasher 是累計狀態、update 追加有效 bytes、finalize 借出結果而不清空，Hash 是摘要值而非同名 Hash trait；每個角色／receiver／結果及 trait 條件在 source 前。相同 bytes 必同摘要，不同仍可能 collision，對應候選而非確認。ABC+DE 與 ABCDE 接起分批規則；只更新 buffer[..n]、Interrupted 重試、空與非 UTF-8 成功，回收 7.4。**機制／性質足夠；縮桶用途足夠，非證明相同內容。**
3. **004-004：把短讀取切分差異排除後才比較。** 同串 ABCDE 的左右 read 批次不同，是獨立於 fill_chunk 的原問題。各自填同容量或到 EOF，再以新教的 slice PartialEq（同長度且各位置相同）比較；否則 3/1 只是來源切分差異。圖的四格對實作 4096 有明示，ABC、D、E 解釋剩一格只能收 D；line/bar 不是資料 byte。CompareReadError 的左右身分、Err 不當 false、兩邊空才 true，以及 ShortReader/正數上限/位置變化都先教。assert! 再使用前也有契約。**操作、機制、用途及圖例關係皆足夠。**
4. **004-005：成功觀察不是保證一致快照。** PathBuf 不鎖檔案；開啟前、handle 開啟後讀取前、工作後的觀察與原 Snapshot 比較。新 File::metadata 接到既有 Metadata；FileIssue 保留確切左右路徑、動作與原始原因。前後可見差異的受控 fixture 支持觀察能揭露變動，不是假裝每次 read 都同步改檔。正文保留時間精度、改後恢復與路徑替換等無法保證的情況，防止把 Ok 誤讀成永久不變。**機制／性質足夠；保留可觀察失敗的用途足夠，安全快照未知／不承諾。**
5. **004-006：合併鍵與有限示範。** (len, digest) 的 tuple Eq/Hash 條件先教，使用候選記錄的 owned map 與首次 Err 的政策明說。篩選 5-byte 只展示先前已知候選元件，不冒充全掃描工具；不是每次都對所有檔兩兩比較。**表示轉換與用途足夠。**
6. **004-007：A/B/B 揭示必須搜尋各組代表。** 摘要相同只是手工受控鍵，不宣稱實測真碰撞。b1 與 a 不同後仍與 b2 比較，才加入 B 組，真正呈現「只比桶第一份」會漏掉的關係。native path unit marker 去重，ContentGroup 只保存借用；每組至少一位的 invariant 支持 index 0。新 singleton 確認多讀一次的成本及不依賴 caller 的原因明說，沒有用摘要代內容判準。錯誤不當不同內容、不交未完成群組；保存確切失敗者、確實移除後重試才能有限進展。**每項設計均有對應理由；完整繼續／報告 CLI 合理留後續專案。**
7. **004-008/009：量的含義與驗收。** 量是按不同路徑的 (count−1)×len，空／零／一路徑零；hard link 每一路徑仍算，明確不是磁碟可回收量或刪除建議。TryFromIntError、checked_mul/add、u64::MAX 與 hard_link fixture 的契約在需要前補足。A/B/B 練習預期 1/2 組及 3 bytes，排除把整桶算 6 的錯誤；b2 先到仍排 b1/b2，回收 native sort。全部必要規則在折疊答案外。**答案含義、操作、例子與支持範圍足夠。**

## source、輸入小變化與累積負擔

完整 source 的核對包含四個 package 的 manifest、library/main、正常 examples、答案、feature integration test、build-script test、scan test、streams test、files/groups test，以及唯一 cfg! 失敗 fixture/expected。本次恢復後重點覆核所有 integration tests 及新 Reader 的狀態變化，沒有從測試答案回補學生正文。

- `tests/streams.rs` 有多種正數短讀取上限、9001 bytes 跨 buffer、最後 byte 改變、少最後一 byte、空來源、非 UTF-8、Interrupted 後 PermissionDenied，及左右讀取失敗。它們測的是已教的切分／完整 bytes／錯誤身分，不是只重述固定 ABCDE 輸出。
- `tests/files_groups.rs` 有真 (len,digest) 鍵、首次失敗無 partial map、輸入到達順序改變後的路徑排序、空候選、真檔內容、可見變動、手工同摘要 A/B/B、重複原生路徑不加倍、確切失敗者移除後重建其餘有效組，以及 checked 溢位／hard link。已將最後一項的平台限制保留，沒有算其他 OS 已執行。
- `tests/scan.rs` 將不存在路徑造成的 walker Err 放成功間驗收，另覆蓋空／隱藏／root file／root missing／link／非 Unicode；這些新 fixture API 在 003-006，測試本身沒有未教的輔助捷徑。
- 以上是實讀 source 的契約及用途核對，**本輪未重跑 Rust packages 或 all-workspace**。CONT 沒有需要以更多執行釐清的技術疑點，避免把重複跑測試當銜接證據；執行正確性另由 TECH/root 負責。

以下是本審閱者依已教規則提出、只作核對的小變化，不是作者既有的新例子或已執行 probes：

| 小變化 | 預測與最短已教理由 | 覆核位置／範圍 |
| --- | --- | --- |
| root 設 dependency，另一 member 不採用 | 不能因此使用該 dependency；共同來源只讓成員選用 | 001-002。可用性不是 workspace 成員身份提供。 |
| 關掉 batch，但照原樣買 80×2 | 單筆仍 160，批次函式與呼叫 block 被排除 | 001-003/004。feature 不改原 subtotal 規則。 |
| 建好 practice 成品，直接啟動時改 LESSON_MODE=demo | 仍 practice；script／env! 已在建置固定文字 | 002-002/003。不拿尚未建出的 demo 成品替換條件。 |
| root 改成一般檔案 | RootError::NotDirectory，不是 Ok(empty) | 003-002/004。root 工作需求與成功空目錄不同。 |
| 收集序列 Ok/Err/Ok | 兩筆成功＋一筆 issue，後筆仍處理 | 003-005/006 及 scan test；不能由此推知打不開的目錄內有什麼。 |
| 左右交同 bytes，但各次短讀取上限不同 | fill_chunk 對齊比較段後仍相等；read n 不同不等於內容不同 | 004-004。有 EOF 的有限來源、正數上限與非空內部 buffer。 |
| 相同 digest 的 AAA/BBB/BBB，或重複輸入同一路徑 | 前者 1/2 真群組；後者只算一次 native path | 004-007/009。完整內容比較與原生路徑去重是不同判準。 |
| 兩個 hard-link 路徑，各觀察 3 bytes | 按路徑量 3，仍無實體可回收量結論 | 004-008/009。量的定義不從輸出反推。 |

階段順序可追蹤：9.1 設定控制→9.2 編譯前固定資訊→9.3 執行期檔案觀察→9.4 bytes 內容工作。9.2 明說是另一 package，不會改 9.1 工作；9.3 從單檔擴目錄樹且先只取 metadata；9.4 才讀 bytes，先候選再確認。數字商品例與檔案例的轉換目的有交代，不是默默更換同一任務。

9.4 內容密度高，但必要限定各回應相鄰誤解：成功 metadata≠內容不變、摘要相同≠內容相同、短讀取差異≠內容差異、Err≠false、路徑重複量≠磁碟回收。沒有為了通過而刪掉限定或要求新演算法。片段、正常例與可控測試沿用同一組 roles；沒有必須開折疊答案或外部閱讀才能接起的必要規則。

## 親看原位圖、頁面與必要圖缺漏

親看 `work/ch09-continuity-v5/screens/` 的全部 28 張固定預覽截圖：四頁各有桌面／手機頁首及收束；三張 SVG 各有兩種尺寸原位；三個答案 fold 各有兩種尺寸展開。視窗為 1280×900 與 390×900，device scale factor=1。DOM/開關紀錄見 `visual-metrics.json`，包括初始 fold 的 closed 狀態、展開後 code 尺寸、圖位置與相鄰圖說。

| 圖與所在位置 | 必要對應／追蹤 | 實際視覺判定及 asset SHA256 |
| --- | --- | --- |
| build-phases，002-002 | 1 編譯 build.rs；2 host 執行讀 LESSON_MODE／交 TEACHING_MODE；3 主要 crate env! 固定；4 成品不重讀。向下箭頭只表示先後，資料從第二框接第三框由圖說明說 | 桌面／手機顯示 340×570；框、箭頭、24px SVG 文字完整。就在時間分工文字後，沒有搬到頁尾或折疊。`1a69b9cb08e010bb93d8827c459a3dbdb6df9d5a4da404bc9924211a913a569f` |
| short-read-comparison，004-004 | 同一 ABCDE，左 ABC/D/E 與右逐 byte；bars 是原 read 切分，不是內容；各填四格或 EOF→ABCD/ABCD，再 E/E | 桌面／手機顯示 340×455；兩方與兩段可定位，全部字線可讀。相鄰圖說說明 4/4096 比例與剩一格只收 D，沒有靠放大才辨識。`ef3f5de75db2c695eec994cac60568da088faee1a9e792f08c69ceb6acc34491` |
| candidate-content-groups，004-007 | 上框是手工同摘要桶 a/b2/b1；中框 b1 先與 a 不同、再與 b2 相同；下框才是 a 與 b1/b2 真群組 | 桌面／手機顯示 340×480；名稱到群組的追蹤及第二次比較都可見，圖前後文字直接點名關鍵作用。`3cd8b006d9184d9b6001ec567bc8154ad811d0466cbb0595c47dec9aad27fe82` |

三圖都有明確角色／動作及當下用途，不只是架構裝飾。手機沒有整頁橫向溢出（DOM document width 390）；長 source/命令僅由 code 區域橫向捲動，有可見 scrollbar。必要型別參數如 `HashMap<u64, Vec<&FileRecord>>`、`ContentGroup<'a>` 等在頁面保留，未被當 HTML 隱去。頁首、表格及收束導航沒有把必要規則藏在 fold；展開／收合保留答案的參考用途。

也檢查沒有圖的部分：9.1 根／成員依賴由 path 起點、兩份設定與 feature 對照直接建立關係；9.3 link 定義＋root/內部政策表、檔案記錄／錯誤模型及受控序列已能定位當前對象；9.4 長度／摘要／確認三判準表及 borrowed/owned 型別說明足夠。未發現必須讀者自行補畫才能理解的必要位置或資料流，故不只以「已有三張」判通過，也未為增加圖數提出額外圖。

## 可選改善、未知與邊界

### CONT9-O1：兩處開場粗體標記未生效（可選版面改善）

- 原位：002-001 的 `**Cargo 先執行一支 Rust 程式，把結果交給接下來的 crate 編譯。**這支...`；004-001 的 `**哪些不同路徑的檔案，讀出的 bytes 完全相同？**檔名...`。
- 依據：桌面及手機 `09-02-build-scripts-*-top.png`、`09-04-content-confirmation-*-top.png` 顯示 literal `**`；文字內容仍完整，後面時間圖／判準表也照常可讀。這是實頁發現，並非從 Markdown 猜有問題。
- 收益／最小建議：調整 closing delimiter 周圍的分句／空白，或改用普通文字，讓原本重要的新步驟與原問題沒有排版雜訊。修後只需核兩處 DOM／畫面，不需要補 Rust 概念或重跑 workspace。
- 分級：可選。失去的是強調與整潔，未隱去問題或設計角色；不能因此把已完成的概念解釋判為未教。

### CONT9-O2：折疊答案 code 比一般 source 小（可選閱讀改善）

- 原位：002-005、003-007、004-009 的答案 fold；實看六張 `*-fold0.png` 及 DOM 開關。
- 依據：一般 code 的 computed font-size 為 12.8px，fold 內為 10.9227px。手機 fold 內 code 寬 326px，比一般 source 可用寬度小，長行仍可捲動；不是只有不存在的圖字太小。
- 收益／最小建議：由共享樣式考慮讓展開答案的 code 至少維持一般 code 字級，降低邊看邊比對答案時的視覺負擔；不另複製 source 或增加 API。此項涉及共享 theme，應由 root 評估是否統一處理，本文未改設定。
- 分級：可選。折疊外已給足必要規則，展開的程式仍有 scrollbar/copy；目前沒有用「必須開答案或放大才知道主線」作通過依據。

### 未驗證及合理留後續

- 未執行跨平台／cross compilation；host/target 文字关系與 conditional fixture 的來源範圍可審，不將 Linux 示例或 cfg(unix) source 當 Windows 已跑。未知不是先備缺陷，也不算本輪實跑通過。
- 未測量長度／摘要候選安排的速度、峰值記憶體或 BLAKE3 效能；正文沒有承諾這些實測效果。已驗的是工作分配理由與可見行為。
- 未驗證對所有檔案變動的一致快照或安全性；正文明確是盡力觀察並列限制。讀後再改長度的 fixture 只支持該觀察能揭露變動。
- 本輪以全部 BODY/source 與 28 張原位截圖完成 CONT；沒有逐個 viewport 捲過整頁每個 source 行，也沒有用真手機、點擊外部延伸閱讀或逐個操作 code-copy。這些未查項不能改稱完全首讀或全面 UI 驗收。
- OUT_DIR 生成／完整 CLI 報告政策合理留後續，當下 source 不需要未教的生成 macro 或最後專案程式。未讀後章，不對其內容作承諾。

必要待修項：0；可選項：2；上述未知保持未驗證。封存此原初判後不讀同行、不修學生檔，等待 root 開放 cross。
