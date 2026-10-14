# 第 1 章尾七頁：獨立技術初判 v2

日期：2026-10-14。審閱者：`chapter10_concurrency`。角色：clear-tutorial 第二輪技術與證據，未參與 1.4–1.10 的寫作。本初判不修改教材。

## 凍結範圍與獨立性

- 讀者入口為基本 Python；可用先備是已完整讀過的 1.1–1.3 正文，以及本輪依順序讀到的尾段正文。前綴修正不變更這輪所用 package／crate／Cargo 選項等基本能力。
- 只按 `/workspace/work/tutorial-implementation/frozen/ch01-full-v2/reading-order-tail.txt` 讀 `004-001` 到 `010-005` 的 35 個凍結單位；沒有讀 tail 首讀紀錄、作者帳冊、其他尾段 review、已知問題或修後 v6 正文。
- 曾參與前綴技術審閱與交叉複查，不宣稱沒有前綴歷史的首讀盲測；這輪是未參與尾段作者工作的獨立來源初判。
- manifest SHA-256：`46c96d0ea57de6361707cf24d28bbdb1d40a8ee1f7904eb17f44ef27ce81a612`；tail reading-order SHA-256：`1f99b8f06792c12e6d2b71e807918257ec7b2c9584aa7e910707d78d8993ffa9`。
- 35 個 chunk 的實際 SHA-256 全吻合；16 份列入 manifest 的 Rust source 與現行 source 相符。原位凍結 SVG 指紋為 `d9fbd63ef9ec1c907015fe23570fdeac0c5bcc1ca9c8b48bd9387810f36ff4a6`，與 manifest 相符。現行 SVG 指紋已不同，未用它替代本輪凍結圖，也未開其修改內容。
- 另外讀取正文連結的七份完整參考答案；未列入這版 manifest 的答案以本輪實際 source 指紋另存，不冒稱它們原先已被 manifest 凍結。
- 指紋與 source 核對資料保存於 `/workspace/work/tutorial-implementation/technical-ch01-tail-v2/source-fingerprints.json`。

root 已告知正常範例、check.sh 與 fixtures 的執行驗證。本角色只為浮點除零疑點做一個相稱執行，沒有重新跑正常全套、重裝工具或更改 workspace／lockfile。未親看桌面／手機頁面，不能把 root 的視覺檢查當成自己的驗收。

## 必要項 N1：除數限制概括到剛介紹的浮點數

