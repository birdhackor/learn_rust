# 第 6 章技術交叉與修後局部覆核

## 身分與版本界線

本輪仍由非第 6 章作者的技術審閱者執行。我曾為第 12 章寫作讀過較早的 6.1–6.5，原初判已記錄這項經驗，這輪更已取得同行提示，因此**不是新的 blind 首讀或獨立初判**。先前初判不回寫；只在本文件追加交叉判斷與修後證據。

依據為 clear-tutorial、review-protocol 與專案術語指引。讀者仍為懂基本 Python、已按順序讀指定正文的人；正常例子、答案、測試、失敗 fixture 與設定的首次需要採同一標準。不能以失敗 fixture 本來就不會編譯豁免先備，也不能以執行成功代替方法理由。

| 資料 | SHA-256／狀態 |
| --- | --- |
| 原始 v6 manifest | `dacb0f09813d368bcdb9c7bf94b23a618c29cb6b29da1d5f55fb37aa39fdfb0b`，未變 |
| 原 `planning/reviews/ch06-technical-v6.md` | `ecfc097d2a6f0e006cd1740c3da4ee3addd53ec6a93838a79a8bfe42edaecb55`，未變 |
| `technical-ch06-v6/ch06-technical-v6.initial.md` 封存副本 | 同上，未變；沒有修改 initial JSON 或 SHA sidecar |
| 唯一修後 actual：`frozen/ch06-review-fixed-v10/manifest.json` | `d43db1810524ae6858774b65594eae53c367305bbb50d30ca6739dd928e567f6` |

以上 frozen 與封存副本均位於 `/workspace/work/tutorial-implementation/`。本輪重新核對 v6 與 v10 各 50 個 chunk、80 份 archived source 的實際 bytes，均吻合各自 manifest。80 份 source 的路徑及 SHA 完全相同，包含 Rust、39 個 examples／答案、test 所在 source、15 個 negative／expected、Cargo 與 3 SVG；不是隻接受 root 的「程式未改」摘要。

正文語意變更只有 `003-002`、`004-002`、`006-002`、`006-004`。另 `001-007`、`005-003` 及 `006-004` 的圖片絕對路徑由 v6 換成 v10；圖檔本身未變。沒有以原稿結論宣稱所有修後頁面重新首讀通過。

## 同行交叉：來源與分級

初判封存後才讀 `ch06-first-reading-v1.md`、`ch06-continuity-v6.md` 的對應問題及通過項，以及 `ch06-disposition.md`。以下保留各人的原始發現狀態。

| 缺少的關係 | 原始來源／我的發現狀態 | 交叉裁定 |
| --- | --- | --- |
| FnOnce／FnMut／Fn 的名稱來源與 prelude | FIRST 必要、CONT N1；**我的原 TECH 初判沒抓到**，本輪同行提示後確認 | 同意必要。知道 Clone 或 From 在 prelude，不能推出所有標準 trait 都自動可見；已教 receiver／Output 也不能回答為何這些 bare 名稱不用 use。這不是 Fn 呼叫機制全未教，也不是 Rust 程式不合法。 |
| collect 失敗 fixture 先用整份 Vec Debug | 我的原 TECH N1、CONT N2 分別在交叉前提出；FIRST 沒抓到 | 同意必要；兩份 initial 的獨立重現保留。Debug 語法、Vec Clone 的條件與 Vec Debug 的條件是不同關係。既有段落前移即可解除，無需新 API。 |
| evaluate 一次呼叫卻選 Fn 的理由 | FIRST 可選；我的 initial 未提出，本輪受提示核對 | 同意可選。Fn 能合法支持一次呼叫，參數／回傳選型主線可理解；但本例選較強契約的理由值得簡記。不是所有一次 callback 都應要求 Fn，也不需要為此改動程式。 |
| adaptor 的「不是複製」容易混同按值 Copy 與主動 clone | 我的原 TECH I1；同行提示前已提出 | 同意可選。原例用到的 iterator 本身沒有因此錯誤，最低需要是準確保留已有 Copy／move 規則；不值得把自設 Copy iterator 實驗加進學生正文。 |
| Vec 的 FromIterator 由 i32 例提升為一般 T | CONT I1；我的 initial 未提出，本輪受提示核對 | 同意可選。3.6 的泛型 Vec 與 6.6 的 Item／FromIterator 已能支持後續各種 T；原文不是必然斷點。改一句有助看出後續 char、借用文字、tuple 仍是同一契約。 |

Fn 名稱、evaluate 理由與一般 Vec 契約的交叉同意，都**不補算我的 initial 獨立發現**。處置文件的必要／可選分級與最小修改，均可由原稿與實際先備支持；不因 root 已決定採納而自行升降分級。

## v10 的四處修後覆核

本輪直接讀四個完整修後單位與相鄰過渡：`003-001` 至 `003-004`、`004-001` 至 `004-003`、`006-001` 至 `006-005`。另讀以下方法通過項，及 archived `callbacks.rs`、`opaque.rs`、`adapters.rs` 的實作與 tests、`text_ops.rs` 的排序／報表部分和 collect 失敗 fixture。沒有以 live 改稿或作者帳冊當 actual。

