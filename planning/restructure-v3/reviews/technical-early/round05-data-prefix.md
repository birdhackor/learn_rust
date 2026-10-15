# 第5章前段獨立技術初判：5.1–5.2

已依基本 Python 背景及實際第1–4章正文，順序核對 5.1、5.2 與其完整 selected source。先讀最終4.2、4.7兩處窄修，再開始本範圍；第4章交叉與本初判分開保存。封存前未讀第5章作者 ledger／validation 或其他角色的結論，也未用尚未交接的5.3–5.5及其文字／vector來源補先備。

**結果：必要問題0項；選讀建議0項。** 本輪為整頁技術審閱與source推理，沒有執行 Cargo、rustc、Clippy、測試或瀏覽器；不把作者的成功執行當成本角色實測。

## 實際載入與完整性

5.1有 generic_holder、generic_impl、generic_answer，5.2有 option_result、result_answer，共5條精確 `cargo run --example ... --locked`。package `intro-data-steps` 的 manifest只有已教的name、version、edition；沒有library、build script、dependency或repo內額外Cargo config。每個目標各載入一份完整example，沒有隱藏helper module。正文都展示完整source與所需manifest；19個snippet引用的檔案及起訖標記均存在。

intro workspace與lockfile已包含此member，根workspace排除intro路徑。別的member及同package未選example只屬這些命令的metadata範圍，不因存在就編譯它們的source。本輪沒有以all-targets或未發布source增加先備。`src/main.rs`只檢查manifest形狀；指定example不會載入它。

一份完整失敗例generic_same_type及其非空expected亦已核對：Pair<T>兩欄共用T，整數／bool衝突，靜態預期E0308與正文及expected一致；本輪未執行失敗例。

## 首次概念、規則與答案

- 5.1:7、9在首個泛型定義前區分值參數／型別參數並宣告T；26在函式snippet前連起輸入T與Holder<T>；44先解釋`::<i32>`的型別引數與值引數。具體的i32、bool例子保留相同保存規則，沒有假定任意T可運算、顯示或比較。
- 5.1:79、81先交代`impl<T> Holder<T>`的宣告／使用位置、Self及owned self取值，再展示實作。String例沿用3.8已教的建立、push_str、as_str，能顯示非Copy的值move進出盒子；原值沒有被誤用。一個T對應兩欄的限制在失敗例前已說明，兩個T、U與欄位的分工在答案前也明確。
- 5.2:7–11在首次source前定義Option<T>、Some／None、缺值與零、prelude及標準variant可直接使用的理由；19清楚說None的推導線索。match、if let及if expression均直接沿用實際第2、4章，沒有提早引入未教的組合方法。
- 5.2:41–50在divide前說清Result<T,E>的兩個角色與完整簽名；56先解釋Err是資料與呼叫者處置，再展示nested pattern。divide避免零除數；62明確限於不溢位小整數，沒有把這份函式宣稱成處理全部i32邊界。成功0與失敗／缺值的區別保持一致。
- 兩份答案均符合題意，helper完整且沒有新trait、derive、whole-enum equality／formatting或未教Clippy allow。result_answer的assertion只針對題目指定的0、−5取出的primitive值，不錯稱對任意輸入都成立。

5個目標的輸出順序及primitive assertions均由實際source靜態核對，與正文相符；逐目標輸出、完整閉包、概念四題依據及20份SHA-256存於同名JSON。沒有未解的型別／所有權／結果疑點需要重跑指令。本結論限於5.1–5.2的技術與首次必要依賴，不包括實際版面或目標讀者實驗。
