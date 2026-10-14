# 第 4 章技術交叉與修後局部複查

日期：2026-10-14。本輪在獨立技術初判封存之後讀同行紀錄；沒有修改教材、回寫原初判或重跑全套。第 4 章銜接審閱尚未完成，**不宣稱三角色交叉覆核全部完成**。

## 初判、可見材料與版本

- 原技術初判 `ch04-technical-v5.md` SHA-256 仍為 `2464e1c1c8ac4225067bd1593ca86fdd56383dd980e8c020cb829d0f0b21cebd`：必要 N1、可選 I1（兩處）、I2。
- 互看材料：實讀 `ch04-first-reading-v1.md`（4.1 用 frozen v1、4.2 用 v3、4.3–4.5 用 v5，分階段追加）、`ch04-first-reading-followup.md`（四處 v6 局部複查）、`ch04-disposition.md`。首讀者只用指定補讀背景，不是完整前1–3章的首讀驗收，也沒有看完整答案或執行程式。
- 修後局部：`/workspace/work/tutorial-implementation/frozen/ch04-review-fixed-v7`，核對 `002-003`、`003-003`、`003-005`、`003-006`、`004-009` 及改動的 `custom_keys.rs`／`map-query.svg`；其餘只作 source／chunk 指紋與版本範圍核對。
- v7 manifest SHA-256：`d672bf58e8f197b3c603129adc3f9a72619d383a4a987a08c0db04e0b0333f14`。38 chunks、56 archived sources 的實際 hash 全與該 manifest 相符；與 v5 的 source 表相比，僅 `custom_keys.rs` 與 `map-query.svg` 有內容改變。
- 沒有本輪自己的 Rust 執行或頁面／圖像實看。root／首讀者的 custom_keys 執行及 map-query crop 實看只是引用的同行證據；SVG source 幾何與語意核對不代替渲染驗證。

## 原問題配對、分級與修後核對

### N1：延後初始化

首讀 N1 與技術 N1 獨立重現同一缺少關係：首次 `let counts;`、稍後給初值與無 mut，沒有在使用前教。兩角色都保留主 get_mut 統計方法已理解／正確的部分，沒有把局部語法缺口擴成整節未教。

