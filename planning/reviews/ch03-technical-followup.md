# 第 3 章技術交叉與 v4 局部複查

審閱者：`/root/chapter04_traits`。日期：2026-10-14。這份紀錄在技術初判封存後追加，不改寫原始判斷，也不是新盲讀或整章重新驗收。

## 初判與新增可見材料

原初判：`planning/reviews/ch03-technical-v3.md`，SHA-256 `3055833111ababcb645559ca99ea918d44fc4ef63af9749322dff828aa77c867`。本輪開始前重核，指紋保持不變；原初判副本也留在 `reviewer-ch03/technical-v3/`。

在初判保存後，依 root 指示實讀 `ch03-first-reading-v3.md` 的完整 32 單位紀錄與 `ch03-disposition.md`。第一個合併工具輸出截斷了後段，另開 195–339 行補足，不將截斷部分算已讀。沒有開仍在獨立進行的 continuity 初判。

因此下列分級比較受到 first-reader 與 coordinator 已存紀錄的提示；不將互看後的同意回寫為自行首次發現。

## 與 first-reader 原判的差異

| 焦點 | 主讀者原始證據／判斷 | 技術初判與本輪裁定 |
| --- | --- | --- |
| 3.4 首次 String 指派，原 `004-003` | Q3 留下「此示例沒再讀 greeting，當下不靠它證明原名不可用」；到 `004-004` 才追加 String 沒有 Copy 的理解。原首讀沒有把它列為必要缺口 | 觀察到後文才確定這個型別的交接行為，與技術初判所指位置相同；但必要性不同。本節以誰管理資料為主題，首次指派已發生，需要在當下知道它採 move 或 Copy，不能以沒有再用舊名暫免這個使用契約。保留 first-reader 的原分級與證據限制，不宣稱它獨立報過 N1。root 已採納前移說明 |
| 3.6 heap／空 Vec，原 `006-002` | 主讀者理解為標準同型元素清單、空 new、元素連續 heap，沒有要求判配置成本，也沒有報缺口 | 技術 I1 原本就是可選精確化。本輪未因主讀者沒報或 root 採納，就升級為必要項；root 用本節元素與 new 的窄限定處理，沒有引入 ZST／allocator |
| 其餘主線理解 | enum／結果形狀／泛型／文字分界／清單 ownership 均有逐段原句與結果解讀；尚未讀完整答案、實跑程式或實頁 | 與技術初判的主要通過範圍相容，但不能將主讀者未做的來源／執行檢查說成已驗證。沒有因互看新增必要技術項 |

主讀者讀第 1／2 章的歷史版本與本技術角色採用的 v8／v4 不完全相同，原記錄明說未取得第 1 章 v8 的全部修句。本輪不補寫為同一先備版本；上述比較聚焦第 3 章固定來源及其當下留下的理解，不作兩角色逐字同背景測試。

## 修後固定版本與來源界線

修後：`/workspace/work/tutorial-implementation/frozen/ch03-review-fixed-v4`。manifest SHA-256 `87a9c31a900f1094c89be7e56a9511f87b3a07479c73ec8d5a391b7ce18c195e`。

本輪只實讀受影響的三個正文單位，核對它們的前後過渡：

| 單位 | SHA-256 | 實際複查範圍 |
| --- | --- | --- |
| `004-003.md` | `a7182c1c09414e96fa742268361f01eedf84be6a4ee835d07257ca0d4053438d` | String API／第一次完整程式之前的 no-Copy 說明；既有 source、輸出、原位圖說與借用結束後追加的連接 |
| `004-004.md` | `c7dfc1bf8c2e3b1d3d5cf4f2b1af43be22e22424588408c78309b0efe6f0cd27` | 承接已說明的指派，再說管理／heap 不另複製及兩個既有失敗契約 |
| `006-002.md` | `7af798de5b90e16bd7118d17056253478481897fca8c03a1c3bbeb4e0ddff999` | Vec 的本節元素儲存／空 new 限定、操作表與原正常／失敗程式 |

v4 manifest 的 37 份 archived sources hash 集合與 v3 完全一致，也逐檔核對 v4 archived bytes；Rust 正常例、答案、測試、fixtures、package manifest 與兩 SVG 都沒有變。這次只改正文說明，不用可變 current source 代替固定證據，也不重跑無改動的全套 package。

## 逐項局部複查

**N1 已在需要之前補足。** `004-003` 的第一份完整 source 前，現在明說「String 沒有 Copy 能力」，指出最後 `let moved = greeting;` 將 ownership 交給 moved、不能再從 greeting 使用該 String。前面的 show 仍是 as_str 借用。兩種操作都由實際前文的 move／Copy／reference 機制接得起，新句只補 String 的具體能力與本次交接對象，沒有要求 Clone、derive 或更深 memory layout。

