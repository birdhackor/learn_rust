# 第8章作者交接：錯誤、格式化與轉換

目前8.1～8.6全部作者稿、來源與限定檢查已完成並依序凍結，供root實稿交接；未宣稱網站版面或獨立讀者驗收。root先依實際1～6章放行前兩節，之後交接實際7章，才依8.3→8.4→8.5→8.6順序實作與凍結。

依AGENTS及clear-tutorial撰寫。實際1～3章完整重讀、4章七頁與必要source的親讀沿用第5章作者紀錄；5章由本作者完成。收到root第6章全章正式交接後，親讀6.1～6.6全文與其全部完整snippet來源，共34檔，指紋在`ch08-authoring/actual-ch06-read.json`。沒有用outline或ledger代替前綴正文。

## 前綴與首次契約

| 必要能力 | 實際已教與本節責任 | 能判斷的行為 |
| --- | --- | --- |
| 8.1具體錯誤enum、完整Result | 4.1／4.2／4.5 enum與owned／borrowed pattern；5.2 Result兩型別參數；6.2 Debug／PartialEq／derive | 不合法原值與原因一起交給呼叫者，Err不是停止指令 |
| 8.1 panic、unwrap／expect | 本節先教停止正常流程，receiver消耗／Copy條件、Err要求Debug、None與Err的各自失敗行為，再短例與完整fixture | 固定保證與可預期外部失敗政策不同；expect訊息需給預期理由 |
| 8.1整份Result比較／Debug | 6.2整份Option比較；本節在首次Result assertion前明說T/E兩者的PartialEq及Debug條件 | 比較Ok／Err形態再比內容；assertion失敗可顯示左右值 |
| 8.2 type／pub type | 1.8 tuple、2.6公開module／use、6.4 associated type；本節先說module型別別名與associated type的角色不同 | alias不產生新型別、不新增驗證、不改move／Copy |
| 8.3 Display／Formatter／fmt::Result／write!／format! | 6.1 trait impl、6.2 Debug與文字比較、7.2／7.3 placeholder與含借用模型、8.2 alias；各自先說用途／借用／結果再source | 業務錯誤可正常顯示，寫入錯誤另有fmt::Result；同一值可借讀顯示並另存String |
| 8.4 From／Into／TryFrom | 6.4 associated type、3.1欄位公開、8.1政策與8.3 Display；先說方向／infallible／prelude／i64，再驗證Count | 普通整數需檢查，Count取回i32不失敗；value借讀與into接手不同 |
| 8.5 FromStr／AsRef／ToString／ToOwned | 7.4 receiver輸出、6.5 Borrow鍵身分、8.3／8.4；每個API先說輸入借用及輸出型別 | 整數與件數失敗分開；借view與獨立owned產物不同 |
| 8.6 ? | 2.2 return、8.5完整From封裝；先手動match、Result早退／轉換，再Option早退與identity From | helper失敗時不做後步，呼叫者仍能回報並繼續；相容回傳型別與轉換缺一不可 |

6.6實稿沒有無初值`let count;`宣告，而以block尾端值交回owned map。8.5及其後複製的parsing測試都改用`let count = { ... };`，不把舊分析的延後初始化假設當成已教。7章實際來源親讀完成後才使用對應能力，未倒灌進早期target載入閉包。

## no-lib來源隔離與舊內容保全

`examples/intro-workspace/error-stages`的package名為`intro-error-stages`，manifest明寫name／version／edition，沒有dependencies或lib。先建合法manifest與println-only main，root接入members與lock並通知後才可Cargo；本作者不修改shared workspace、lock、nav、checks或其他章freeze。

8.1有policy、unwrapping、policy_answer三個targets，各自在`examples/名稱/main.rs`宣告`pub mod policy;`，載入同層`policy/mod.rs`完整快照。8.2 aliases只載入自己的aliases與policy；alias_is_name／alias_answer自足單檔。每份policy都只有CountError／checked_count與兩項測試，不含alias／Display／conversion／parsing／?。

保留舊政策0／100合法、−1／101分原因與原值、逐筆繼續、固定保證的unwrap／expect、Some中的borrowed數字、缺Debug的E0277，以及三份預期runtime失敗。panic_contract多加前後觀察行，直接看到停止後的那行不執行。答案多加整批完成訊息。alias保留同一Result直接交接、負數不被名稱驗證、tuple小計240。所有正常根／helper／tests／答案／四份fixtures都在各自當頁列完整真實snippets，沒有只外連答案。

每份compile-fail／runtime-fail都有同名非空.expected。fixture依repo固定2024工具鏈檢查，診斷只比對穩定字串，不把位置／完整Debug外觀當跨版本協定。

## 檢查與freeze

8.1／8.2共有27項限定檢查通過：六個examples與default binary各自run／Clippy；四個帶policy快照targets各有兩項測試通過，cfg(test)來源也通過Clippy；unwrap_without_debug為E0277，三份runtime fixture先編譯成功、再以非零結束且符合各自.expected；package fmt通過。六組text輸出逐行匹配實跑logs。

兩頁以真正pymdownx.snippets／details展開，包含所有完整helper與折疊答案，沒有未解析引用或把泛型角括號當HTML tag。這是作者自查，**未驗證**全站建置、實際網站桌面／手機版面與獨立首次閱讀。沒有因本文自查而改寫6章或5章freeze。

