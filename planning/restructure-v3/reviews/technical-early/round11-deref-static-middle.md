# 第 11.4–11.5 節獨立技術初判

封存：2026-10-15T08:33:28.920Z。必要問題 0；可選問題 0。

以實際 11.1–11.3 為前綴，逐頁讀完兩節及全部 actual selected source。核對 27 個 snippet、7 個 run target、2 個 selected test 命令（2 與 5 tests）、4 組負例與完整答案。33 檔 SHA-256 見同名 JSON。封存前未看作者驗證、peer、reader 或 historical findings，未用 11.6 以後補先備。

- Deref／DerefMut 的 Target、receiver lifetime、借用方向、* 存取位置與 ownership 的差別準確；Box 的 owned move-out 特例和自訂 Deref 清楚分開。into_inner 明確 consumes self，Wrapper 沒有 Drop。
- coercion 的 String→str、Vec→slice、mutable→shared 與 method autoderef/autoborrow，均對實際 helper、push_str/as_str 及輸出成立；借用介面不會繼承 Copy trait bound。
- T:'a、?Sized+'a 與 &'a T implied bound 都已先解釋；'static reference 的資料有效性與 T:'static 的資料依賴契約分清，未說成值必須活到程式結束。Box<&str> 的外層 ownership 不會消除內部來源。
- stage target 沒有隱藏 lib 或後續 helper 載入。Borrowed、choose_label、require_static 的早期快照是各 target 自己的完整定義；static_complete 才載入完整 helper／Wrapper 和五個 tests。各 helper 副本已實讀，basic/coercion 及兩個 complete Wrapper 對應副本 bytes 相同。

沒有新增 Cargo/Rust 執行；輸出及 fixture 錯誤碼均為靜態核對。deref-coercion.svg 只檢查 XML/source，不稱視覺驗證或首讀實驗。