root 保留此寫法、補一段解說，與技術初判建議的 block expression 是不同但同樣有效的修法。v7 `003-005.md:85` 明寫：「這次是初始化，並非修改既有值，因此不需要 `mut`；型別可由後面的初值推導。Rust 會檢查在讀取前已完成初始化。」段落位於測試之前，第一次程式形式在第114行纔出現。型別推導、immutable 首次給值與讀前初始化，均有 [Reference 的 let 契約](https://doc.rust-lang.org/reference/statements.html#let-statements)及[官方 Declare first 範例](https://doc.rust-lang.org/rust-by-example/variable_bindings/declare.html)支持。

這項補教確實涵蓋原缺口，source 不需改。新負擔是一種宣告形式，但就地完整說明，沒有隱藏分支／任意延後讀取規則。**N1 修後技術通過。**不將 v7 的解說回算成 v5 當時已教。

### I1a：4.2 比較敘述

此處只由技術初判提出，首讀者原紀錄已能說「固定 amount 只改 count，使第二欄作用可見」；沒有在此報必要問題。這支持技術的可選分級：derived 全部欄位規則與原具體需求都已成立，改的是結果文字範圍。

v7 `002-003.md:70` 改成「顯示相同金額下，數量不同仍是不相等的摘要」。它與 source 的 360／3、360／4 精確對應；不增加例子或 API、不收窄已教的 derive 一般規則。**root 的可選採納成立，局部修後通過。**

### I1b：複合鍵對照

首讀 I2 與技術 I1 的 GroupKey 部分獨立重現：文字說兩欄，而材料固定 region 只改 category。兩者都承認 derive 已明教全部欄位，並保持可選分級；沒有要求另證明 Hash 演算法。

root 選擇增加同形材料，而不是隻收斂文字。v7 `003-006.md:11` 現說「分別只改類別、只改地區」；第39–42行加 `different_region`，值為 `(2,2)`，與已存 `(1,2)` 只差 region。第51–54行查詢／顯示，第57行 assertion 為 None，第71行列預期「地區2、類別2未收錄」。archived `custom_keys.rs` 有完全對應的增加。

新材料只用已教 struct 建立、shared 借用鍵、get、Option、println 與 assertion；不引入 trait 手寫實作、新查詢 API 或數量規則。兩種單欄變化各有可見結果，原聲稱的例子範圍現在匹配。root 與首讀 followup 記錄的新增負擔小、收益成立。**局部 source 與 first-use 修後通過**；執行沿用 root 的既有紀錄，未算本角色新實跑。結果仍不能推出 hash 不碰撞或所有自訂實作正確。

### I2：4.4 已教文字比較的指涉

首讀 I3 與技術 I2 獨立指出同一前文狀態問題；雙方都維持可選，因 first_nonempty 的兩來源契約與指定長度檢查已能完成。

v7 `004-009.md:7` 現為「本練習用數值 assertion 檢查長度，並顯示文字核對選擇，先聚焦可能的借用來源；4.2 已教的文字內容比較也能用來檢查結果。」它承認前文能力，並保留本題焦點，不把內容相等當來源 owner 身分證據。未改算法／答案、沒有新操作。**root 的可選採納成立，局部修後通過。**

### 同行新增視覺 I1：map-query 查找線避字

這是首讀者實看 v5 crop 才報的發現，**本技術初判沒有親看，不能算獨立重現**。原技術 source 語意通過不否定原位名稱辨認的呈現負擔。

v7 的 SVG SHA-256 為 `75f1b7da233283b4a81ba86a4e8f26533b077893171de262c729f12725ac015c`，與讀者 v6 followup 所看圖相同。綠查找線改為 `M112,265 L112,335 L8,335 L8,448 L40,448`，從查詢框下方沿 map 左側繞到 String 鍵；兩條藍借用線不變，query／counts／found 及鍵值文字、圖例也不變。source 的兩借用來源關係維持，沒有新增概念。

root／首讀者已有原位 crop 避字與可追蹤的實看紀錄；這支持可選採納收益。本輪只確認圖指紋與 source 變化，**不自行判渲染字級、避字或整頁閱讀路徑通過**。正文 `003-003` 扣除 frozen 路徑，沒有內容改變。

## 主要通過項交叉覆核

- 4.1 兩種資料各實作 amount、泛型 bound 保證共同門檻可呼叫，以及編譯簽名不證明商業算法：首讀與技術對身分、分工與用途的支持範圍一致。
- 4.2 Debug／PartialEq／Clone／Copy 各回應原開發需要，String 副本與 Vec<&str> 共借的界線一致；首讀者沒有從相同輸出反推同 owner 或深層複製。
- 4.3 名稱統計有缺鍵建立與已有鍵更新的實際兩分支，query 只供查找、found 借 map；首讀者沒有以外連 Borrow／Entry 解說補正文。源程式驗證與局部語法問題保持分開。
- 4.4 annotation／elision／placeholder／outlives 描述介面關係，不延長來源；first-only 是改需求，owned 結果獨立保存，與首讀的使用條件理解相容。
- 4.5 模型持有 reference 與原文字 ownership、receiver 借模型與欄位借來源各有清楚角色；兩欄輸出只連相關來源。首讀與 source 判斷沒有把 byte 長度當 lifetime。

以上是互看後的來源支持範圍，不新增原始發現或冒充真實讀者成效。必要 N1 與可選文字／材料修正已在指定 v7 局部核實，未發現這些改動引入新的必需先備。獨立銜接角色尚待完成，執行／視覺與發布 QA 也由實際完成者另記；本輪不稱全章所有驗收已完成。
