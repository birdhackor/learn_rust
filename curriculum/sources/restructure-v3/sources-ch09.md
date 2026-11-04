# 第9章外部來源與實讀證據

作者已親讀實際EN／繁體Book closure、Iterator與function pointer／returning closure章，並以Reference、Edition Guide與std API核對細節。21筆的完整URL、HTML與text SHA256、1-based inclusive read ranges、採用與不採理由，保存在/workspace/work/tutorial-implementation-20261015/ch09-preparation/source-read-ledger.json；原始HTML／body也在同目錄sources。長API只宣稱該ledger列出的實讀範圍，不把下載當閱讀。

| 原文 | 實際body閱讀範圍 | 取捨 |
| --- | --- | --- |
| [EN Book Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)／[繁體](https://rust-lang.tw/book-tw/ch13-01-closures.html) | 全文510／302行 | 保存工作再呼叫；body讀／改／移出與move capture分開。沿用運費材料，不提前thread／排序；不採sort_by_key恰每元素一次的概括。 |
| [EN Book Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)／[繁體](https://rust-lang.tw/book-tw/ch13-02-iterators.html) | 同日EN快取全文245；繁體全文153 | next推進、惰性adaptor與collect；把接手iterator和遍歷分開，不說map沒有消耗原iterator值。 |
| [EN Book Advanced Functions](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html)／[繁體](https://rust-lang.tw/book-tw/ch19-04-advanced-functions-and-closures.html) | 全文177／71 | fn型別、Fn能力與pointer轉換；closure有具體但通常不能具名的型別。不提前Box/dyn/Sized路線。繁體章號由實際TOC找出；兩次404嘗試保留在fetch紀錄。 |
| [FnOnce](https://doc.rust-lang.org/std/ops/trait.FnOnce.html)、[FnMut](https://doc.rust-lang.org/std/ops/trait.FnMut.html)、[Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html) | 各全文100／84／68 | 值／mutable／shared receiver與Output；能力包含關係。穩定bound寫法，不複製nightly rust-call內部API；Fn不等於純函式。 |
| [Reference impl Trait](https://doc.rust-lang.org/reference/types/impl-trait.html)／[Edition Guide RPIT capture](https://doc.rust-lang.org/edition-guide/rust-2024/rpit-lifetime-capture.html) | 全文134／222 | APIT/RPIT選擇方向、單一具體型別、2024預設capture與use清單；不提前TAIT/HRTB/migration trick，use不延長真實來源。 |
| [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)／[IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) | Iterator240–281、650–766、1360–1438、1689–1756；IntoIterator1–120 | Item/next、None後可恢復、for首None停止；map Item與filter &Item；IntoIter及equality。未選API不加入正文。 |
| [FromIterator](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)／[Result](https://doc.rust-lang.org/std/result/enum.Result.html) | 前者全文246；後者655–726 | 自訂目的型別與collect目標；首Err停止，E不套From。避免Vec配置策略與內部FromResidual旁支。 |
| [Entry](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)／[HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) | Entry全文244；HashMap398–451、610–632、1008–1095 | FnOnce的occupied/vacant工作、eager候選key與lazy值；三條ownership與任意順序。9.6不載sorting helpers。 |
| [str](https://doc.rust-lang.org/std/primitive.str.html) | 50–65、475–504、608–643、2484–2503 | scalar、LF/CRLF/loneCR、is_empty不trim；字典順序按bytes、不等於locale排序。 |
| [slice](https://doc.rust-lang.org/std/primitive.slice.html)、[Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)、[PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) | slice2106–2142、2177–2208、3408–3555；Ord1–203；PartialOrd1–186 | stable/unstable、回呼契約、NaN None非Equal、完整tie-break。sort_by_cached_key的至多一次不推給sort_by_key，不加入該API。 |

舊正文／36 runners／14 fixtures與expected／實際modules與manifest的全文實讀指紋在ch09-preparation/local-read-ledger.json。三SVG已讀XML並實際render親看；不使用外部圖。新版選材是作者原創的closure-transfer、iterator-next、adaptor-pipeline，正式圖文隨實際source核對。研究與前綴先備分開：source ledger不能代替新5／6／7／8的正文與程式交接。

正式9.5–9.7寫作前，root已交接實際8.5／8.6；作者另親讀這兩頁及必要source、6.5／6.6正文與必要map／Entry材料，範圍及SHA256追加在ch09-author/prefix-read-ledger.json。9.5的Result collect採原文相同E、首Err停止的契約，答案中的`?`回用實際8.6；9.6的owned key回用實際8.5，Entry使用本表原文的occupied／vacant條件；9.7同時提供count與name比較，未把stable曲解為可修正任意HashMap順序。未新增外部來源或把交叉引用提案改稱已教證據。
