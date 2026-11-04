# 1.7 外部來源實讀與採用紀錄

本次作者先親讀原始 cached 正文及圖，再作 1.7 教法；既有研究報告只是索引。網頁原始保存於 `/workspace/work/tutorial-reassessment-20261015/research-foundations/evidence/`，取得時間與原始 HTML SHA256 可回查同層 `sources.json`。本輪沒有重新抓取網頁，也沒有把舊研究者的 browser 操作當成本輪操作。

| 原始來源與本次實讀範圍 | 具體借鑑 | 不採用／限制 |
| --- | --- | --- |
| [Brown interactive Rust Book 4.1](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html)，`brown-en-ownership.txt` 1–138 全文；九份 `embeds.json` 程式；親看 `brown-first-embed-browser.png` | 68–74 行先以數值函式的L1／L2／L3觀察點對到frame，再給模型限定。1.7改用本書真實 `subtotal` A／B／C，逐狀態指認caller／callee與回傳結果。80行「A pointer is a value that describes a location in memory」支持位置值與內容分開解釋。 | 不搬UB／assembly／百萬array／Box／假想free／String，也不沿用「所有變數必在stack」或把近似清理規則當完整Rust契約。只親看第一嵌入的保存截圖，未操作任何互動題或其餘八個嵌入。 |
| [The Rust Book EN 4.1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)，`book-en-ownership.txt` 1–270；親看EN圖4-1／4-2 PNG | 36–47行定位資訊與內容的關係；160–174行把執行時請求、用完歸還及過早／重複／忘記歸還分開。圖4-1把定位資訊框與實際內容分開，支持1.7的H1示意。 | 不搬未教String／method語法；不作效能量測結論，不以固定大小推出實體placement或Copy。圖4-2屬論述中間狀態，不能單獨當Rust合法最終狀態。未親看4-3／4-4，也未讀本頁後半。 |
| [Rust Book 繁中4.1](https://rust-lang.tw/book-tw/ch04-01-what-is-ownership.html)，`book-tw-ownership.txt` 1–125 | 對照9行配置器、位置與內容，54–69行配置／歸還與scope的中文銜接；以本專案既定stack／heap／pointer用法重新組織解說。 | 繁中不是另一份學習效果證據；39行等固定大小必在stack的語氣不照用。未親看其獨立圖檔，圖比較採EN實際圖。 |
| [The Rust Book EN 15.1](https://doc.rust-lang.org/book/ch15-01-box.html)，`book-en-box.txt` 1–95 | 28–41行實際把 `i32` 放heap，是固定大小也能heap的原始反例；只用來校準1.7的文字限制。 | 1.7不展示 `Box` 程式或遞迴型別；不沿用Box沒有額外成本、handle必在stack的泛化；未讀頁面後半或看cons-list圖。 |
| [Rust Book 繁中15.1](https://rust-lang.tw/book-tw/ch15-01-box.html)，`book-tw-box.txt` 1–55 | 10–18行與EN同例對照，確認固定大小整數可heap的中文說法。 | 不提前教其語法，不把翻譯「堆疊／堆積」蓋過本專案術語；未讀後半。 |
| [Comprehensive Rust EN Review of Program Memory](https://google.github.io/comprehensive-rust/memory-management/review.html)，`.txt` 1–43全文；[繁中對應頁](https://google.github.io/comprehensive-rust/zh-TW/memory-management/review.html)，`.txt` 1–34全文 | 比較已有String先備後的複習式呈現，確認本節仍需自己建立值／bytes／位置與呼叫時序，不能以短列點代替。 | 本輪未親看該圖，沒有聲稱圖像實讀；不採unsafe／transmute、未教String或固定尺寸／placement總括。中文「本機變數」不沿用。 |

本次另外親讀原 `docs/chapters/01-05-functions.md`、`02-02-ownership.md`，核對舊節只有數值呼叫文字與四段記憶體模型，缺少可逐物件追蹤的frame／heap時序。新1.7用真實新前綴，而非把舊節當先備。

CPU／RAM／儲存裝置、bit／byte的基本橋樑是本節為基本Python讀者直接建立的內容，不聲稱上述Rust來源原本已如此教，也沒有主張此呈現具有已測得的學習效果。

作者研究階段與圖分鏡保存在 `/workspace/work/tutorial-implementation-20261015/foundations-author/`。所看過的外部圖只作教學關係研究；本節五張SVG由作者手寫，沒有複製外部圖檔。
