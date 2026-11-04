# 新第 7 章來源、實讀範圍與採用界線

日期：2026-10-15。作者實讀紀錄；不是獨立審閱或外部教材逐字翻譯。準備階段的來源原文、HTTP/文字指紋保存於 `/workspace/work/tutorial-implementation-20261015/ch07-preparation/`；原始研究筆記保留當時 gate 尚未就緒的狀態，正式放行與實際前綴另記 author-ch07。

## 英文、繁中與必要 Reference 原文

| 實際來源 | 親讀範圍（本次 cache 文字行號） | 採用與未採 |
| --- | --- | --- |
| [Rust Book EN：Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) | book-lifetimes-en.txt 全文 1–809；其中 annotation 131–475、borrowed struct 477–512、elision 514–648、method 650–729。 | 採回傳需要→省略歧義→標註→合法與過短來源；採單一來源／區域值／owned 結果對照。未照搬「較短的 scope」當精確定義、延後初始化、String::from、split/iter/unwrap；static 與 trait 綜合例不帶入本章。 |
| [繁中 Book：透過生命週期驗證參考](https://rust-lang.tw/book-tw/ch10-03-lifetime-syntax.html) | book-lifetimes-zh-hant.txt 全文 1–551；annotation 84–322、struct 324–346、elision 348–430、method 432–496。 | 同樣借用關係，用本書術語 lifetime/reference；不把不同版本的繁中診斷視為固定工具鏈測試。 |
| [Rust Reference：Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html) | reference-lifetime-elision.txt 1–94；後續 trait object/default static 未讀作本章來源。 | 核對輸入分別推導、唯一輸入連輸出、receiver 連輸出、`'_` 依同樣規則推導。沒有把範例裡的 dyn/Fn/HRTB 帶入教學。 |
| [Rust Reference：Trait and lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html) | reference-trait-bounds.txt 193–216，只讀 lifetime bound 與相鄰 T:'a 定義。 | 只採 lifetime-to-lifetime 的至少涵蓋且允許相等；T:'a、implied bounds、HRTB 不在本章教。 |
| [Rust Reference：Generic parameters](https://doc.rust-lang.org/reference/items/generics.html) | reference-generic-parameters.txt 85–125。 | 核對函式／impl lifetime 宣告位置與 placeholder 不是可宣告的新名字；const generics、where 等未帶入。 |

`research/fetch-manifest.json` 保存原頁 URL、HTTP200、原 HTML 與正規化文字 SHA-256；`preparation-hashes.json` 與 source-coverage.json 保留該次研究版本。上述每頁已實際讀過所列範圍；不是只下載或只讀二手摘要。

## 舊正文與真實 source 的 preservation

已全文讀舊 `04-04-lifetime-annotations.md`、`04-05-borrowed-models.md`，及 `examples/ch04-lifetimes/` 全部 manifest、Rust 與 expected。兩個舊 SVG 讀過原始碼；新版僅重畫必要的 borrowed field 圖，不假稱看過舊版頁面。舊來源指紋見 preparation/source-coverage.json。

| 舊內容／來源 | 新位置與快照界線 |
| --- | --- |
| source.rs same/longer；single_source；原主程式；ambiguous_output/source_too_short | 7.1 single_source、source_basic（只有 same/longer）、兩個完整負例。交換參數與等長內容輸出都保留；新增同頁 shorter_answer。 |
| first_text；source_scope | 7.2 自足 source_scope.rs，簽名與兩種使用位置先教再完整 run。不偷載 placeholder/outlives。 |
| copy_text；owned_output；local_output | 7.2 自足 owned_output.rs、完整 E0515 fixture；增加來源修改後結果不變的可見對照，只用已教 String API。 |
| same_placeholder；placeholder | 7.2 自足 placeholder.rs；先解釋 `'_` 依省略規則再執行。 |
| fallback；outlives；missing_outlives | 7.2 自足 outlives.rs 與完整負例；明辨 Some 空文字、None、至少涵蓋並允許相等。 |
| first_nonempty；lifetime_annotation_solution；source.rs 四 tests | 7.2 source_complete/main.rs＋source/mod.rs，整個 helper 只在全部功能已教後提供／執行。練習完整答案同頁；此階段 tests 保留數值／bool。 |
| TextView/make_view/byte_len；borrowed_model；model_source_too_short | 7.3 view_basic，只含這三項，完整負例與 borrowed field 新圖同頁。沒有 text/original/TextPair。 |
| 原 borrowed_model_solution 的 Label::original | 保留到 7.4 label_answer；7.3 另寫只回傳 byte_len 數字的 view_answer，不提早編 receiver-output 契約。 |
| text/original；method_source；method_receiver_too_short | 7.4 先用 receiver_only helper，只含 TextView 與三個已教 method。自足 E0597 反例指出 view 過短，與原 String 過短分清楚。 |
| TextPair/兩來源方法；two_models；views.rs 兩 tests | 7.4 TextPair 先教，再提供 complete/src/views.rs、two_models.rs。兩項 test 分別驗證原來源可跨越模型、第一來源可跨越第二來源與模型。 |
| 完整 src/source.rs、src/views.rs、src/lib.rs、manifest | 7.4 `lifetimes/complete/` 專用 library package，同頁顯示全部檔案；測試改用 actual 6.2 已教文字比較，補等長選第一段。早期父 package 無 lib，避免 selected example 暗中編到 future source。 |

## 教法取捨與驗證界線

採固定 outside/inside、source/view/receiver 對象，讓內層結束前後的使用位置可追蹤；不同規格才換函式。長度比較與 lifetime 有效性分開，不能從一次輸出推論來源契約。將同一個 body 的 text/original 直接對照，讓簽名的作用可見；不把 `&self` 一律等同欄位 `'a`。

本章沒有照搬外部教材的字串切割 Iterator、一般泛型 trait 綜合 API，沒有以 `'static` 解錯，也不宣稱所有回傳 reference 都必須來自參數。區域值 E0515 只針對當下新建、即將結束的 String；owned 替代明確付出保存另一份內容的工作。

作者 selected commands、診斷、同頁 source closure、prefix 與 final 指紋見 author-ch07。單張新 SVG 已在 Chromium 實際看過；網站桌面／手機與獨立首讀仍待 root 安排。本檔不是那些驗收的替代證據。
