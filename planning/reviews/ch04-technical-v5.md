# 第 4 章獨立技術初判：凍結 v5

日期：2026-10-14。角色：未參與第 4 章寫作的獨立技術審閱者。這份紀錄先封存，尚未交叉閱讀首讀、銜接審閱、作者 ledger、source-notes、coordinator 問題清單或修後稿；沒有修改教材。不是未提示的目標讀者首讀，也不以技術判斷回填讀者的理解。

## 範圍、版本與實際查覈

- 原稿：`/workspace/work/tutorial-implementation/frozen/ch04-complete-prerequisite-v5/`，按 `reading-order.txt` 實讀 5 頁、38 個正文單位；另讀 manifest 所列全部正常程式、完整答案、library helpers／tests、失敗 fixtures、預期錯誤與 Cargo manifest。
- 必要前文：實際已讀的 `ch01-complete-source-v8`、`ch02-complete-source-v4`，本輪另實讀 `ch03-prerequisite-v3` 的 6 頁正文。只用正文真正教過的語法、能力與接口；未以規劃表、未讀教材或外部參考充當讀者先備。
- 讀者：懂基本 Python；以好懂好學與首次使用前已教為最高判準。片段、完整答案、fixtures、tests、helpers、manifest、指令均受相同規則約束。
- `manifest.json` SHA-256：`dfc3ce020bc3b8e860901e28b52bdc48abb3132ff6d583fd57d7b560055f07f0`。
- `reading-order.txt` SHA-256：`980c379d0ca6ab150ad1c5545bc109b781c52b8ceb5fc62e20d8f87b1b525bbc`。
- 38 個 chunk、56 個 archived sources 的實際 hash 全與 manifest 相符，其中有 39 個 Rust source、2 個 Cargo manifest、11 份預期錯誤及 4 個 SVG。正文引用的 `assets/` 副本也逐一與 archived SVG 指紋吻合。完整逐檔核對保存在 `/workspace/work/tutorial-implementation/technical-ch04-v5/fingerprints.json`。
- manifest 的 5 個原始頁面 hash 是凍結者的來源識別；凍結包沒有原始完整頁面檔，本輪沒有把它們說成已重新計算。可重算的是實際閱讀的展開 chunk 與 archived source。
- 本輪親讀 SVG source，核對資料、名稱、箭頭意義與正文對應；未親看圖片渲染、桌面／手機實際頁面，不聲稱版面與可讀性通過。
- 沒有自行執行 Rust 範例／fixtures，也沒有重跑 `check.sh` 或整套測試。根協調者既有執行結果不算本輪自己的執行證據。本輪唯一必要問題是未教語法，其語義有官方直接契約；不需要重跑正常程式確認措辭與先備是否存在。

## 結論

發現 **1 項必要先備修正 N1**、**2 項可選編輯改善 I1、I2**。未發現凍結 source 的計算錯誤或必須提前依賴 `Box`、`dyn`、`Deref`、closure、`Entry`、`From`、`Iterator`、HRTB／`'static` 的正常範例。除 N1 所在測試以外，已核對的新語法與具體 API 契約均在使用前由正文介紹，或能直接沿用必要前文。

這是技術與來源初判；未驗證的視覺、實際執行與獨立讀者理解仍保持未驗證。

## N1：ownership 測試首次使用未教的延後初始化

**必要；先備／時序，增加理解負擔。** 發現者是本輪技術審閱者，未受其他問題紀錄提示。

