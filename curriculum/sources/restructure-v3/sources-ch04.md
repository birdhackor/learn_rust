# 第4章來源實讀與取捨

本輪作者先親讀實際外部cached body，再核對新1–3正文與source，才寫第4章。沒有把前作者的研究摘要當作原文實讀；沒有重新抓取網站、執行外部例子或檢視外部圖，因此不作那些主張。

原始HTML與正文位於 `/workspace/work/tutorial-implementation-20261015/ch03-05-preparation/sources/`。本作者的16筆實讀ledger另存 `/workspace/work/tutorial-implementation-20261015/ch04-author/source-read-ledger.json`，逐筆列URL、原始取得時間、親讀行範圍、HTML及.txt SHA256；HTML指紋已與原始fetch索引核對相符。實讀內容與採用／不採理由的展開版本在同目錄 `research-and-snapshot-plan.md`。

| 原文與本次親讀範圍 | 借鑑的關係 | 本書不採用的部分 |
| --- | --- | --- |
| [Book EN 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) `.txt`1–250；[繁中6.1](https://rust-lang.tw/book-tw/ch06-01-defining-an-enum.html)1–165 | 一次一種形態仍屬同一型別；各variant可附帶不同形狀資料。 | 不搬IP背景；不把「用enum更直接」說成struct絕對無法表示；Option留第5章。兩頁後半未讀完。 |
| [EN 6.2](https://doc.rust-lang.org/book/ch06-02-match.html)全文1–332；[繁中6.2](https://rust-lang.tw/book-tw/ch06-02-match.html)全文1–216 | arm由上往下選第一個符合者、pattern binding、expression結果及exhaustive。 | 不用Debug／Option範例；以Quote::Amount(0)和Missing回應本書真實需要。 |
| [EN 19.1](https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html)全文1–217；[繁中18.1](https://rust-lang.tw/book-tw/ch18-01-all-the-places-for-patterns.html)全文1–131 | let／參數／for都能解構；值的形狀與pattern一格一格對應。 | 原for使用Vec／Iterator／enumerate，while let EN用thread channel、ZH用Vec.pop，全不提前。本書用固定tuple array與本地enum helper。原「不能if let &&」說法不沿用。 |
| [EN 19.2](https://doc.rust-lang.org/book/ch19-02-refutability.html)全文1–98；[繁中18.2](https://rust-lang.tw/book-tw/ch18-02-refutability.html)全文1–62 | 普通let／參數／for不能配對失敗，條件形式才有失敗路徑。 | 不用Option解釋；不在4.3用尚未教的let else修fixture。 |
| [EN 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)全文1–598；[繁中18.3](https://rust-lang.tw/book-tw/ch18-03-pattern-syntax.html)全文1–371 | struct欄位對應與改名、_／_name、..、or、range、guard、@逐步增加。 | 不把guard說成match免於涵蓋檢查；guard的pattern不能當成已覆蓋其所有值。無Option／Debug／clone／trait例子。 |
| [EN 6.3](https://doc.rust-lang.org/book/ch06-03-if-let.html)全文1–279；[繁中6.3](https://rust-lang.tw/book-tw/ch06-03-if-let.html)全文1–69 | 同任務比較單一成功分支與忽略其他情況；let else把成功binding留在後文。 | EN使用Option<String>／format!／州歷史，繁中尚無let else；改自訂enum。else必須return不是一般規則，以Reference核實break／continue也可。 |
| [Edition Guide：2024 match ergonomics](https://doc.rust-lang.org/edition-guide/rust-2024/match-ergonomics.html)全文1–95 | default binding mode已轉為ref／ref mut後，不能再任意加ref／ref mut／mut；& pattern只能在fully-explicit prefix。 | 不搬深層unit array表示；改單層enum與明確binding型別對照。 |
| [Rust Reference：Patterns](https://doc.rust-lang.org/reference/patterns.html)125–402、446–558、564–591、908–929 | Identifier預設依Copy或move；ref建立借用binding；& pattern匹配reference；partial move、_不move、or的binding一致、range邊界。 | 不把ref與&當成同義；不展開常數路徑、raw pointer、trait或複雜slice layout。 |
| [Reference：let statements](https://doc.rust-lang.org/reference/statements.html)全文1–128 | let-else失敗分支必須diverge，成功名稱在後續scope可用。 | 正文不先教never型別；直接追蹤return／break／continue離開哪個範圍。 |
| [Reference：loop expressions](https://doc.rust-lang.org/reference/expressions/loop-expr.html)1–175 | while let每次重算右式，符合就執行body，不符就結束；loop+match的對應。 | 不以Vec／channel／Iterator示範；更新和終止規則在helper首次使用前教清。 |

本次語言與術語按專案既定指引：enum／variant／pattern／match／guard以英文主名配中文解說；一般變數／函式／欄位仍用中文。沒有複製外部程式作為已教範圍的捷徑，也沒有把上述來源當作本次讀者學習效果的證據。
