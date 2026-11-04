# 15.8 B2 作者來源：先保留輸入，再依序檢查

本輪只寫15.8，依 clear-tutorial、writing-prompt 與 Rust 術語指引。A／B1 sources 不改；15.9 等 actual gate。

## 開寫前真正實讀

- ROOT actual `ch15-b1-prerequisite-v8` 全七 BODY 與 whole 兩 packages 九 source／manifest／README：單筆 owned Check、原文與 final URL、Input／Request／Timeout、單筆10秒工作期限與有限 test guards、typed error 與 repo 外 path dependency。不是拿自己尚未驗收的稿代替 gate。
- actual `ch06-complete-source-v6/007-002` 全文：lines 的 Item 為 &str、LF／CRLF／單獨CR／尾端換行規則，is_empty 不修剪。沒有把標題當 enumerate／trim 已教。
- actual `ch11-timing-prerequisite-v4/001-002`：Duration Copy 可重複交給每筆函式，沒有把同一 Duration 誤寫整批共用倒數。
- actual11.8 gate的 controls source與008正文：Fixture hits 是當下計數、next_started／release 是通知與一次gate、client和server分別await收尾。
- v2 B政策與15.8順序：空白行略過、原行索引與原文不改、重複仍各一筆、非空無效URL仍 item、零輸入fatal；先依序，沒有整批timeout／JoinSet／CLIJSON／取消。

## Fresh EN／ZH 教材

重新開英文／簡中 Book12.4 TDD頁，另下載並完整實讀兩份官方來源 narrative：英文 rust-lang/book main 共218行；中文 Kaisery/trpl-zh-cn master 共264行（該來源保留舊翻譯commit，沒有假稱與新版網頁逐字相同）。原始來源保存在 work-only `ch15-b2-author/en-book-tdd.md` 與 `zh-book-tdd.md`。

採用「從具體多行材料建立答案→起始版真失敗→先補行處理→保存結果→再次驗收」，讓 for/Vec 的工作可見。此處輸出要 owned，沿已教 to_owned；不搬 Book 的 borrowed Vec lifetime 作另一條主線、不搬 unimplemented!/super::*、Box<dyn Error> 或 CLI。英文 narrative 的 listing 引用不是已讀 listing source；本次另讀網頁已展開的例子，以及中文來源直接內嵌的逐行／push完整程式。

來源：https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html；https://kaisery.github.io/trpl-zh-cn/ch12-04-testing-the-librarys-functionality.html；完整 narrative 分別 https://raw.githubusercontent.com/rust-lang/book/main/src/ch12-04-testing-the-librarys-functionality.md 與 https://raw.githubusercontent.com/Kaisery/trpl-zh-cn/master/src/ch12-04-testing-the-librarys-functionality.md 。

## 固定1.99 selected標準契約

fresh下載1.99.0官方rustdoc，逐一實讀 lines、trim、Iterator::enumerate、Vec::is_empty 的簽名、描述與例子；selected text保存 work-only `str-1.99-selected.txt`、`iterator-1.99-selected.txt`、`vec-1.99-selected.txt`。第一次抽取程式因bs4未安裝而在任何下載前失敗，改用標準HTMLParser，不把失敗算閱讀。

- `lines` 借用原文字，無尾分隔符；尾端LF不多一行，空字串零行；單獨CR仍在內容。
- `enumerate` 取得iterator ownership，Item成為(usize索引,原Item)，從0開始。先enumerate才判空，略行不重新編號；本題是有限已讀文字，不把它泛化成無限計數器。
- `trim` 交回去除兩端Unicode White_Space的借用str，不修改來源。本題只拿trim後是否空字串判斷略行；保留與送入B1的是原line。非空行沒有新增token化／註解／URL去重政策。
- `Vec::is_empty` shared借讀，零元素true；prepare與runner都拒絕零項，全部非空無效URL不屬Empty。

網址：https://doc.rust-lang.org/1.99.0/std/primitive.str.html#method.lines、同頁#method.trim；https://doc.rust-lang.org/1.99.0/std/iter/trait.Iterator.html#method.enumerate；https://doc.rust-lang.org/1.99.0/std/vec/struct.Vec.html#method.is_empty 。