`004-004` 改為「前面最後的指派把管理責任交給 moved」，再解釋 move 不要求另複製整份 heap 文字，順接 E0382 與借用衝突。沒有重複一大段能力介紹，也沒有把後文說明當成前面已教的替代。局部修改回應原缺關係，N1 在 v4 這個範圍內關閉。

**I1 已以窄限定處理。** `006-002` 現在把「配置的元素空間位於 heap 且連續」限於本節 i32／String，並明說 `Vec::new()` 先建立空容器、不預先配置元素空間。它不把已有 Vec 清空等同歸還所有配置，也沒有加入本節無用途的零大小型別或 allocator。仍可用後面的 new→push、pop／remove、slice get 的既有流程理解資料存放；I1 的精確化已達到，不增加新 code 前置。

局部複查沒有發現新增 API、數值結果或借用時序問題。原正常／失敗 source 沒變，原技術初判其餘通過項與未查界線不改。

## 尚未驗證與後续交叉

未新執行任何 Rust 程式、fmt／Clippy／完整測試；原初判已有 Unicode 局部實驗，不改稱這次重跑。未親看實際桌面／手機版面，兩 SVG 未改的來源關係不代表新畫面通過。

本輪尚未開 continuity 初判，待其獨立保存後再按 root 指示交叉。結論只涵蓋 N1／I1 的 v4 三單位局部複查，不宣稱第 3 章全部 QA 已完成。

## root 呈現查漏提示後的 v5 追加

在以上 v4 核對之後，root 另告知其實際 HTML audit 找到三處未以 inline code 包住的型別參數，並在 disposition 追加來源與時點。這是 **root 的新提示**，不是本技術初判的自行發現；沒有傳給仍在獨立審閱的 continuity 角色。v4 上述紀錄保留，以下只追加 v5，不將 v5 回填成 v4。

新固定版本 `ch03-review-fixed-v5` 的 manifest SHA-256 為 `5fcf421fbb1071af5a72e117bdae119e6a46f8984d15900f66e2de0c0058439b`。與 v4 對照，實質變更是：

- `006-002` 的 pop 操作表將 `Option<T>` 加 code 標記；chunk hash `4d9240a8052a9362bd501952e6f73e11b88d8e167f31e176f21293c2025cfec5`。
- `006-003` 的 String ownership 段將 `Vec<String>` 加 code 標記；chunk hash `6dedb66d897ebca64d5acdd758309023f344ef15978d19bac03e21ed1560a535`。
- `006-004` 的成果入口將 `&[i32]`、`Vec<i32>` 加 code 標記；chunk hash `c92f6babe3fab5f0acbc966e732e378b4aa736ef0996ac3ccb618bb2d7b59846`。

`004-003`／`005-002` 的 chunk hash 也因 snapshot 的原位 asset 路徑改為 v5 而變；圖 bytes 與文字教學契約未改。N1 的 no-Copy 前置、I1 的空容器／已配置元素空間限定都保持 v4 的內容，`004-004` 也未再修改。37 份 sources hash 集合仍完全相同。

本角色實讀變更的原段與上下文，並用專案固定依賴中的 Markdown parser 將 v4／v5 這三個 chunks 各自解析（tables／fenced_code）。v4 可重現 raw `<T>`／`<String>`／`<i32>` 被當成標籤；v5 的對应詞變為 escaped code 內容，完整保留型別參數，不再產生 t/string/i32 標籤。除了標記，回傳 `Option<T>`、保存 `String`、輸入借用 slice／輸出 owned Vec 的原契約沒有變。

局部 parser 結果與生成 fragment hashes 保存於 `reviewer-ch03/technical-v3/v5-markup-checks.json`，片段 HTML 同目錄存檔。這是本角色親自做的解析核對，仍不是親看實際站點 DOM 或桌面／手機畫面；root 的實頁證據由 root 另存。不重跑 Rust，不將這個提示後追加算入原技術初判的獨立發現。

v5 的受影響型別文字已完整，N1／I1 的局部關閉狀態延續；其餘原初判與未驗證範圍不變。

## continuity 初判封存後的交叉與 v6 追加

日期：2026-10-14。root 確認 continuity 初判已獨立封存，才開放本角色交叉。本輪實讀 `ch03-continuity-v3.md` 全文，包括實際依賴、需求／機制／用途、manifest 必要發現、char 可選發現與其獨自實頁檢查的界線。沒有將這些互看後的新提示回寫到本技術初判，也沒有改變原始發現數量或分級。

