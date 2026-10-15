# 1.7–1.8 獨立技術初判與 1.3 metadata 複核

本輪未發現必要技術缺陷、範例載入缺口、需要修正的 SVG source 對應錯誤或另外提出的可選項。僅完成靜態技術審閱；沒有把程式推算、SVG source 或 XML 可解析性當成執行／版面實測。

## 範圍、獨立性與版本

- 完整重讀 `01-07-lesson.md`、`01-08-lesson.md`，逐一讀四份成功 target、runtime-fail fixture、五張 SVG 原文與必要 manifests。讀者可用背景為基本 Python 加實際已讀 1.1–1.6，不使用後續章節知識補缺。
- 封存前未讀作者 ledger、首讀紀錄、root 其他結論、歷史問題或審閱。只用自己已封存的 prefix JSON 比較 source hashes；這是技術整頁重讀，不是盲讀、目標讀者實驗或 browser visual。
- 兩頁 SHA-256 分別為 `017a1fa288e5545af8878f752fc7b4730ca639df1025ba8a90c4e5ffc41fc49d`、`4d1f23cea4daf930e04075fe08f544226142f8e774b20c7405d4003afa7c03cd`，與此次 root 指定版本相符。完整 source 指紋保存在同名 JSON。
- root 正在 browser 預覽，章 2 作者使用 Cargo，因此本輪未執行 Cargo／rustc／fixtures，也沒有重跑 prefix 已通過的 12 個命令。

## 1.7：執行模型、frame 與 heap

| 檢查點與原文 | source／技術核對 | 判定與讀者影響 |
| --- | --- | --- |
| 7–15 行區分原始碼檔、執行檔、process、RAM、CPU。11 行說明啟動箭頭與材料／結果的兩種方向。 | `program-process.svg:18–69` 由儲存裝置的 .rs → 編譯 → 執行檔，再啟動到 process P1；RAM 含 120、3、360，CPU 框含 120×3→360；兩條材料／結果箭頭方向一致。 | 角色模型正確且與圖的標記相對應。沒有宣稱重新指派會改寫 .rs，也沒有把修改 source 等同更新已存在的執行檔。這是角色示意，實體暫存空間的 caveat 另見 64 行。 |
| 19–27 行把 bit、byte、型別表示大小、值與位置分開。 | i32=32 bits=4 bytes 正確。`i32-value-size-location.svg:18–74` 分別標示 P/main.price 與 Q/subtotal.price，各有四個 1-byte 框與值 120；76–83 行比較值 80 仍是四 bytes。 | 不用十進位位數推測儲存大小；相同數字與不同位置也有明文區別。P/Q 是示意位置，不偽稱實際印出的地址或二進位內容。 |
| 31–64 行從已完成的 subtotal 程式追蹤 A/B/C，並解釋 `//` 註解、frame、stack 及模型界線。 | `memory.rs:1–17` 的 A 在呼叫前；B 在 result 算好且回傳前；C 在 first 收到回傳後。第一筆輸出 B=360、first=360；第二筆依次 B=160、second=160，與正文相符。 | 每個呼叫有本次的參數與結果；數字由回傳值交到 main，不需要保留已結束的 subtotal 工作區。圖沒有把尚未初始化的 first 畫成 0。 |
| `subtotal-frames.svg` 的三個觀察點與箭頭。 | SVG:18–31 的 A 只有 main.price/count；37–59 的 B 同時保留等待的 main 與 subtotal.price/count/result；65–79 的 C 只保留 main 與 first=360，並明說 subtotal frame 已結束。A→B 虛線是時間，B→C 實線標示回傳數值 360。 | 數字、呼叫身分與三階段一致，沒有靠相同的 120 判定是同一個存放。64 行明說可能使用 CPU 暫存或省掉資料，不保證逐格實體儲存；block 不等於新函式 frame，scope 也不等於存放位置。 |
| 68–78 行轉到不同內容量的明細，解釋 allocator、配置、pointer 與 heap。 | 明說前面數字程式沒有為這些數值額外配置 heap，且新圖是資料關係、後面才用 Rust 型別操作。`heap-location-content.svg:18–66` 是兩項 H1 與另一次五項 H2；定位框、內容框與「沿位置找到內容」箭頭均分開。 | 動機是每次要保存的內容量不同；設計把定位資訊與整份內容分開。這個抽象沒有冒充只有 pointer 的完整 Rust 容器布局，也沒有要求此處理解尚未介紹的 Vec/String／metadata。78 行保留固定大小也能放 heap 的事實，不把 i32 大小當存放位置判準。 |
| 82–92 行追蹤 H1 交接、釋放與使用期間。 | `heap-lifecycle.svg:18–104` 的 T1/T2/T3 反覆標示同一 H1、相同 120/80：由內層定位、交給外層、內層結束仍可用。T4 沒有有效使用箭頭，原空間標示可再次配置，107 行說不是清零。 | 教的是內容使用期間可以跨過建立它的呼叫，沒有宣稱 heap 一定活得較久；88 行另給內層用完就歸還的情況。90 行說明不能以舊定位繼續使用與清理責任，92 行區分 compile-time 檢查與 runtime 配置／歸還。 |
| 96–113 行完整練習與答案。 | `memory_answer.rs:1–15` 完整定義 subtotal/main，只有 80、2 的一次呼叫；B 與 first 都是 160，型別仍 i32。 | 修改的是值，表示大小仍四 bytes；回傳此數字不需要新 heap 配置。完整 source 不依賴另一檔的函式或未教 API。 |