- 位置／原句：`004-003.md:57`：「輸出是整數商 3、餘數 1、小數商 3.5、溫度 −2.5。**除數不能是零**；整數運算也必須留在型別可表示的範圍內。」
- 已教部分：`:39` 正確分開整數除法往零截去與浮點除法，兩份數值例子和預期輸出正確。沒有將這節的除法整體判為錯誤。
- 技術契約：[Rust Reference 的運算規則](https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators) 將除零會 panic 的規則限定為整數型別。`f64` 的基本浮點規則會提供無限大或 NaN 等特殊結果，不能以同一句無條件限制暗示兩種運算都被 Rust 拒絕。[f64 文件](https://doc.rust-lang.org/std/primitive.f64.html) 指向其 IEEE 754 型別與 f32 的基本說明；[f32 特殊值說明](https://doc.rust-lang.org/std/primitive.f32.html) 明列浮點除零的無限大結果。
- 最小執行：Rust 1.99.0／edition 2024，只用已知變數、f64、除法與 println 的私有檢查，`1.0 / 0.0` 與 `0.0 / 0.0` 編譯成功並輸出 `inf`／`NaN`，process exit 0。紀錄位於 `/workspace/work/tutorial-implementation/technical-ch01-tail-v2/verification.json`，沒有新增學生範例。
- 影響：讀者剛在同一段學兩種除法，這句的對象未限定；若做零這個小變化，可能把預期的整數失敗規則誤套到 f64。這不是要求此時全面教浮點特殊值。
- 分級：**必要／技術限定不精確，增加理解負擔**。
- 最小建議：縮為「整數除法／餘數運算的除數不能為零」，保留本節普通有限數值的例子。不需為這句修正提前加入 is_nan、浮點排序或完整 IEEE 754 章節；若作者另要講浮點特殊結果，必須先給它們最低含義，不能只多列名字。
- 發現時點：互看前獨立正文閱讀與原始來源查核；未讀尾段其他問題。
- 修後：尚未複查。

## 必要項 N2：觀察門檻失敗時改用了不同範圍的命令

- 位置／原句：`010-003.md:60`：「暫時把 `pricing.rs` 的免運條件……改成……，再執行 `cargo test`。」
- 已教部分：同單位 `:50–54` 已明示在 repo 根執行 `cargo test -p ch01-basics --locked`，四個測試、邊界材料與預期 failure 完整。1.3 也已教依名稱選 package 與保留 lockfile。
- 技術契約：[Cargo test：Package selection](https://doc.rust-lang.org/cargo/commands/cargo-test.html) 說明在 virtual workspace 根、沒有明設 default-members 時，裸 cargo test 預設選全體 members；這不等於只選 ch01-basics。裸命令也沒有此教材操作前文約定的 --locked。
- 影響：如果依原句實際輸入裸命令，工作範圍從當前計價 package 變成 workspace 預設集合；讀者可能混進其他練習的輸出／檢查，卻以為只重做這四項。這份最終教材 repo 將有多章 package，保持已教選取方式才能有穩定的當前觀察。
- 分級：**必要／命令範圍與既有操作約定不一致**。不是說本次裸命令必然執行失敗，也沒有猜測另個 package 必然壞掉。
- 最小建議：寫「再執行上面的測試指令」或重列 `cargo test -p ch01-basics --locked`。保留四項測試與門檻實驗；不教尚未需要的 workspace default-members。
- 範圍限定：這個判斷針對需要選取／建置 package 的 test 命令；`cargo fmt` 的後端與選項不同，不為了機械套用 --locked 在格式化命令加上不適用的旗標。
- 發現時點：互看前獨立來源初判；只查官方選取契約，未重跑全體 tests。
- 修後：尚未複查。

## 必要項 N3：把訂單輸入限制寫成全章／所有函式限制

- 位置／原句一：`006-001.md:5`：「**本章把輸入限制在非負的小整數**，且相乘與相加都不會超過 i32 範圍。」
- 位置／原句二：`010-003.md:62`：「**我們的函式仍採用非負、小數值的輸入約定**。」本單位被稱作「我們的函式」的測試對象包含 sum_three。
- 已教部分：訂單計價的單價／數量使用非負、範圍內的小值是合理的簡化，`006-001` 已說明它把重點放在選擇規則；沒有暗中要求現在完成外部輸入驗證。
- 衝突材料：`008-005.md:5` 明確要求以 `[0, -5, 5]` 驗證 sum_three 得 0；`010-003.md:42` 的正式 unit test 同樣使用負數。真正 source 的 sum_three 只做 i32 加總，並没有以非負條件拒絕這些值；這個負數測試是有意驗證累積全部輸入。
- 影響：限制句的主詞擴到「本章／我們的函式」，使讀者無法一致判斷負數到底是有效加總材料還是違反本章契約。不是負數相加的數學或程式出錯，而是契約範圍與實際練習互相矛盾。
- 分級：**必要／限定對象不清，增加理解負擔**。
- 最小建議：改為「本節的訂單計價輸入……」，1.10 收束改成計價函式採非負小值、加總函式的 i32 材料可正可負但仍限制在不發生 overflow 的範圍。1.9 `009-003.md:46` 的「這些函式仍遵守前節的小數值範圍約定」也可保持小數值限制並明確區分兩種資料角色，無需加錯誤型別或額外執行期驗證。
- 發現時點：本角色對照原條件段、練習、測試後獨立提出，未見其他角色問題。
- 修後：尚未複查。

## 可選項 I1：total 的文字長度誤寫

- 位置／原句：`004-003.md:5`：「例如 {total} 顯示計算後的總額，而不是四個字母。」
- 判斷：total 有五個字母；格式 substitution 與 source 輸出均正確，只有這個附帶數量不正確，不影響會代入值這個主要理解。
- 分級：**可選／編輯準確性**。可改「而不是名稱本身」，避免加入無教學用途的字母計數。發現於互看前；未修改。

## 可選項 I2：跳過零沒有讓 continue 的作用出現在總和差異

- 位置：`008-004.md:33–40` 的最後一項示範，先教到 `if number == 0 { continue; }`，結果為 6；`:5` 明說本項要跳過 range 中的零。
- 已教部分：`008-003.md:9` 已在程式前說清 continue 跳過 body 剩餘工作、for 取下一項；source 也確實跳過零。沒有技術錯誤，也不抹去已教的控制流程。
- 有限收益：因為加零不改變總和，刪掉 continue 仍得到 6。輸出不能單獨讓讀者觀察「沒有執行那次相加」；若改跳過 2，結果 4 與全部相加的 6 就有直接差異，仍只用已教的等值比較與同一 range。
- 分級：**可選／例子可觀察性**。前文規則加 source 已足以推得真正流程，所以不列為未教或必要障礙。不要求再加另一組大量材料或新 API。
- 處置建議：若作者採用，需同時更新說明、source、輸出與對應檢查，做一次相稱執行；若保留原稿，也可保持其有限操作示意用途。發現於互看前，未修改或先跑改版。

## 核心契約、第一次使用與材料核對

以下以當前具體數值能力為檢查單位，不把 Rust 內部實作或完整泛型 trait signature 當成讀者必須先知道。例如 array.len 的具體 method 契約與 i32 assertion 的使用在本章先教足；不因它們底層牽涉 reference／trait 就提前插入後章內容。

| 範圍／小節問題 | 首次操作前的具體依據 | 技術與例子核對 |
| --- | --- | --- |
| `004-001–002`：保存訂單材料與型別 | 問題先要求總額隨材料計算；再教 let／型別註記／初值／literal／bit／各型別範圍／推導。 | [Numeric types](https://doc.rust-lang.org/reference/types/numeric.html) 的 i32、u32、usize、f64 契約相符。數值界限沒有當成 Python 任意長整數；本例型別選擇足以容納輸入。 |
| `004-003`：乘法與顯示、除法差異 | code 前先教 +／-／*、格式字串的具名位置；run 前先教 example target 與 --example；除法 block 前先教 /／%、整數向零截去與兩邊浮點。 | 120×3=360，80×3=240；7/2=3、7%2=1、7.0/2.0=3.5，溫度 −2.5，均與 source 相符。除零限定 N1 與文字計數 I1 另列，不擴大成數值範例錯。 |
| `004-004–006`：重新指派、shadowing 與材料變更 | immutable 失敗程式前先說禁止第二次指派；mutable 例前先教 mut／+=；shadowing 例前先教同名新變數與右邊沿用舊值；練習提醒 before 保存舊答案。 | E0384 fixture 與規則相符；3→4、360+20=380 再換同名 bool 合法。variables_solution 算 160→240，沒有因改 count 而自動改 before。 |
| `005-001–002`：命名共同計算與參數／回傳 | 先給兩筆資料及規則重複問題；code 前給函式型態、按位置對應參數、expression／statement／block 尾端值／unit。 | subtotal 與 show_total 的簽名／body相符，兩筆為360及160。[Block expressions](https://doc.rust-lang.org/reference/expressions/block-expr.html) 支持尾端值與順序；定義函式不會立即執行。 |
| `005-003–005`：分號失敗、內部 block、運費材料 | compile-fail 前教加分號會丟掉尾端計算結果；block 例前教 scope／外層可用結果；三參數練習只增加已教 i32 輸入。 | E0308 fixture 的實際 body 是 unit、簽名為 i32。內部小計360加20=380；functions_solution 得380及180。未要求新回傳型別／reference／泛型。 |
| `006-001–004`：空訂單、免運、門檻先後 | code 前教比較的 bool、if／else、分支的結果型別、return、&&／||／! 的必要行為與短路。else if 題前先教第一個成立者優先。 | [If expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html) 與 [operator 規則](https://doc.rust-lang.org/reference/expressions/operator-expr.html) 相符。80×2加20=180、120×3免運=360、count0為0；多門檻299／300／500得到20／10／0。非負限定範圍另列 N3。 |
| `007-001–003`：tuple 角色與 array 位置 | code 前教 tuple 固定項數／型別／零起點／解構、空格式位置、array 型別與初值形式、usize 索引、len method、array 解構及數值重複。 | source 的各操作在當前具體契約內；len3、索引1為80、三個零中的首項0正確。[Array/index expressions](https://doc.rust-lang.org/reference/expressions/array-expr.html) 支持本例；沒有學生需要解讀的未教 Copy 泛型定義。 |
| `007-004–006`：越界與 range | runtime-fail 前教最大有效索引與 panic 的最低角色；range 表先給起終點包含規則，不當 array；練習只用既有 tuple／array。 | fixture 的 price_at(3) 在3項 array越界、println 正常數值不會執行；訊息判準為 index out of bounds。0..3不含3、0..=3含3、2..2空，與 [Range expressions](https://doc.rust-lang.org/reference/expressions/range-expr.html) 相符。答案小計240、索引2、最後值200正確。 |
| `008-001–003`：按材料／條件重複與停止 | code 前教 for array 的每次具體 i32 值、迴圈外累積、range、while 前置條件、loop／break 結果及 continue。 | [Loop expressions](https://doc.rust-lang.org/reference/expressions/loop-expr.html) 支持每項順序、條件與停止方式。沒有在這裡展示 Iterator／IntoIterator 的未教泛型簽名；基本操作契約夠用。 |
| `008-004–005`：四種控制與有限加總 | 完整 code 只組合此前材料；第二個 sum 的 shadowing 在前文已教，後文明確說新累積不是沿用6。 | 400、6、找到3、跳零仍6皆正確，公平說 while 不具備 for 做不到的魔法。continue 的可觀察改善另列 I2。sum_three 的 [0,-5,5] 得0，答案只用已教的固定 array、for 與尾端值。 |
| `009-001–002`：module／公開路徑 | 原需求是把計價規則與顯示分開；任何 source 前先教 mod pricing、相鄰文件、pub mod／pub fn、路徑::與crate根。 | [Modules](https://doc.rust-lang.org/reference/items/modules.html)／[Visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html) 支持本例相鄰pricing.rs與公開祖先路徑。private shipping 的設計契約不允許其他 binary／example 直接呼叫。 |
| `009-003–004`：main／lib 兩個 crate 與另一使用者 | 先辨認 src/main.rs／src/lib.rs、package連字號與預設crate底線；圖置於 source 前，use 在 main source 前教。 | [Cargo targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) 支持 lib／binary／example 默認角色與名稱。本輪凍結 SVG 的兩箭頭分別標呼叫公開功能／宣告並載入，不把pricing看作另一個binary；未視覺實測。真實main得180／400，modules_solution得160／180。 |
| `010-001–003`：assertion／test／cfg 與政策錯誤 | 測試前教 bool assertion／相等 assertion、失敗會panic、#[test]／外框、同檔mod、cfg(test)、library內crate路徑與unit回傳。 | [assert_eq](https://doc.rust-lang.org/std/macro.assert_eq.html)／[Testing attributes](https://doc.rust-lang.org/reference/attributes/testing.html) 支持本次i32與unit測試。四個test各自依規則計答案；≥300改>300會使300小計增加20、得到320≠300，不是型別錯。裸命令N2、契約對象N3另列。 |
| `010-004–005`：fmt／Clippy／門檻材料 | 命令前先教fmt會改檔、Clippy只查常見問題、all-targets、--轉交rustfmt、--check不改檔；題前說goods/total來源，body只用已教assertion。 | 格式化、警告與政策正確三者分清；[Cargo fmt](https://doc.rust-lang.org/cargo/commands/cargo-fmt.html) 指向rustfmt後端。140×2=280加20=300，不是商品小計已達免運門檻；assertions_solution計算與兩項檢查相符。 |

七份參考答案的所有實際操作都在對應連結之前的已讀能力內；沒有 reference、Box、dyn、String、泛型、closure 或後章 third-party API。body 外的來源標記是教材取片段用的 comment，未讓它承擔需要讀者知道的新運作機制。

## 證據與結論的邊界

本輪獨立列必要 N1（整數／浮點除零的限定）、N2（門檻實驗的命令選取）、N3（計價／加總輸入契約範圍），可選 I1（字母計數）與 I2（continue 可觀察性）。主要數字、正常 source、三個失敗 fixture 的性質、threshold failure 的因果與 module 公開界線均有正文及原始契約依據。

本角色未自行執行全套正常範例／fixtures／threshold mutation；相應通過主要是 source與規則核對，root告知的執行另有其紀錄，不混成自身執行。唯一新增實測是浮點除零疑點。沒有以「編譯過」宣稱教材全部已教、沒有以一次輸出宣稱任意i32正確，也沒有把未看頁面說成SVG可讀性通過。

此稿是互看前來源初判。後續交叉與修後結果須另存，不回寫為本輪已理解或已修妥；範圍僅第1.4–1.10，不對第2章之後作驗收。
