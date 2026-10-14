# 4.4–4.5 作者依賴與驗證紀錄

作者：`chapter02_ownership`。本作者只寫第 4 章的 4.4–4.5；4.1–4.3 的原作者檔案不修改。作者研究已追加於 `planning/source-notes/ch04.md` 的分工作者段落，保留原研究紀錄。

## 本輪實際前文

root 釋出 `/workspace/work/tutorial-implementation/frozen/ch04-trait-prerequisite-v1/`（4.1 的 8 個閱讀單位與 sources）。作者已實讀目前第 3 章六頁、4.1 全文，以及 `ch03-data/examples/strings.rs`／`utf8.rs`／`src/text.rs`、`ch04-traits/src/charges.rs`。第 1 章先備已在第 2 章寫作時讀完，第 2 章由本作者實作。

只採用已實際可用的操作：

| 必要先備 | 實際位置 | 本輪使用 |
| --- | --- | --- |
| ownership／shared reference Copy／mutable reference reborrow／scope 與 lifetime 不同 | 2.2–2.4、2.6 真實正文與 source | 交回 reference、短 block 的來源限制、借用資訊與原資料的區別 |
| struct／impl／receiver／Self／associated function | 2.1、2.5 | 4.5 存 reference 的 model 與 method |
| enum／pattern／`_`／Option 與泛型 | 3.1–3.3 | 來源候選的 Option、typed wildcard 參數與缺值選 backup |
| String::new／push_str／as_str／String 無 Copy／str literal 與 shared reference | 3.4「先認識建立、追加與借用的接口」及 `strings.rs` | 建立來源文字、顯式借出 str、重建 owned 結果 |
| str.len 的 byte／usize 契約 | 3.5「先決定正在數哪個單位」及 `utf8.rs`／`src/text.rs` | 以 byte 數字選來源與數值 assertion；不把 byte 長度當 lifetime |
| pub struct／公開欄位／pub trait／trait impl／泛型 bound | 實際 4.1 與 `charges.rs` | 4.5 的公開 borrowed model；第 5 章後續才回收 trait method 的介面角色 |
| module／use／library 與 examples／numeric tests／assert／cfg | 1.9–1.10，3.5–3.6 實際 library tests | 獨立 ch04-lifetimes package 的 source module 與 unit tests |

尚未把 4.2 derive／Clone／Debug／PartialEq，或 4.3 HashMap／Borrow 當先備；本稿未使用這些能力。

## 4.4 首次使用 ledger

| 概念或 API | 第一次需要 | 正文在 code 前的介紹位置 | 讀完能解釋什麼 |
| --- | --- | --- | --- |
| 函式回傳 borrowed &str／單一來源 elision | same 函式 | 「單一來源，省略的關係也存在」 | 回傳借用不是重建文字；來源 owner 仍由 caller 保持有效 |
| 省略輸入 lifetime 各自分開／兩來源輸出 E0106 | ambiguous_output fixture | 「兩個可能來源，不能略去來源關係」 | 介面要說可能來源，不按某次輸入的 byte 長度猜 lifetimes |
| `'a`／函式 lifetime parameter／&'a str annotation | longer 函式 | 「lifetime annotation 描述關係，不延長存活」 | `'a` 是共同有效要求，不讓原值同時建立／結束，也不延長短來源 |
| 來源在內層結束／E0597 | source_too_short fixture | 「原文字較長，也不能替另一來源延長 lifetime」 | 實際選外層資料也不能違反兩來源簽名；只改結果使用位置便可不同 |
| 只將第一 input 連到 output／typed `_` 參數 | first_text 函式 | 同上，在對照 code 前 | 第二 input 的有效性不一定約束結果；改功能和改標註要區分 |
| 局部 String reference 不能回傳／E0515 | local_output fixture | 「要獨立保存結果，就交出 owned 資料」 | 任意 annotation 不能使函式內 owner 持續存在 |
| copy_text：以 new／push_str 重建 owned String | copy_text 函式 | 同上，在 code 前 | 結果不借原來源；名稱 copy 不表示 String 具有 Copy |
| lifetime placeholder `'_` | same_placeholder 函式 | 「'_ 仍依省略規則推導」 | placeholder 依相同省略規則，不能為兩來源歧義提供未知契約 |
| ordinary lifetime bound `'b: 'a` | fallback 函式 | 「不同名稱的有效範圍如何涵蓋」 | backup 能供回傳借用需要的範圍；至少涵蓋可相等，不是多活秒數 |
| Option<&str> 的 Some 空文字 vs None | fallback 與測試 | 同上回收 3.2 情況表示 | 空文字是合法存在片段，不能一概改選 backup |

