# 第 11.9–11.11 節獨立技術初判

封存：2026-10-15T08:55:12.289Z。必要問題1；可選問題0。

TC11F-01（必要，先教後用的語法負擔；不是Rust錯誤）：11.9:152實際完整helper recorder/cells/mod.rs:1首次寫 `use std::cell::{Ref, RefCell};`，11.11:151的any_type_id/payloads/mod.rs:1又寫 `use std::any::{Any, TypeId};`。已讀前綴只教單一路徑use，沒有說明共用前綴後用花括號／逗號列出多項；自己已封存的395份owned Rust來源沒有先用這個形式，前綴正文也查無說明。基本Python讀者須自行猜這段新語法如何建立兩個名稱，才能逐行理解實際載入helper。最小修正是兩處各拆成已教的兩行use；也可在11.9首次snippet前解釋等同兩行，不需要引入更多API。

其餘技術核對未見問題：Cell的值交接／Copy條件、RefCell runtime guard與Drop期限、try_borrow不等待、Rc owner與guard分工、Weak父連結數字均準確。newtype的型別身分／欄位可見性、Add方向與Output、orphan/local／uncovered／fundamental、重疊impl規則對實際source成立。Any static與reference期限、borrowed/mutable/owned downcast及Box TypeId查詢對象分清。

全文、61snippets、30份成功owned Rust來源、12compile-fail+2runtime-fail的完整source/expected、18run及3selectedtest（8tests）已靜態核對。73檔SHA-256、全target closure與9份helper body比較見JSON。no-lib stage只載明示target-local module，complete才合併完整功能/tests；全部inline，沒有早期命令載未教後文。

未先讀作者驗證、peer、reader或歷史問題。沒有新增Cargo/Rust執行；全部結果與診斷是靜態核對。2張SVG只讀XML/source，不稱browser視覺或盲首讀實驗。