### 主要概念四題（技術重讀）

- **值／大小／位置：**身分由 23 行分開定義；機制是型別的固定表示大小不隨本次數值而改變；用途是回答 120、80 與不同呼叫中的值是否等於同一份存放；四 bytes、P/Q 與 80 的比較直接支持這個判斷，不需要實際地址。
- **frame／stack：**身分是一次呼叫的邏輯工作區與尚未完成呼叫的堆疊；機制是 main 等待 subtotal，回傳後 main 收到結果再繼續；需要來源是 3 行「subtotal 已結束，為何 main 還能使用結果」；A/B/C 與完整 source 接起這個問題。實體配置不是這組圖承諾的結論，64 行明定界線。
- **heap／pointer／配置：**身分分別為可請求區塊、找內容的定位值與配置器安排區塊的動作；機制是另存定位資訊、內容獨立放置；需要來源是 68 行兩項／五項明細的內容量不同；H1/H2 顯示保持定位角色而內容量改變，沒有只以「heap 更好」代替理由。
- **交接／釋放：**機制是把找到同一 H1 的資訊交給後續工作，最後歸還配置器而不是清零；用途來源是 82 行外層仍需顯示明細；T1–T4 完成從建立到後續使用、再清理的關係。這只是概念模型，不宣稱已教 Rust ownership API 或任意 pointer 使用方式。

## 1.8：tuple、array、方法與索引失敗

| 檢查點與原文 | source／技術核對 | 判定與讀者影響 |
| --- | --- | --- |
| 7–21 行 tuple、逐位置型別、`.0`、解構與位置約定。 | `tuple_array.rs:3–7` 的 `(i32,i32,bool)`／`(120,3,true)` 對應正確；order.0=120，解構依序 unit_price=120/count=3/ready=true。 | 新句法在片段前交代。固定格數與每格固定型別正確；沒有因前兩格同為 i32 而省掉單價／數量的約定。正式具名欄位可以待教，這裡已給位置的實際含義。 |
| 25–47 行 array、`[T; N]`、索引、len 方法與 usize。 | `tuple_array.rs:11–21` 第一／三項為 120／200；len=3，usize index=1 選到 80。 | element、型別中分號、從 0 起算、方法點呼叫和空括號，以及 usize 的角色均先說明。`[i32;3]` 和 `[i32;4]` 不同型別、同一 array 項數不變正確。未偷用未介紹的 iterator／reference／closure。 |
| 51–57 行 `[0;3]` 與 mutable 元素。 | `tuple_array.rs:25–28` 用三個 i32 零值建立 mutable counts，改 index1 為2後讀回middle=2。 | 建立值與描述型別的兩種分號寫法有明確對照；固定長度不等於元素不可變。此例只有數值重複，不需要先教 Copy 等一般化限制。 |
| 79–95 行超界與 panic；完整 helper 及重現方式。 | `array_index.rs:1–9` 完整定義 selected_price(usize)->i32 和 main；main 傳3到長度3的 array，函式內依執行參數索引，再回傳。沒有 allow、unsafe 或外部 helper。 | 依目前已教句法即可解讀。型別／名稱合法與這次執行索引失敗被區分；3 是第四項，錯誤核心 len=3/index=3，panic 先於後面的 println。93 行保留明顯靜態超界也可能被提早拒絕，不把「能編譯」當成邊界有效證明。 |
| 87 行重現路線與 99–107 行練習完整答案。 | fixture 可完整複製到 1.2 自建專案 src/main.rs，沿用已介紹的完整 manifest；不要求讀者執行未介紹的 fixture harness。`tuple_array_answer.rs:1–12` 自含所有計算，總額160、index2價格200、len3。 | 成功／預期失敗程式的操作路線足夠，沒有編譯未展示的 library 或測試 helper；輸出是靜態推算，本輪未宣稱實際執行。 |