## 4.4 實際驗證

- package 真實 library/main/六個 example 及 manifest 已建立；root 統一加入 workspace 與 lock。
- 作者執行 main 及 六個 examples，與正文的預期輸出相符。
- package fmt／all-targets Clippy `-D warnings` 通過；library 四項 unit tests 通過。驗證包括 owned 結果在原資料修改後保持獨立，與空文字／缺值的不同選擇。
- 四個 compile-fail 的 `.expected` 都由維護 fixture checker 查核；其中 lifetime bound 的 rustc 診斷沒有 E 碼，保存其 `lifetime may not live long enough` 主句作契約。此次累積 checker 檢查 29 個 fixture，不宣稱其他章由本作者撰寫或審閱。
- 新 SVG two-sources 寬 340、字 20、title／desc 齊；與相鄰正文保留 first／second／'a 對應。作者尚未看真實桌面／手機原位頁面，不將 source／建置結果等同視覺通過。
- 4.4 尚待 root 安排的獨立首讀／技術與前後銜接審閱；作者自查不當作發布驗收。

## 4.5

| 概念／API | 第一次需要 | code 前的解說位置 | 讀完能解釋什麼 |
| --- | --- | --- | --- |
| struct lifetime parameter／reference 欄位 | TextView<'a> | 「欄位保存 reference，原文字仍有自己的 owner」 | 模型持有 reference 值，但沒有取得原文字 ownership；資料存活要求不因裝入模型改變 |
| `TextView<'_>` 路徑 placeholder／回傳 borrowed model | make_view | 「函式回傳含借用的資料模型」 | 路徑 lifetime argument 依唯一來源推導；模型裡的 reference 借自 input |
| `impl<'a> TextView<'a>` 的宣告／使用位置 | view-methods | 「借用模型，與模型借用原資料，是不同關係」 | impl 宣告參數與型別使用參數各有角色，不是建立固定全域模型 |
| receiver 的另一份借用／method output elision | TextView::text | 同上，在 method table 與 code 前 | 借用 &self 的有效性與欄位來源 'a 不必同長；省略輸出依 receiver 契約 |
| 顯式接到來源的 `original -> &'a str` | method_source 正常程式 | 同上，在 method table 與 code 前 | 交回的是 source reference，模型可先結束但原來源仍須存活；body 的 reference 欄位可 Copy |
| receiver 太短 E0597 | method_receiver_too_short | 正常 method_source 後的契約對照 | 相同資料與 body 不代表相同 output lifetime 契約；變成 text() 後 view 本身需滿足結果使用要求 |
| 多個獨立來源 `TextPair<'a, 'b>` 與來源 getter | two-view／two_models | 「多個欄位可以借自不同來源」在 code 前 | 兩個 input 的來源可分開，first_source 結果不必與第二來源或模型一起結束 |

真實驗證：

- 4.5 正文引用四個實際 examples 與 views module；第一個 borrowed_model 在 method 尚未介紹前只讀公開欄位／既有 str.len，沒有超前呼叫自訂 method。
- 所有本節新方法與路徑 lifetime argument 均先教角色再展示 code；沒有使用未讀的 4.2 derive／Clone／Debug／PartialEq，亦沒有借用 4.3 HashMap／Borrow 作先備。
- 作者實跑四個新增 example，輸出與正文一致。新增兩個 compile-fail 契約都是 E0597；維護 checker 此次累積驗證 35 個案例，其中本 package 有六個 compile-fail。
- 新 module 加入後，package fmt／all-targets Clippy `-D warnings` 通過；library 共六個 unit tests 通過，本節增加兩項，覆蓋 borrowed model 的來源和第一來源越過另一來源／模型結束。
- borrowed-view SVG 寬 340／字 20，含 title／desc，圖與正文都用 view.text／text 區分 reference 欄位與 String owner；尚未由作者看真實原位頁面，仍待 root 整合版面檢查。
- 教材先備與技術的作者自查已記；完整首讀／技術與銜接審閱另由 root 記錄，不把本 ledger 當成獨立審閱通過。
