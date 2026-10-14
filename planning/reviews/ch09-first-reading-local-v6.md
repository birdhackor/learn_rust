# 第 9 章 v6 兩段局部覆核

本紀錄是 AI 模擬讀者的字面修後局部重讀，不是 original FIRST9，也不重做全章。沿用本人原第9章 v5 已讀27單位至004-009的 checkpoint；讀取前只核 own counter 與原紀錄 SHA，未重讀已封存原判。當前背景為基本 Python、本人實讀1–9章正文與已記錄局部補讀（含8v9指定六段）。不以模型原有 Rust 知識或他人答案填前提。

唯一新正文為 `frozen/ch09-review-fixed-v6` 的 `002-001 → 004-001`。只收到指定位置與字面修訂範圍，沒有預期結果；按 original 同一 visible_filter，在工具輸出前過濾 closed 答案，使用獨立 ch09v6 state／ledger，先保存四問、Q2機制／用途、原句與阻塞判斷，才開下一段。每段 raw／visible hash 隨當下紀錄保存。

原 FIRST9 分類及 record SHA `7feff159c1a6f23bc9e61c53bf0893fd261c7a624b11a39cbd7d32dc4e91740d` 不變；局部理解不回填 original。不得開 peer9／root disposition／作者／archive／source／live／未指定 BODY；不要求展開答案，未做實頁字體核對或 Rust／Cargo 探測。兩段保存與封存後停下。

### 002-001 — 建置設定、build.rs 位置與執行階段

Q1：建置時決定的設定與每次啟動時重新讀環境分開；build script 是 package 根目錄的 build.rs，不是執行程式 src/main.rs。
Q2a（機制）：Cargo 先執行一支 Rust 程式，再把结果交接下來的 crate 編譯；本段明示這個時點，不在此重展開交付 API。引文：「把結果交給接下來的 crate 編譯」。沿用本人已讀9.2背景理解，未假稱核過 v6 其他段。
Q2b（用途／需要）：練習版要保留自己建置時選的 demo／practice，多次執行回報同一設定；不讓啟動時環境改成品身份。引文：「不是每次啟動都重新讀環境」。
Q3：在另 package ch09-build-script 的根位置放 build.rs，相關 Cargo 命令由 repo 根執行；與9.1工程 package 分開，當下沒有要求未知的新操作。
Q4：demo／practice 的固定身份例，加上兩個檔案位置和 package 路徑，足以辨認誰在何時工作；source 中檔名表示可讀，但我未看正式頁面字體或執行結果。
改後理解／阻塞：導入的建置前工作與身份用途清楚，沒有必要新猜測或仍有阻塞；具體交付步驟不在本次指定段內，不冒稱重新核全節。未新增 necessary／optional，original 分類不改。

原始SHA-256：ececaf57fdcbca7853f987a02f84964295a4bcad215ab2506cce1aa07847db4a；visible SHA-256：ececaf57fdcbca7853f987a02f84964295a4bcad215ab2506cce1aa07847db4a。

### 004-001 — 大小、摘要、完整 bytes 的分層判準

Q1：原生路徑／metadata 是已觀察記錄，現在問不同路徑讀出的完整 bytes 是否同；三個步驟各有不同證據範圍，不把檔名／長度／時間當內容答案。
Q2a（機制）：異長可排除，同長仍可能 AAA／BBB；異摘要可排除，同摘要仍需直接比完整序列。引文：「摘要相同仍是候選，需要內容確認」。完整比較也不保證外部程式之後永遠不改檔案。
Q2b（用途／需要）：利用已有 metadata 先排除單獨大小，再讓直接比較集中較小候選桶，減少不需要的兩兩重讀；文字只說工作分配，不讓讀者推定測過特定倍數效能。
Q3：單份6-byte 無同長對手，先排除；其他可配對者計摘要，同鍵仍待 bytes 確認。package ch09-content／library ch09_content／上一節依賴及 repo 命令位置均明列，當下不要求新 API 猜測。
Q4：AAA／BBB 同長反例與三欄表可直接核對各步能／不能判定之事；本段沒有實際測試結果或速度資料，我沒有把它當實跑證據。
改後理解／阻塞：候選、讀出的內容與往後磁碟狀態三個界線清楚，沒有仍有阻塞。只交有限元件、完整參數／報告／部分失敗政策留練功的限定亦足夠；不以未指定 v6 後文补判。未新增 necessary／optional，original分類與closed答案紀錄保持。

原始SHA-256：ccfcb6bc2147d2e1284557c0edca133e7a8ebd8adffacb31bb27163905294d92；visible SHA-256：ccfcb6bc2147d2e1284557c0edca133e7a8ebd8adffacb31bb27163905294d92。

## 兩段局部封存

兩段按 002-001→004-001 逐一過濾、保存四題／Q2兩部分與 hash，再開下一段；新 gate／ledger 僅兩筆，closed計數皆0，無原位圖 hook。原本 closed 答案沒有展開或改紀錄。

局部結果：建置階段固定設定／script 的位置，以及大小→摘要→完整bytes的分層判準，字面皆可理解，無仍有必要阻塞、無新增 necessary／optional。用途与現有前文相接，沒有新增必須猜的 API／操作。這是有指定修後位置的局部理解，不算 original 重現，也不改原分類。

original FIRST9 record SHA保持 `7feff159c1a6f23bc9e61c53bf0893fd261c7a624b11a39cbd7d32dc4e91740d`；sidecar SHA保持 `6b2dc0c526547bbeb893b97c94f9ac1e38d524cdab024136d9630d9fee8f7e32`，39項檔案hash全部相同。核對只讀其file/hash metadata，沒有重讀封存原判正文。

通過範圍只為v6兩段 BODY 的字面與當下理解。未看其餘新版正文／展開答案／peer9／root disposition／author／archive／source／live；未核正式實頁字體、圖、實際命令或編譯結果。封存後STOP，不擴其他BODY或後章。