| 修後位置與原句 | 技術、需要與順序的局部結論 |
| --- | --- |
| `003-002:9`：「都由 std::ops 提供，也已在 Rust 的 prelude 中」 | 出現在 `run_once` 的第一個 bound（第 16 行）之前。`std::ops` 與自動引入規則正確；其後 FnMut／Fn 可以沿用，不必重複同一段。接回 4.1 自訂 trait 的 use 規則，不列不相干的完整 prelude。**必要項關閉**。 |
| `004-002:11`：「這組參數／回傳對照先使用同一種 Fn 契約」；「單獨設計這項工作時，FnOnce 就已足夠」 | 前一句交代教學比較保留的介面，後一句保留最低能力的判準。evaluate 的 source 確實只 call 一次；make_adder 仍以 Fn 描述可交回且共享呼叫的工作。沒有拿比較方便反推一次任務必須 Fn，也未新增語法。**可選項完成**。 |
| `006-002:5`：「按值接收原 iterator」；「不主動呼叫 clone」；「仍依來源型別的 Copy／move 規則」 | map 的 self 按值，保存 source 與 callback；此處 clone 的敘述限定 adaptor 的建立操作，不承諾 callback 或原 iterator 內部絕不 clone。與前文 ownership 相容，也不把 lazy adaptor 說成預先整理一份中間清單。**我的 I1 完成**。 |
| `006-004:7`：「標準 Vec<T> 能從 T 元素依取得順序建立」 | 泛型 T 已教；FromIterator 在當下先解釋再以 i32 例示，不需要先教全部標準實作。後面 `Vec<char>`、`Vec<&str>` 與 `Vec<(String, usize)>` 可沿同一規則判定；T 本身是 reference 時仍保存 reference，沒有暗示 collect 一律取得來源內容 ownership。**同行可選項完成**。 |
| `006-004:32` 的 Vec Debug／PartialEq 條件前移 | 現在位於 fixture 引言（第 34 行）及 `println!("{owned:?}")` 之前，也早於正常 adapter_collect 的顯示與整份比較。元素是既有 i32，有對應能力；其後 tuple 能力仍在 6.7 首次需要前另教。移動沒有改變 `&i32` 與目標 i32 不符的 E0277 機制。**我的 N1／CONT N2 關閉**。 |

四處修訂沒有擴大主要目標或新增 helper、語法與範例，不需要改其他章的導覽或重排課程。這個局部範圍未發現新問題。

## 沒報問題的主要方法通過項覆核

不是隻同意同行的缺口清單。另選已通過但涉及需要、比較及限定用途的項目，親讀 v10 正文／相關 source 重建以下關係：

- `003-003/003-004`：原需求分別為保存同一計數狀態做兩次與僅有 shared access。FnMut 的重複 mutable receiver、Fn 的 shared receiver 各有自己的理由；「多次」本身不升格成 Fn。移出 String 的失敗例不能靠 mut 生出第二份；Fn／FnMut／FnOnce 的能力包含關係也沒有變成互斥分類。原正文、call_twice／apply_shared 實作與 test 一致，局部通過。
- `004-005`：刻意未使用的 &str 參數隔離了 edition 2024 的 implicit capture 與真借用；`use<>` 改的是回傳介面允許的依賴，不是延長來源。precise_reader 仍保留真 reference 的 lifetime，具名型別參數的清單限制也有明說，沒有把示例推成任意省略 T。原需求是展示契約差別，與一般 reader 的使用需要分開；局部通過。原 initial 已對這個疑點做相稱編譯／執行，本輪 source SHA 未變，不再重跑。
- `006-006`：原需求是整批成功才交出清單、失敗保留第一個原因。Result FromIterator 的短路正好完成這項政策，而不是從早停產物反推使用者想早停。visits 在 parse 前加一，輸入 10／bad／30 對應兩次 callback，原清單 len=3；來源借用未消耗與局部成功未交回是兩件事。沒有借用 ? 的 From 轉換冒充 collect 的契約，局部通過；既有 initial 實驗支持所列行為，本輪不冒稱再執行。
- `007-006/007-007`：原需求保持「次數降序、同次數名稱升序、固定輸出」。stable 的例子只按第一欄，第二欄 30／10 留原順序，確有觀察到限定的穩定性而未偷偷用第二鍵。HashMap 任意來源順序使穩定性不足；`right.1.cmp(&left.1)` 的降序方向和 Equal 時名稱升序在左右實例與 comparator 對得上。鍵唯一加完整 tie-break 才產生明確位置，並且 owned into_iter／collect 保存名稱而非 clone。方法理由與結果、限定條件一致，局部通過。

這些覆核沿用既有先備與相同學習問題，沒有自設新應用替來源補理由，也不將上述抽樣說成新的全 50 單位首讀。

## 正式依據、執行與未驗範圍

本輪為新增文字的最低技術疑點查閱 Rust 1.99.0 官方文件：

- `https://doc.rust-lang.org/std/prelude/index.html`：prelude 自動引入，內容列 `std::ops::{Fn, FnMut, FnOnce}`，2024 延續前版內容。支持名稱來源修訂；沒有把完整清單補進學生正文。
- `https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-FromIterator%3CT%3E-for-Vec%3CT%3E`：一般 `impl<T> FromIterator<T> for Vec<T>`，支持新的一般形式。
- `https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map`：map 按值接 self 與 f；配合原 initial 已查的契約與 Copy iterator 實驗，支持當前措辭。並非新寫一個 test 來鏡射文句。

本輪沒有執行 Cargo、Rust examples／tests／negative 或整套 check。root 所報 candidate full check 及 50 negative PASS 是 **root 的證據**，不記成我跑的結果；原 initial 已保存的局部探查也維持原來的執行時點。

本輪**未看實際桌面／手機教材頁面、未做 DOM 或 SVG 視覺檢查**；圖片路徑及 bytes 核對不能證明實頁可讀性。FIRST 的獨立 SVG render 也不轉算為我看過實頁。此處只關閉以上文字／首次需要關係，網站呈現仍由相應 QA 紀錄判定。

局部修後技術與先備問題已關閉，未新增教材修改；原 initial 保持原樣。本任務到此停止，第 12.4 節仍等待 actual 10.8 先備驗收。