tuple 的身分是每個固定位置可有各自型別的合成值，array 的身分是同型別、固定項數的合成值；兩者的需要來自 3 行「具不同角色的訂單」與「同類價格清單」。正文展示如何建立、按位置取出／解構與計算，不只是列出新名詞。索引檢查的用途由 95 行範圍外資料不能當元素說明；預期失敗 source 顯示索引等於長度為何失敗，不需要未教的條件判斷才能理解此處結果。

## Cargo closure 與 1.3 的小範圍更新

- 同一份 `intro-steps` manifest 仍只有已教的 name/version/edition；沒有 dependency、src/lib.rs 或 build.rs。memory、memory_answer、tuple_array、tuple_array_answer 各自是單檔 example binary crate，沒有 mod/include、自有額外載入檔、測試 helper、attributes 或 Clippy allow。fixture 也只有本頁已展示的完整函式與 main。
- 比較自己 prefix 的 37 個來源指紋，只有 `examples/intro-workspace/Cargo.toml` 與 Cargo.lock 改變。原六頁與所選 source 都未變。
- workspace 現在列 hello/greeting/steps/control-steps/modules-basic/control，resolver 仍為3；新增三個成員的 manifests 也只含 name/version/edition，無額外依賴設定。lock 有相符的六個 package 名稱／0.1.0 版本。
- 1.3:39–45 的 snippet 顯示完整現行 members 清單；原文已說這些是相對成員資料夾路徑，沒有綁定清單必須恰為三項。新名稱是 metadata，沒有使 `-p intro-steps --example ...` 編譯後續 member 的 Rust source。沒有因新增 members 而產生新的必要教學缺口，也沒有為此重跑已過的 prefix 指令。

## 實際完成與限制

五張 SVG 均成功以 XML parser 解析；只核對 source 中的文字、數字、箭頭種類／方向及必要對象的對應。未看實際圖像／browser，不能判定字是否被裁切、手機實際大小或閱讀路徑已通過。圖中座標與尺寸的存在不作視覺證據。

本輪沒有實際執行新增四個成功 targets、runtime-fail fixture 或檢查腳本；沒有擴大到全 workspace、後續成員的 Rust source 或未讀章節。後續路線的章號承諾僅記為導覽，沒有驗證尚未開放的章節是否已履行。

## 封存後追加：1.7 標題改動

於 2026-10-15T06:31:06.660804+00:00 依 root 明示的 heading-only 變更追加窄範圍核對：H1 改為 `# 1.7 計算機基礎：程式、記憶體、stack 與 heap`。當前 SHA-256 為 `bfb2f7bfe932726dd5c3d16a61128105f1783e6bd12b9cef9ca07a9b3f06f36a`。把當前檔案的第一行換回本輪原標題後，恰好重現原封存 hash，證實其餘正文未變；原 tail snapshots 的全部 steps sources 與五張 SVG 指紋亦未變。因此保留原初判，不重查整頁、重跑程式或宣稱新增視覺證據。此項由 root 提示，不算獨立新發現。