- 位置：4.3 `003-005.md:110–117`；實際 archived source `examples/ch04-traits/src/counts.rs:54–60`。
- 原句／程式：正文說「原清單在內層 scope 結束後，map 的 owned 鍵仍然可查」，接著使用 `let counts;`，再於內層寫 `counts = count_labels(labels.as_slice());`，離開 block 才 assertion。
- 已教部分：1.4 `004-002.md:5` 以 `let price: i32 = 120;` 教名稱、型別與初始值；`004-004.md:5` 正確限定 immutable 為「初始值已給定後，不能再對同一個變數指派另一個值」。1.5 已教 block 的最後 expression 交出值；2.4／4.4 已有從 block 交回結果、再在外層使用的形式。這些足以教「String 清單先結束，owned map 留給外層」。
- 缺少關係：此前三章與本章較早正文沒有教可省略初始值的 `let name;`、第一次初始化與重新指派的區別，以及未初始化名稱不能先讀。前文的「初始值已給定後」避免了技術矛盾，但不等於已教這項新宣告形式。
- 官方契約：[Reference：let statements](https://doc.rust-lang.org/reference/statements.html#let-statements) 允許省略 initializer；[Rust By Example：Declare first](https://doc.rust-lang.org/rust-by-example/variable_bindings/declare.html) 明示先宣告、後初始化，並要求使用前已初始化。本例在 Rust 語義上合法，問題是它作爲示範測試提前用了新語法。
- 實際影響：讀者需要自行猜測 `counts` 在最初是否已有 map，以及為什麼沒 `mut` 仍可在後面寫 `counts = ...`。這個旁支不是證明輸出 ownership 的必要條件，且違反所有 example／test 只用此前已教內容的標準。
- 最小修法與收益：改成已教的 block expression：`let counts = { let labels = ...; count_labels(labels.as_slice()) };`，再保留同一個外層 `assert_eq!`。同時修正文引用的 source；內層 labels 仍結束，外層 map 仍被查詢，不新增教學目標或 API。若作者確實要保留原形式，必須先在正文介紹延後初始化；就本節主線，block 形式負擔較低。
- 處置：待協調者裁定與修正；本輪未修稿、未作修後複查。

## I1：把兩個對照例的結果敘述收斂到實際變化

**可選；例子／證據文字精度。** 不認定 derived 契約未教或程式有誤。

- 位置與原句：4.2 `002-003.md:70`，「第一組比較固定金額，分別讓數量相同與不同，顯示兩個欄位都參與判斷」；4.3 `003-006.md:11`，「下面固定地區編號，只改類別編號，檢查完整鍵的兩個欄位都參與分組」。
- 已教部分：前面的 `derive(PartialEq)`／`derive(Hash)` 已明確教逐欄比較／使用欄位資料；[PartialEq 的 derived 規則](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html#derivable)、[Hash 的 derived 規則](https://doc.rust-lang.org/std/hash/trait.Hash.html#implementing-hash) 也支持這些程式確實使用全部欄位。讀者不必由兩組輸出猜出一般規則。
- 實際材料：第一例把 amount 固定 360，只變 count 3／4，直接顯示不能只按金額認同一筆；第二例把 region 固定 1，只變 category 2／3，直接顯示不能只按地區認同一鍵。這些都完成原先提出的具體需要；單靠這兩份輸出不另驗證變動 amount 或 region 的效果。
- 最小收益：可分別改成「相同金額下，數量不同仍是不相等的摘要」與「相同地區下，類別不同仍是不同鍵」。一般的全部欄位規則留在已存在的 derive 說明，不必增加 example 或測試。更容易看出本次對照刻意控制了什麼。
- 分級理由：原主線可由明示的 derived 規則直接理解，且例子確實回應數量／類別的需要，沒有必要先備缺口；此項只改善結果敘述的範圍。
- 處置：待協調者裁定；未修稿。

## I2：4.4 練習的比較限制可避免像是退回未教狀態

**可選；先備狀態／編輯精度。** 不認定本題實際用了未教操作。

- 位置與原句：4.4 `004-009.md:7`，「只以已教的數值 assertion 檢查長度，顯示文字核對選擇；不直接拿整個自訂資料或文字結果作尚未介紹的比較」。
- 已教部分：4.2 `002-003.md:7–13` 已教自訂資料的 `PartialEq` 與 `assert_eq!` 所需能力；`002-004.md:5` 明確教 `String`／`&str` 的內容比較，完整答案也已比較 `.as_str()` 的文字。因此讀者已能理解文字相等檢查；本題仍可以刻意沿用長度與輸出作檢查。
- 缺少／影響：句子的教學意圖可以理解，但「尚未介紹」容易讓讀者誤以為先前已教的文字比較目前仍不可用。這不是 lifetime 選來源機制的障礙，也沒有迫使使用新 API。
- 最小修法與收益：保留「本題沿用 byte 長度的數值檢查，另顯示文字核對選擇」，刪掉不合本章順序的泛稱；不必改答案或擴充測試。這樣把練習的選擇與能力是否已教分開。
- 處置：待協調者裁定；未修稿。

## 首次使用核對：正文建立的實際先備

| 新概念／操作 | 使用前真正已教的位置及關係 | 核對結果 |
| --- | --- | --- |
| `pub struct`／公開欄位 | 4.1 `001-002:7`：example 在 library 外建立／讀欄位，需要公開；前文有 module／`pub fn` | 在第一個 model 片段之前已教，不靠 crate 內可見性補洞 |
| trait、method 簽名、`impl Trait for Type` | `001-003:5–7` 先教契約、`&self`／回傳型別與無 body 分號；`001-004:5–7` 先教為型別實作、符合簽名與不加 `pub` | `Charge`、兩份實作皆在使用前有角色與新語法 |
| `T: Charge`、generic method 呼叫 | `001-005:5–9` 接回 3.3 尚不能假定任意 T 的能力；bound 授權 amount，shared 參數保留原資料 | 正常函式與移除 bound fixture 的理由一致，不需 `dyn`／`impl Trait` |
| 外部 trait method 的 `use` | `001-006:5–7` 在外部呼叫程式前教 trait 進 scope、package／crate 名稱差異 | source 的 use 與已教公開路徑吻合 |
| `Debug`、`derive`、`:?` | `002-002:5–11` 先教整筆檢查用途、格式設定、attribute、欄位必要能力 | fixture 只移除 derive；不要求 Debug 永久格式穩定 |
| `PartialEq`、derived enum 相等、整筆 `assert_eq!` | `002-003:5–13` 先教全部欄位／同 variant、assert 的 Debug 要求、比較借用而不 move | source、答案均在介紹後使用；I1 只涉及結果用語 |
| `String`／`&str` 比較、Clone、Vec／shared reference 的 Clone | `002-004:5–17` 先教文字內容比較、借原值回傳新值、String 副本獨立；再區分 reference 與 Vec 各自 Clone | `Vec<&str>` 沒有被誤說成複製 owned 文字；來源仍需有效 |
| derived Copy 與 Clone 關係 | `002-005:5–9` 接回 2.2 整數 Copy 與自訂 struct 不自動 Copy；先說欄位條件、Copy 也須 Clone | 正常兩份數量 3／4、String 欄位 E0204 fixture 均對應；不以 stack／heap 判 Copy |
| `HashMap<K,V>`、`Eq`／`Hash`／collision | `003-001:10–12` 有按名稱存次數的需求；`003-002:5–14` 教 Eq 額外自反承諾、相等 hash 一致、collision 仍須比較 | 沒把 Eq 當新增比較方法、沒把相同 hash 當相等；未選演算法或硬編 hash 值 |
| `new`／`insert`／`get`／`len`／String→str 借用表示 | `003-003:5–24` 先教 import、具體參數／結果／receiver 契約、ownership、保留原鍵、Borrow 的 Eq／Hash 一致性 | get 回傳借自 map 的值，不借自 query；先改 query 再讀 found 是該契約的有效對照 |
| `get_mut`、`&[String]` 走訪、缺鍵才 clone | `003-005:7–14` 先教 mutable 結果、for 取得 `&String`、明確 `(*label).clone()` 建 owned 鍵 | Some 修改借到的次數，None 結束本次借用再 insert；不需 closure／Entry 或 String→str 的 `Deref` trait |
| Option／shared 數值 reference 相等與 Debug | `003-005:79–81` 在測試前教 Some／None 逐內容比較、reference 比較所指數值、`Some(&2)` 與 Debug | 所有 `Option<&usize>` assertions 已有具體契約；單一未教語法是 N1 的 `let counts;` |
| derived `Eq`／`Hash`、同型別借用鍵 | `003-006:5–11` 先教 GroupKey 的完整身分、欄位必要能力與配合；查詢表示為鍵本身 | lookup 的 3／None 與缺 Hash fixture 相符；I1 不否定全部欄位的明示契約 |
| 單一來源 elision／兩來源歧義 | `004-002:5–9` 先教 same 回傳借用資訊及唯一輸入關係；`004-003:5–7` 再教兩個獨立輸入無法決定輸出 | 先單一正常例，再 E0106；不將 String 本身當成函式內會結束的輸入 reference 值 |
| `'a` 宣告與 `&'a str`、共同有效要求 | `004-004:5–9` 在函式前教參數名、位置、所指資料至少滿足有效範圍；不要求 owner 同時建立／結束 | 兩次 Python 輸出只示範 byte 選擇，另用內層 hi 的 E0597 示範來源約束 |
| 只連第一來源／owned 返回 | `004-005:27–35` 先說改功能與 `_` pattern；`004-006:5–30` 教區域 String 無法借出，改回傳新 owned String | first_text 能跨另一來源結束，copy_text 可跨原來源結束；未把 annotation 當延長存活 |
| `'_`／lifetime bound `'b: 'a`／`Option<&'a str>` | `004-007:5–7`、`004-008:5–7` 分別在程式之前教 placeholder 與至少涵蓋的方向；Option 與 reference 的角色此前已教 | fallback 的 Some 空段與 None 不混同，移除 bound fixture 的必要關係正確 |
| struct 中的 lifetime、`TextView<'_>`、`impl<'a>` | `005-002:5–9`、`005-003:5`、`005-004:5` 在第一次片段前說明來源參數、唯一輸入推導與 impl 的參數範圍 | 沒讓模型取得文字 ownership；來源過短 fixture 與前文規則一致 |
| receiver elision 與欄位來源的兩次借用 | `005-004:7–17` 在 methods 前教 &self 是對模型的另一次借用；表格區分 text／original／owned byte_len | original 可跨模型結束；text() 的 E0597 是 receiver 契約，不能直接把兩次 lifetime 當同一個 |
| 兩個欄位、兩個 lifetime、單一來源輸出 | `005-005:5–9` 先教各來源與方法輸出各自連結；已有兩種借用的區別 | 13 + 4 = 17；第二來源／模型結束後外層仍讀第一來源，不宣稱資料一起變長命 |
| 答案、helpers、manifest、commands | `owned_text`／`copy_text` 用 3.4 的 new／push_str；5 份答案與 lib／tests 只沿用相應已教項；2 manifests 的 workspace 繼承與 -p／--example／--locked 均來自 1.3 | 無新增第三方依賴／未知轉換 macro；範例目標與指令名稱一一吻合 |

上述是技術審閱按實際正文建立的依賴，不代表讀者已答對四題。先教具體 API 的 borrowing／結果契約是有效教學，不要求先把它內部的所有泛型 trait 簽名都教完；這項允許也不替 N1 的未介紹語法放行。

## 主要通過項與真正支持的範圍

### 4.1：共同金額能力與門檻判斷

- 原需要：`001-001:3`，兩個資料欄位不同的型別，都要判斷金額是否至少 300，門檻規則不要各寫一次。
- 機制／性質：`001-003:5–15`、`001-004:5–23` 將各型別如何取金額分在 Charge 實作；`001-005:7–17` 的 bound 讓共同函式只看能力。技術足夠。
- 需要／用途：同一比較 body 接 &Order／&Bundle，分別得到 360／160、true／false，且保留原資料。`001-005:23` 也承認原本可分別寫函式，未貶低舊法能力。用途足夠。
- 例子／證據：`charges.rs` 的 2 個 tests 實際驗金額及兩型別的 300 包含／301 不包含邊界；DiscountedBundle 答案讓新資料沿用原門檻函式。這能示範分工與重用，不能讓編譯器證明每個實作者的商業規則；`001-007:47` 已明確限定。

### 4.2：依資料需要選能力

- Debug 的機制是生成逐欄檢查表示，用途是一次看整筆，`002-002:5–11` 在 use 前接起；失敗 fixture 去除整體能力，單欄仍存在，對照有效。格式不穩定的限定回應檢查輸出是否適合存檔。
- PartialEq 的機制是依全部欄位／同 variant 內容比較；用途源自 `002-003:5` 金額相同、數量不同是否相同的問題。正常例可觀察原材料仍使用，且 answer 可比較文字，沒有偷偷 move。
- Clone 的機制先以獨立 String 內容、後以 Vec 中仍共享來源的 reference 區分；用途是 `002-004:7` 保留原標籤再改另一份。label 副本改後 original 仍 Rust，數值不變；Vec<&str> 的兩份顯示只是 reference 共借原文字，不是實測深層複製。
- Copy 接回已教整數複製，自訂 struct 須明確提供能力；兩份 count 3／4 與 String 不能 Copy 的對照實際運作。能力選擇並不以資料在 stack／heap 上決定。上述都在源材料範圍內成立，不宣稱所有 Clone 都深複製或零成本。

### 4.3：名稱統計、ownership 與查詢來源

- 原需要：`003-001:3–10` 保存 Rust 2／Python 1，名稱是身分，位置不是身分。
- 機制／性質：Eq／Hash／Borrow 的文字把鍵身分、一致的借用表示、collision 和最後相等檢查分開。new／insert／get／get_mut 的具體表格和 prose 各在調用前教；derived GroupKey 的能力契約符合官方條件。
- 需要／用途：`003-005:5–12` 明確區分重複名加一與單純替換值。三步表格實際同時出現缺鍵創建與已有鍵更新；輸入slice保留，map回傳獨立owned名稱，不需未來 Entry／closure。get使用 &str 讓查找無需另建 owned key，Borrow 一致性限定正好回應此需求。
- 例子／證據：正常 map 先 Rust 1→2、Python 1，len2；固定名稱 array 用來報告而非宣稱 map 順序；query 修改與 found 再讀實際將兩個來源分開。3 個 tests 分別涵蓋重複／缺鍵、空輸入及來源清單結束後仍查 owned map。N1 不影響目前 source 的 Rust 正確性，但其示範語法需修。
- StockItem 答案把加一變成加 quantity，source 的 Rust 2+6=8、Python3、len2，且原items可在後面讀取；新增數量範圍約定已在題目前說清楚。

### 4.4–4.5：借用來源契約與 receiver 的區別

- 原需要：`004-001:3` 想交回借用文字而不重建內容，必須知道哪個來源仍需有效；`005-001:3` 又要把借用放進資料模型供後續工作。
- 機制／性質：先 single-source elision，再 multiple-source 歧義、`'a` 共同要求、first-only 合約、owned 獨立結果、placeholder／outlives；隨後把來源關係放入 struct，並區分 model receiver 借用。這些與 Reference 的規則吻合，也不把 `'a` 說成秒數／整個 lexical scope／延長 owner。
- 需要／用途：`004-005:5–35` 刻意在 outside 13 bytes、inside 2 bytes 下，仍以 E0597 示範 two-source 合約不能靠人看出的實際分支繞開；改爲 first-only 是改功能，不是假修較長選擇。owned 返回回答獨立保存需求。`005-004:7–17` 的 receiver 對照則回答「原文字還在、model已結束」能否把結果保留的問題，不能用“借用都一樣”合併它們。
- 例子／證據：source source.rs／views.rs 的 6 個 tests 與正常程式分別觀察 byte選擇、source改後owned結果仍4bytes、Some空段不回退、receiver／field來源及第一來源跨第二來源／模型結束。它們是具體材料檢查；未測所有文字，不證明額外分支或未來borrowed資料模型都正確。
- byte單位一致：Rust4、Python6、Rust tutorial13、hi2、note4、two-models17。文本長度與 lifetime 長短沒有混稱；所有答案 numeric assertions 與借用來源在此前已教。

## 圖的內容查覈與限制

| frozen SVG | 指紋 | source 讀取後核對的必要對應 |
| --- | --- | --- |
| trait-contract.svg | `b3411edef987179eedd841cad5bd7afda48f1f58deb55bf97dca21b79951f204` | 左 Order 120×3→360，右 Bundle price160→160，各提供 &Order／&Bundle；共用判斷框仍是兩次呼叫，結果true／false各保持左右對應。不是一個合併值。 |
| map-query.svg | `aae0087e2a06d48d21083276f039b37267ff2c5859489efbb2ce609335f2dd37` | query String 擁有 Rust，as_str 借 query；綠虛線以文字找 owned String 鍵；found Option<&usize> 的藍實線指 map 的值2。數值／字串相同沒有被用來當同一個 owner。 |
| two-sources.svg | `b998606f6f12113e38fe18efab59cca38029be305512083ac608ecd2c172c83c` | first與second各爲 &'a str，可通往單一 &'a str 輸出；圖說標可能來源而非同時交兩值，不把共同參數當相同owner存活起訖。 |
| borrowed-view.svg | `b6c5e7c89ca18a4e22522f2ff94a7e21b9af1c725b7da94a63eb6c60b4d2b4d0` | 上 view:TextView<'a> 的 view.text reference，向下借外部text:String管理的Rust tutorial內容；正文5.2明確分開同名field和local。 |

正文位置分別是 `001-005:21`、`003-003:26`、`004-004:23`、`005-002:19`，源文本與圖說相鄰。以上只支持內容／原稿排序，**不支持圖像渲染、頁面原位視覺、手機字級／裁切或四條線是否真的清楚**。

## 官方查覈來源

本輪實際查閱以下原始來源，僅作技術查覈，不以它們替讀者正文補先備：

- [Reference：Traits](https://doc.rust-lang.org/reference/items/traits.html)：required method、實作及泛型 trait。
- [Reference：Trait and lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html)：bound 允許依契約使用能力，lifetime outlives 至少涵蓋。
- [Reference：Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html)：每個省略輸入獨立、唯一輸入接輸出、receiver優先規則、`'_` 路徑推導。
- [std：Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)、[PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)、[Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)：derived 行為、borrowed 比較、相等法律與 Debug 格式不穩定。
- [std：Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)、[Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)：borrow→同型別新值、逐欄 Clone、shared reference 不復制被指數據、Copy 的欄位／Clone 條件與 String 限制。
- [std：Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html)、[HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)、[Borrow](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)：全部欄位參與、相等鍵 hash 一致、get／get_mut／insert 的真實receiver與結果、保留舊鍵、借用表示身分一致。Borrow 的抽象示意不當成 insert receiver 的依據，實際 receiver 取 HashMap 官方API。
- [Reference：Statements](https://doc.rust-lang.org/reference/statements.html)、[Rust By Example：Declare first](https://doc.rust-lang.org/rust-by-example/variable_bindings/declare.html)：N1 的合法但未教語法。

## 尚未完成

初判保存後纔可進行與第 4 章首讀／銜接紀錄的交叉核對。必要問題N1未修；I1／I2尚待協調者逐項裁定。任何修後核對另存新檔，不改本份來源初判；未授權寫第10章或擴至其他章。