continuity 同樣有先前為寫作讀第 3 章的歷史，並明記第 2 章作者身份；其原判不是新的無背景盲讀。以下使用 `C-N1`／`C-O1` 指 continuity 自行發現的兩項，避免與技術原 `N1` 混淆。

| 互看焦點 | 原分級／差異 | 本輪技術覆核 |
| --- | --- | --- |
| C-N1：manifest 的 `[lints.clippy]`／`manual_map = "allow"` | continuity 列必要的小型完整參考包設定依賴；本技術初判沒有報出，未因曾讀 manifest 就稱已獨立注意 | 同意這是使用者要求中完整 manifest 的前置契約。知道 Clippy 與 TOML 區段不能推知這項提醒被關閉、為何保留 match；最小補角色即可，不需要提前教 map／closure |
| C-O1：char 第一次 `{}` 顯示 | continuity 列可選的具體能力補充；本技術初判未列 | 同意維持可選。已有 println! 使用方式與完整輸出可跟上主線，補一句能保持標準型別介紹一致，不升級成 Unicode 必要缺口 |
| 技術 N1：String 第一個指派前的 no-Copy | continuity 原表將 `004-004` 的 non-Copy 說明列足夠，沒有另報 `004-003` 的前置時點 | 與 first-reader 的原差異相似，保留角色原判。本輪仍按首次發生指派時必須知道具體型別能力的技術理由，確認已採納的前移持續成立，不重寫 continuity 原分級 |
| 技術 I1：空 Vec／已配置元素空間 | continuity 把操作與儲存主線判足夠，未提出配置成本的精確化 | 保留本技術原可選分級，不因 root 採納而升級。修後限定沒有增加未教布局或 allocator |

修後固定來源為 `/workspace/work/tutorial-implementation/frozen/ch03-review-fixed-v6`，manifest SHA-256 `45146f5eafd8d8b3af1cf74b5cb1f3cb7e173c9f04b26108a06d060c1f02ccc7`。實讀兩項受影響材料與其首次使用處，也回讀技術 N1／I1 的局部：

- archived `examples/ch03-data/Cargo.toml`，SHA-256 `633e1d3caf2ea700e3d6cb4495696f93e054532907ec43243221721a9c5f5ed7`。四行註解在設定之前，先說 TOML `#` 註解不改設定，再解釋 table 選 Clippy、manual_map 是 match 簡化提醒、allow 只關閉這項以保留本章已教 match。其餘檢查仍照常。沒有要求讀者先懂 map 的方法或 closure，C-N1 所缺工具角色已補足。
- `005-002.md`，SHA-256 `6b55ed58cd8f212671fab918c95d4fd96ecc1306c5c1b7ec97f65ca8272c47f1`。char／literal 說明後加入支援 `{}` 顯示其文字，再到 `005-003` 的第一次 `println!("一個 char={marker}")`。它只補實際使用的能力，沒有把 char 等同字形或提前 Display trait。C-O1 已採納且位於需要之前。
- `004-003` 保留先於完整程式的 String no-Copy／ownership 交接說明；與 v5 正文除 frozen SVG 路徑外完全一致，`004-004` 未改。原技術 N1 關閉狀態延續。
- `006-002` 的 i32／String 已配置元素空間限定、new 空容器與 `Option<T>` code 標記都保持 v5；`006-003`／`006-004` 的型別 code 標記也沒有改。原 I1 與 root v5 呈現修正均持續成立。

逐檔 hash 核對 v6 manifest 的 32 chunks／37 sources，與 v5 相比 archived sources 僅 Cargo.toml 的註解變更；所有 Rust／答案／fixtures／SVG bytes 未改。以 Python TOML parser 比較兩版 manifest 的設定物件完全相同，沒有改 Clippy 行為。局部證據保存 `reviewer-ch03/technical-v3/v6-local-checks.json`。這是設定與來源核對，沒有重新執行 Rust 或全套 Clippy。

本輪沒有親看 v6 桌面／手機頁；continuity 原判中親看的 3.4／3.5 圖與相鄰文字屬於其自己的證據，不轉作本角色實頁成果。也沒有重跑 v5 Markdown parser，因相應 chunks 與 code 標記不變。結論僅為上述交叉與 v6 局部修正通過，沒有宣稱全章發布驗收或新的首次閱讀完成。

原技術初判指紋仍為 `3055833111ababcb645559ca99ea918d44fc4ef63af9749322dff828aa77c867`，v4／v5 追加紀錄與各自未查界線保留。本輪第 14 章研究已另存 `planning/source-notes/ch14-author04.md`，只有實讀來源／候選 API／工具契約，沒有開始正文或工具執行；本角色完成後停止，待 actual gates 與 root 後續指示。
