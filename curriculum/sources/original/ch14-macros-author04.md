# 第 14.1–14.3 作者 fresh 來源閱讀

作者：chapter04_traits；日期：2026-10-14。先前只研究的 `ch14-author04.md` 保留，這份是 actual gate 開放後重新取得、親自閱讀的材料，不能以舊研究代替新 API 核對。

## 14.1 開稿前實讀

網路閱讀 tool 的三個 open 長時間未回應，已停止該次唯讀呼叫，改以環境既有 HTTP proxy／CA 的 urllib 取得原 URL，未繞過政策或關閉 TLS。四頁均 HTTP 200。原 HTML、抽出正文與實際 URL/hash/取得時間保存於 `/workspace/work/tutorial-implementation/research-ch14-macros-author04/` 的 `fresh-http-sources.json`。下載與閱讀分開記錄，尚未讀的 procedural 段不算已教／已核對。

| 來源 | 本次真正已讀範圍 | 借鑑與先備陷阱 |
| --- | --- | --- |
| [English Book：Macros](https://doc.rust-lang.org/book/ch20-05-macros.html) | fresh text 1–155：macro/function 比較、整個 declarative 例、proc 段 TokenStream 導入；尚未 fresh 讀完其 derive/attribute/function-like | 從既有使用回到展開；先固定生成的普通 Rust 答案。不能沿用普通函式無法處理變長資料的概括：slice/Vec/泛型可處理；本稿理由是宣告樣板重用 |
| [簡體 Book：Macros](https://kaisery.github.io/trpl-zh-cn/ch20-05-macros.html) | fresh text 1–70：declarative 全段、proc 導入與 derive 開頭；後面待 14.3 再實讀 | input pattern 與實際展開的對照可學。關於未 macro_export 就不能引入的概括不拿來教本地 textual scope；同型 Vec 的元素限制也不能省掉 |
| [Reference：Macros by Example](https://doc.rust-lang.org/reference/macros-by-example.html) | fresh text 1–268：語法、匹配/展開、metavariable、edition、repetition、scope 導入與部分 textual scope | 本稿選 ident、單層 repetition 的 +/*/?，不搬巢狀 matcher/fragment forwarding。matching 是語法；第一條匹配後 Rust 錯誤不會回退找下一條。後續 hygiene 範圍將另實讀 |
| [Reference：Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html) | 已 fresh 取得 264 行，14.1 尚未重新逐段閱讀 | 14.3 開稿前須讀其 crate 角色、TokenStream、三種輸入輸出與錯誤契約，不能把已下載當已核對 |

## 最小任務與不照搬的決策

Meters、Seconds 都維持 8.9 已教的不同 newtype 身分；Reading 只借讀 i32。普通 Rust 的 struct/impl 先給答案，macro 只是依 identifier 產生同一份樣板，再將單一名稱改為清單。由 same task 的宣告重複引入 macro，而不是虛構函式不能接不同數量的同型資料。

14.1 的 Reading 名稱由已教 use 匯入；macro 本地先定義再使用。14.2 才加 exported macro、$crate、名稱解析與診斷，會另起 source，不回填第一頁例子。14.3 預定從相同 Reading impl 轉到 derive；proc crate/consumer 另 package，讓 14.1 manifest、完整答案不被後節加進未知依賴。

本次先固定專案的 Rust 1.99.0／edition 2024 契約；14.1 不使用第三方 macro crate。未將 Book 的編譯階段概括當成所有型別檢查先後的精確模型：只承諾 macro 產生的 Rust 還要通過型別/ownership 檢查。

## 後續固定 API 核對待辦

14.3 需 fresh 查固定 syn／quote 的實際頁，核 DeriveInput、Data/Fields、Generics/Punctuated（若選用）、parse_macro_input、Error、位置資訊、quote 的輸入/輸出與 conversion。現在共享 Cargo.lock 為 quote 1.0.47、syn 2.0.119 與 3.0.9、proc-macro2 1.0.107；這是觀察，不代表本作者已選定依賴或使用白名單。

在 14.1 先完成後才接受 14.2 actual gate；14.2 完成後才接受 14.3 actual gate。每節的新教材／正式 API 讀取與實際範圍在此追加，不改舊研究為已實作紀錄。

## 14.2 開稿前 fresh scope／hygiene 實讀

已取得 actual gate `ch14-macro-rules-prerequisite-v3`，manifest SHA a0bf97a3ce622125803a978dbb0d0c600dc8eb7da703feead2b54eccc7a2e441。親讀全部6BODY units並核指紋；其Reading/baseexamples以原已讀source及root兩項最小修正後的實際版本為先備，不將未教super算白名單。

- [The Little Book of Rust Macros：Hygiene](https://lukaswirth.dev/tlborm/decl-macros/minutiae/hygiene.html)：fresh80行全文，涵蓋syntax context、caller插入片段、同名local、$crate與non-macro item路徑。
- [對應中文譯本：衛生性](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/hygiene.html)：fresh91行全文，與英文對照，名稱上下文的顏色方法可借鑑。正文仍用hygiene；不把中文「展開期間呼叫是空」誤教成總是產生空程式。
- Reference by-example接續fresh讀269–674，至全文末：textual/path scope、export、hygiene、$crate/privatevisibility與follow-set。未把不用的macro_use/local_inner_macros、label或完整follow-set列表搬入正文。

兩份TLBORM來源HTTP200、raw/hash/取得時間與抽出全文在同一work研究目錄的fresh-hygiene-http.json；Reference沿用本次fresh raw。API固定本repoRust1.99/edition2024：macro_export從crate root提供publicmacro；$crate鎖定定義crate，不給caller私有存取權；輸入expr保留caller的syntax context；template local與caller同拼法可以不是同一binding；item名字不因此自動唯一。

教法沿同Reading任務：公開模板讓caller產生newtype、兩份borrowed來源分別交i32；正反例顯示裸Reading與$crate::Reading的差別。普通pair函式先完成同任務，private取值helper供visibility對照。expr輸入只示範對既有owner的reference，不需要未教的temporary lifetime extension。每個輸入在template出現一次是求值次數的理由，hygiene不替次數承諾背書。compile_error/tt/rule順序在使用前獨立教，再隔離自訂輸入診斷和成功匹配後的E0428。

另fresh核對[Reference：operand求值順序](https://doc.rust-lang.org/reference/expressions.html#evaluation-order-of-operands)的選定段：tuple operands依source左到右求值，支撐本模板let左輸入→tuple左只讀既有reference→右輸入的次序。raw與實讀段文字保存在reference-expression-order檔；沒有沿用該頁舊註解中「owned array尚無iterator」的過時主張，也沒有將後續place/value全文算已讀。

## 14.2 fresh 範圍與 actual gate

14.1 actual gate為ch14-macro-rules-prerequisite-v3，親自核對當下完整package與已讀來源一致；Reading及前頁4個example/答案不回填。14.2開稿前新增fresh實讀：

- [TLBORM：Hygiene](https://lukaswirth.dev/tlborm/decl-macros/minutiae/hygiene.html)，全80行；[中譯對應](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/hygiene.html)，全91行。兩份HTTP200原文/hash保存在fresh-hygiene-http.json。借鑑以同名local區別syntax context；傳入的tokens保留caller來源。中譯「呼叫展開實際是空」不作字面規則；macro會產生語法，不是必定空展開。
- Reference by example補讀269–674全部，累計全文已讀：textual/path scope、export在crate root、mixed-site hygiene、$crate與visibility、follow-set限制。只選本例用到的scope與expr/tt；不把pub(crate)、macro_use、labels或巢狀follow-set例子搬入主文。
- 9.2全部Rust source/Cargo本輪已回讀（5份），確認build script的host/target與deps角色；第6份SVG尚未本輪重新親看，沒有把它算新增視覺驗證。

本頁以同一Reading/type模板進入跨crate。普通函式可讀兩份reference；macro範例用兩個同名value的不同來源說明hygiene，再以錯誤macro對照crate path。錯誤訊息的用途是分清matcher拒絕與生成Rust拒絕，不保證所有rustc版本的完整排版一致。實際宣告/API在code前教，crosscrate失敗由隔離Cargo正常載入依賴，核預定診斷。

## 14.3 fresh 閱讀：自訂 derive

取得14.2 actual gate ch14-hygiene-prerequisite-v4後，重新實讀全部7 BODY units。manifest SHA 6f28d31407a51b69fb56e81bd16e6f6143e5ee38f361f9518fa7af6d174e42c9，正文 SHA 33badb9e88f8aa93d021b8407d792e48456254c77f8d46441b2436e04fca4ca8；此為作者回讀，不聲稱新盲讀。重新讀6.2全部8units及associated.rs；9.2的第6份build-phases.svg原始碼亦已回讀，未因此宣稱新實頁視覺。

- [English Book：Macros](https://doc.rust-lang.org/book/ch20-05-macros.html)，本輪補讀156–433，連同先前1–155為全文實讀；[簡體Book對應](https://kaisery.github.io/trpl-zh-cn/ch20-05-macros.html)，補讀71–199，連同先前1–70為全文。參考derive輸入→AST→quote輸出的分工與兩個library／consumer配置。不同於其hello名字任務，本文延續Reading借讀；不採「runtime不能取得型別名」的動機。兩版都把panic寫成必要，這不符實際可回傳compile_error語法的契約；本文不抄unwrap/expect，而將語法解析失敗與有限形狀拒絕變成編譯診斷。完整syn AST長印出並未解釋所有欄位，本文只畫當下使用的ident、params與data/fields。
- [Reference：Procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html)，本輪全文1–264已讀：crate-root、proc-macro設定、TokenStream/Span、輸出新增或替換、unhygienic名稱解析、診斷與tokens。derive保留原item，追加impl；attribute替換原item。union與helper attributes不是本例支援項，不在學生code偷渡。
- fixed API由網路fresh取得，HTTP200、URL/raw SHA與text保存在work/research-ch14-macros-author04/fixed-api/fetch.json。實讀[syn 2.0.119 DeriveInput](https://docs.rs/syn/2.0.119/syn/struct.DeriveInput.html)1–63、Data1–40、Fields1–70、Generics1–43、FieldsUnnamed1–27、Punctuated1–52、Error1–141及parse_macro_input全部54行。所選契約：derive型別feature；params與unnamed不是Vec而是含分隔符的Punctuated；len不算逗號；parse_macro_input需parsing+proc-macro且所在函式回傳compiler TokenStream；Error::new和into_compile_error不用printing（new_spanned才要求）。拒絕helper只使用type名字的span。
- [quote 1.0.47 quote!](https://docs.rs/quote/1.0.47/quote/macro.quote.html)全文167行，含插值/hygiene/return type與Ident例子；proc-macro2 1.0.107 Ident1–87，TokenStream介紹與兩方向From轉換（130–140）；Rust1.99 compiler TokenStream介紹1–45。#name使用ToTokens，不是文字替換；quote與Error產物是proc_macro2::TokenStream，最後.into()轉compiler型別。兩種同名型別不混為同一個，consumer執行與macro編譯時執行亦分開。
- syn/quote的精確版features頁亦實讀feature內容；syn關閉預設只開derive/parsing/proc-macro，quote保留預設proc-macro bridge。不啟用full/extra-traits/debug AST，不跟本repo其他依賴的syn3混用。獨立fixture後續會核最小features真的能編譯，不只依global feature聯集。

教法取捨：把已有兩份tuple struct保留下來，由ReadingValue derive增加相同impl；普通手寫impl與14.1模板都能完成，差別是使用已有宣告作輸入。有限支援nongeneric單欄tuple struct；欄位能否交i32留Rust型別檢查，避免按i32字串比對而錯拒alias。proc macro輸出使用明說的::ch14_macros::Reading外部crate路徑，consumer依賴保留此名稱；不假裝可使用macro_rules的$crate、不暗加crate rename工具。初學讀者先看角色與設定、最低AST/quote/Error API，再讀完整entry，最後分階段核診斷。