本階段以`ch08-authoring/early-freeze.json`固定兩頁及現有no-lib來源；`early-validation.json`保存27項命令與logs，`rendered-early/result.json`保存展開證據。兩份planning檔的本階段內容另快照保存，之後追加章內進度不假稱早期ledger從未改動。

外部原文親讀範圍與取捨見`sources-ch08.md`；完整22份cache／range／hash保存於`ch08-preparation/external-sources/sources.json`。來源範例沒有直接執行，其閱讀證據與repo執行證據分開。

## 8.3接續交接

收到7章實際gate後，完整親讀7.1～7.4四頁與全部完整snippet source／manifest／fixtures／expected，共39檔，證據在`actual-ch07-read.json`。其中placeholder、reference欄位、lifetime impl及receiver輸出來源都是真實前綴；另實讀root在6.2新增Debug借讀summary、不取得ownership的窄說明。

8.3先說Display的用途與reference實作，再分開`&mut Formatter<'_>`的外部mutable借用和內部借用模型；fmt::Result的unit成功／寫入失敗契約在首次impl前。write!只把文字交給給定出口，業務CountError不等於fmt::Error；format!的owned String及借讀契約在首次測試／caller前說明。

display target只載入自己的policy與formatting，完整兩helper與三項測試同頁；display_answer自足，101／100與151／150兩筆確認訊息取自欄位。沒有conversions／parsing／?倒灌。七項selected checks通過（兩targets run／Clippy、三tests、cfg(test) Clippy、fmt），兩組輸出匹配，真正snippets/details展開成功；未做網站或獨立讀者驗收。`stage-03-freeze.json`固定到8.3前綴，早期原freeze與ledger快照保留。

## 8.4接續交接

8.3完整凍結後才把8.4 target放入repo並實作正文。先教From／Into的型別參數、來源／目的方向、infallible契約、i64與標準比較能力、prelude／明寫use，再完整from_into。私有field與借讀value先於Count來源，TryFrom的Error回收實際6.4，而不重稱associated type首次介紹；Self／Self::Error及欄位簡寫均先說明再snippet。

conversions／conversion_answer各自只載policy＋formatting＋conversions，全三helper與tests在同頁完整列出。formatting是測試及Err arm顯示的真實依賴，沒有省略這份impl。每個target六tests；加from_into共三examples。十項selected checks通過，三組輸出匹配，完整snippets/details展開。`stage-04-freeze.json`保留到8.4的完整前綴，8.1～8.3頁／source hashes未變；未驗網站或獨立讀者。

## 8.5接續交接

先讀8.4完整freeze後新增本節source，保留五個examples：integer_parsing、parsing、as_ref、owned_text、parsing_answer。parsing及答案只載policy／formatting／conversions／parsing；owned_text只有policy／formatting；另外兩例自足。沒有propagation／?／closure／Iterator／map／map_err穿透。FromStr／parse／ParseIntError契約、兩種具體原因與From包裝在首次source前；AsRef的receiver借用承接實際7.4，&&str參數、Borrow鍵身分差別與ToOwned具體產物先說再例子。

原parsing測試的無初值let改成已教block尾端交回owned Count，保留來源String結束後仍能使用結果的目的；所有測試、完整helper與答案同頁。parsing／parsing_answer各十tests，owned_text三tests。十五項selected執行／Clippy／tests／cfg(test)／fmt通過，五組輸出匹配。

初次HTML自查抓到普通段落的parse::<Count>等角括號沒有code span；只補Markdown後重新展開，Rust與原成功logs未改／未重跑。原工具結果15項均PASS，結構化紀錄在該HTML assertion後補存並明記恢復方式，不冒稱第一遍整個pipeline已通過。修後snippets／details及泛型tag檢查通過。`stage-05-freeze.json`固定到8.5，原8.1～8.4頁／source hashes保留；未驗網站／獨立讀者。

## 8.6與全章交接

8.5凍結後才新增propagation與propagation_answer，它們此時才載入完整六module。手動match先於?；Result的取值／當前函式早退／From轉換先於短例，呼叫者逐筆繼續與最後訊息由實跑呈現。Option?先說None回傳再source；Option-only target改為自足單檔，只有second_number與main，沒有為查第二筆而載入整個解析閉包。答案的tuple與相同型別identity From均在首次使用前說明。

保留兩份E0277 fixture：unit回傳不能接這個?、缺少ParseIntError到CountError轉換；兩份都附實測.expected。十三項selected checks通過，三組輸出匹配；完整snippets/details展開。propagation與答案各十二tests，option_question無test函式，其正常輸出直接核對。

全章共19個examples，對應舊18個examples及舊主要程式；另有println-only default binary。獨立helper測試函式共12項，分階段快照重複保存相同內容，沒有把各份複製測試另稱不同測試設計。六份失敗fixtures皆有.expected；全章沒有lib、第三方依賴、workspace設定繼承、closure／Iterator／map／map_err／and_then或std::error::Error。

分階段共72項限定命令通過，19組正文text輸出與logs一致。每階段freeze之前核對前綴頁／source hashes未變，保留early及stage03～06原freeze與各期ledger快照；`freeze.json`為全章最終owned版本（另納入最終兩份planning檔），`validation.json`彙整各階段證據。source-closure.json逐target記實際module與同頁完整來源覆蓋。沒有執行全intro all-targets、全書checks、全站建置、網站視覺或獨立首次閱讀；上述限制留給root安排相應驗收，不把作者自查稱全書通過。
