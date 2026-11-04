# 新第 2 章作者來源實讀紀錄

角色：新第 2 章作者，不是獨立技術、銜接或首次閱讀審閱者。日期：2026-10-15。下列外部來源於準備階段實際抓取與閱讀；root 正式放行後，動筆前又親讀新 1.1–1.6、1.8 正文及相應實際 source，沒有以準備摘要取代前綴。新章 2 實作與執行證據另見 author-ch02.md。

## 閱讀方式與證據

以下均於本任務實際透過 HTTPS 取得並讀取對應內文。`/workspace/work/tutorial-implementation-20261015/ch02-preparation/research/` 的 `.html` 是抓取原始資料，`.txt` 是本次從 `main` 內容正規化後的文字；行號只對本地 `.txt`。完整下載不表示全文實讀。兩份 fetch manifest 保存 URL、HTTP 200、SHA-256 與長度。沒有開瀏覽器觀察外站版面。

英文為 The Rust Programming Language（Rust Book）現行官方版，繁中為 rust-lang.tw 的《Rust 程式設計語言》。繁中頁與現行英文頁有版本差異，不能將繁中顯示的自動產生樣板或安裝預設當成固定工具鏈的驗證結果；正式寫作依本 repo 的固定工具鏈與實測。

| 新節 | 英文來源與實讀範圍 | 繁中來源與實讀範圍 | 借鑑與不採用 |
| --- | --- | --- | --- |
| 2.1 | [Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)，`book-data-types-en.txt:205–221`；[Operators](https://doc.rust-lang.org/book/appendix-02-operators.html)，比較與 `!`/`&&`/`||` 表格列（20–36、96–108、126） | [資料型別](https://rust-lang.tw/book-tw/ch03-02-data-types.html)，`book-data-types-zh-hant.txt:138–150`；[運算子](https://rust-lang.tw/book-tw/appendix-02-operators.html)，相應表格列（15–31、87–99、117） | 先區分數字問題與 `bool` 答案。英文表格明列 short-circuiting，繁中表格只寫邏輯 AND/OR，故跳過右側的機制另查 Reference。表格的 trait 欄不搬入早期正文。 |
| 2.2 | [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)，`book-control-flow-en.txt:1–228` | [控制流程](https://rust-lang.tw/book-tw/ch03-05-control-flow.html)，`book-control-flow-zh-hant.txt:1–176` | 借鑑同一輸入由成立改為不成立、非 bool 的編譯失敗、第一個成立分支、`if` 值。改用既有計價情境；型別不一致的失敗例用已知 `i32`/`bool`，不帶尚未介紹的 `&str` 診斷。 |
| 2.3 | 同頁 EN `398–491` | 同頁繁中 `300–366` | 保留由整數 array 直接取得數值、減少手動索引管理的理由；不照搬未教 `.rev()`，不轉入 Iterator trait。英文對效率說 can，繁中舊版說必變慢；本章不據此宣稱所有 for 都比 while 快。 |
| 2.4 | 同頁 EN `367–396` | 同頁繁中 `278–298` | 保留每次進 body 前先檢查的順序，改用剩餘工作量與最後不滿一批的材料，使條件與狀態更新可追蹤；補一開始為零即執行零次。原文用 loop/if/break 解釋 while，本書順序相反，因此不讓尚未教的 loop 成為必要前提。 |
| 2.5 | 同頁 EN `230–365` | 同頁繁中 `178–276` | 採 `break` 結束、`continue` 跳過本次、loop 交出數值、巢狀 label 指向的對照。採終止保證明確的數值材料；不搬 thread 重試動機或猜謎遊戲的 Result/輸入 API。 |
| 2.6 | [Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) 全文 1–65；[Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) 全文 1–188 | [套件與 Crates](https://rust-lang.tw/book-tw/ch07-01-packages-and-crates.html) 全文 1–29；[模組](https://rust-lang.tw/book-tw/ch07-02-defining-modules-to-control-scope-and-privacy.html) 全文 1–124 | 取 crate root、module 名稱範圍、宣告與檔案對應、公開 module 不等於公開函式。保留計價／啟動分工與相同答案；不用 garden 的 struct/derive/Debug，也不以空餐廳函式代替本章可觀察工作。 |
| 2.6 | [Paths](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)：1–30、90–145、165–188、233–280；[use](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)：1–100；[拆檔](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html) 全文 1–125 | [路徑](https://rust-lang.tw/book-tw/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)：1–17、76–80、137–186；[use](https://rust-lang.tw/book-tw/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)：1–85；[拆檔](https://rust-lang.tw/book-tw/ch07-05-separating-modules-into-different-files.html) 全文 1–73 | `::`、同 crate 的 `crate::`、binary 從 library 名開始、`use` 只縮短本範圍路徑；`mod` 宣告 module，並非下載或逐次 include。保留 `name.rs` 與 `name/mod.rs` 對應規則作後續 helper 依據；本節不引入 glob/re-export/外部依賴。 |
| 2.7 | [Writing Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)：1–205、239–278、345–523、570–640 | [如何寫測試](https://rust-lang.tw/book-tw/ch11-01-writing-tests.html)：1–151、377–468 | 取已知規則→呼叫函式→assert→有意義修改→失敗解讀。只對 `i32`/`bool` 使用 assertion；不搬 Rectangle/reference/derive 或 Result-returning tests。`assert_eq!` 訊息是 left/right，將實際值放左、預期值放右只是本書約定，不能說 macro 的參數身分強制如此。 |
| 2.7 | [Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)：1–62 | [測試組織架構](https://rust-lang.tw/book-tw/ch11-03-test-organization.html)：1–28 | `#[test]` 與 test runner、`#[cfg(test)]` 的 configuration/test 條件、同檔 `mod tests {}` 先教再展示。使用逐項 `use crate::pricing::...;`，不默認 `super::*` 已知。本章可先只有 unit tests，不把「章末完整整合」混稱外部 integration-test crate。 |
| 2.7 | [Useful Development Tools](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html) 全文 1–137 | [實用開發工具](https://rust-lang.tw/book-tw/appendix-04-useful-development-tools.html) 全文 1–120 | 分清排版、型別診斷、lint、政策測試。新舊頁的 rustfmt/Clippy 安裝預設不同，依固定工具鏈安排。以既知數值語法讀訊息；不搬需要新標準函式庫路徑的 PI 例。 |

## 補充技術核對（不是讀者外部先備）

- [Rust Reference：operator expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html)，`reference-operators.txt:658–712,749–809`：六種比較語法；`&&`/`||` 兩側為 bool，右側僅在左側尚不能決定答案時求值。2.1 會用已教的函式、`println!`、數字比較，讓右側呼叫有無發生可見，不用 closure、panic、引用或除零作第一個說明。Python 的鏈式比較不能原樣移入 Rust；本章可用兩次比較與 `&&` 表達範圍。
- [Rust Reference：return](https://doc.rust-lang.org/reference/expressions/return-expr.html)，`reference-return.txt:19–29`：回到當前函式的呼叫者。正文只教操作邊界，不提前帶 move、activation frame 或 never type。
- [Rust Reference：loop expressions](https://doc.rust-lang.org/reference/expressions/loop-expr.html)，`reference-loops.txt:95–115,314–320,375–398,482–553`：while 重查條件、無標籤針對最近一層、label 指定外層、continue 回哪裡、loop 可交出值。不能把 `break 值` 宣稱適用 for/while；本章數字例要求各個會交出值的出口型別一致，但不概括否定後章的 coercion。label 前的單引號只是此處標籤語法，不借機講 lifetime annotation。
- [Rust Reference：diagnostic attributes](https://doc.rust-lang.org/reference/attributes/diagnostics.html)，`reference-diagnostics.txt:8–47,69–75,118–150`；[Clippy Usage](https://doc.rust-lang.org/clippy/usage.html)，`clippy-usage.txt:1–118`；[Clippy Configuration](https://doc.rust-lang.org/clippy/configuration.html)，`clippy-configuration.txt:38–90`：精確 lint 名可在函式／module 局部 allow。`allow` 只使所指定 lint 不報告，不能讓型別錯誤或錯誤政策變正確。本章只教精確 `#[allow(clippy::具體名稱)]` 的基本用途，不能一律 `allow(warnings)` 或帶入 `[lints]`/workspace inheritance。

## 舊材料實讀與具體處置

已全文讀舊 1.6 `01-06-conditions.md`、1.8 `01-08-loops.md`、1.9 `01-09-modules.md`、1.10 `01-10-tests.md`，及 coverage manifest 列出的 ch01-basics manifest/lib/main/pricing 與八份 examples。

1. 舊 `conditions.rs` 的第一個函式及 `main` 都有 if，所以不能以完整檔作新 2.1。需要新無 if 的完整比較／短路程式與答案；2.2 才可使用舊完整條件程式。
2. 舊 `loops.rs` 把 for、while、loop/break、continue 放同檔；2.3 和 2.4 要分開的新完整 target，2.5 才能收成全檔。舊教材沒有真正的 label 例，需要另規劃可觀察的巢狀控制材料。
3. 舊 library 的 `pub mod pricing;` 後接 cfg/test/assert。只以 snippet 隱藏 tests 不夠；2.6 的真實 package 必須有無 tests 的獨立 lib root。
4. 舊 `Cargo.toml` 透過 `.workspace = true` 繼承；新早期 member 應明寫 name/version/edition，不能沿用後再只展示簡化 manifest。
5. 舊練習多僅連 GitHub，2.1–2.8 要在同頁內嵌全部答案 source。library 題的完整版本包含實際編入的 manifest/lib/main/pricing 和測試，不能只有 lib 的局部 region。
6. 舊測試頁的邊界變異（>= 改 >）有明確理解收益：恰好 300 由 300 變 320。保留該觀察方式，將練習放在工作副本操作；準備階段未做本輪變異實驗；寫作階段的實際工作副本變異結果見 author-ch02.md。

來源對照本身不證明新教材已可理解或通過獨立審閱；實際編譯與輸出驗證、完整載入 source 及限制列於 author-ch02.md。
