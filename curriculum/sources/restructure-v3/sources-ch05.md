# 第5章來源實讀與取捨

下列10頁英文／繁中原文已在作者準備階段親讀，逐頁原始HTML、抽取正文、讀取行範圍與抓取時點保存於`/workspace/work/tutorial-implementation-20261015/ch03-05-preparation/sources/`。不是依outline猜測來源內容；Book EN/TW版本不保證同步。全文僅指抽取的main正文，不代表外部網站圖或版面已驗。原文程式未直接執行；repo快照另以固定工具鏈驗證。

| 來源 | 本次實讀的保存文字行 | 原始HTML SHA-256 |
| --- | --- | --- |
| [enums-option-en](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) | 1–363 | `dc6f3399ef446aca1ca2fa23a761fb5de024577a0b9b76324573737760ced1ba` |
| [enums-option-tw](https://rust-lang.tw/book-tw/ch06-01-defining-an-enum.html) | 1–211 | `de6266c390a15bf8ec6d8a6539bb1dd521d8a423733f2b6449f9bcf881b241bc` |
| [generics-en](https://doc.rust-lang.org/book/ch10-01-syntax.html) | 1–359 | `0a89df27da7b50bdfafaf4cf54aa1b01687eabf40288ef86d4b1104cf34b65ba` |
| [generics-tw](https://rust-lang.tw/book-tw/ch10-01-syntax.html) | 1–233 | `62f4f6eca8f7432d80a596d07e15bc3e4b23ad93377465e0c3132f95fb734256` |
| [result-en](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) | 1–80、184–258 | `3ee5ea6a5f947cf8c8da81c54a9945db85cc0f37f6fd97f274b0fcbbbfb946e2` |
| [result-tw](https://rust-lang.tw/book-tw/ch09-02-recoverable-errors-with-result.html) | 1–42、105–133 | `3fe21297511f624e151c13d32844a2d4668a7f7289165d6e4bdf5116d47bd56d` |
| [strings-utf8-en](https://doc.rust-lang.org/book/ch08-02-strings.html) | 1–363 | `4502c2cd606c12c4ac2ba3fe5c229288473bf7d5869d5143b970e4d63616e92e` |
| [strings-utf8-tw](https://rust-lang.tw/book-tw/ch08-02-strings.html) | 1–216 | `16b0ad69268d299aa0c7c9970eda8160350c4fd6990945feddb91537754f0c3c` |
| [vectors-en](https://doc.rust-lang.org/book/ch08-01-vectors.html) | 1–225 | `0b76931d37ee15ec06d2b67f6a8e0badf783ff6d16c0aa99a6ab0b9815d1f7a7` |
| [vectors-tw](https://rust-lang.tw/book-tw/ch08-01-vectors.html) | 1–125 | `6253e5a8efb4c0ec8640ca78bc6ce2eb777c5173260dec3345da7e37c297fec6` |


## 已採用與延後的材料

- generics EN/TW：採型別參數與值參數不同、同T兩欄須同型別、T/U可不同、impl<T>先宣告再使用。5.1新impl只保存／交回owned T，不搬largest的PartialOrd、不搬舊Extract的trait／associated type、不建立借用輸出。
- enums-option EN/TW：採Option<T>與T不同、Some/None完整形狀、None需要推導線索與prelude角色。先完成泛型才進標準enum，不以Option開5.1；以零與缺值保留原工作差別。
- result EN/TW：只讀Result定義、基本match與以match交回完整結果／錯誤的部分，範圍見表；沒有把整頁標成已讀。5.2以小整數除法保留成功值與具體錯誤，不搬File/io::Error、unwrap/expect、?或Box<dyn Error>。
- strings-utf8 EN/TW：5.3採byte/scalar/字形區別及range分界。沿用A中🦀材料；不用chars/bytes iterator、closure、map/collect、to_string/format!或Vec<u8>內部表示偷解當前工作。
- vectors EN/TW：5.4逐項交代new/push/len/pop/remove/as_slice/as_mut_slice/get與vec!，5.5再接String ownership與完整整理。push可能換存放區，不說成每次必定配置；borrow衝突按3.5使用區間判斷。舊library不拿來跑5.1～5.4。

舊ch03-data正常例、text/list、fixtures已親讀；本章獨立source與必要改動以`ch05-preparation/snapshot-plan.json`為寫作前安排，完成部分見`author-ch05.md`。本來源紀錄不是獨立首讀或頁面驗收。

## 本章完整版本的source落地

5.1新增inherent generic_impl與5.2 typed result_answer；其他短例、UTF-8 helper/tests、Vec操作與整理從實讀舊source保全後搬成獨立快照。usize assertion等價改成bool條件，沒有提前依賴6.2的整體比較／Debug；manual_map只在函式局部附理由。5.5先使用no-lib vector_ownership，再進完整text/list library，避免例子本身雖短卻編入後文。

A0105-006/007的實際新位置與引文列在author-ch05。root交接的新4章7頁與16份必要完整source另有指紋，不以這些外部Book頁代替現行前綴。正常範例、測試、fixtures已在repo pinned toolchain另驗；執行結果不回寫成外部原文曾被執行或版面已看過。
