# 第 3 章來源實讀與取捨

本章使用專案 clear-tutorial；來源是直接親讀的英文／繁中原文。讀取發生於本輪寫作前的作者準備階段，之後另親讀 root 交接的實際新1～2章及本章所需 source。下表「全文」只指保存的 main 正文與程式文字，不代表看過外部網站的圖或版面。Python 的讀取限定對應小節，沒有把整份 datamodel 說成讀完。

原始 HTML、抽出的正文與逐來源證據保存在 `/workspace/work/tutorial-implementation-20261015/ch03-05-preparation/sources/`。抓取日期皆為2026-10-15；每份 SHA-256 是原始 HTML 指紋，行範圍對應同 id 的 `.txt`。未執行外部範例，也未驗外部網站版面。本章 source 的編譯／執行證據另見 `author-ch03.md`。

| 來源 | 實讀範圍 | HTML SHA-256 |
| --- | --- | --- |
| [ownership-en](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) | `.txt` 1–447行 | `e3fc7f4e1db60f35be5bf71f915af51863b2ad0ccc5dd0e78debf4ce57e966a5` |
| [ownership-tw](https://rust-lang.tw/book-tw/ch04-01-what-is-ownership.html) | `.txt` 1–205行 | `39bdc6825fbbbbf4eaf9eeae56208c1afb68eca6c1fd4db34fc6cc619eab9e26` |
| [borrowing-en](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) | `.txt` 1–285行 | `379c38691d0eec3fed415151166f37705084acbac8ed98b67adf55990ab43552` |
| [borrowing-tw](https://rust-lang.tw/book-tw/ch04-02-references-and-borrowing.html) | `.txt` 1–201行 | `d9c8b878cb9677ba69893aa4015bbc06f4d933cff0d3698ca38f023d311620a3` |
| [slices-en](https://doc.rust-lang.org/book/ch04-03-slices.html) | `.txt` 1–346行 | `20faee1339c2cc9453c79e8002a9266d472124fc3b3f266f8bddb0254621aff0` |
| [slices-tw](https://rust-lang.tw/book-tw/ch04-03-slices.html) | `.txt` 1–242行 | `c400d605af0f7e307b9cd7720ab5482cebaabf3384e17a921a6f2b1fc8d95891` |
| [structs-en](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) | `.txt` 1–312行 | `7145fa8f3264e995894debc5af4df6d54d706a4b86c8faa67cb27ac69366730f` |
| [structs-tw](https://rust-lang.tw/book-tw/ch05-01-defining-structs.html) | `.txt` 1–214行 | `9715974b635652638659e19ea758dac6ba20f647c4e38af95750212b9f905f8e` |
| [methods-en](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) | `.txt` 1–307行 | `0c3ea308ee152d50044c3dae06f68b1759e211cc457303c3c89a7cf0249fbda3` |
| [methods-tw](https://rust-lang.tw/book-tw/ch05-03-method-syntax.html) | `.txt` 1–193行 | `1d4765470a61bef38526f3881685e50e38d68b7d673a94d86dd92328055a2551` |
| [strings-utf8-en](https://doc.rust-lang.org/book/ch08-02-strings.html) | `.txt` 1–363行 | `4502c2cd606c12c4ac2ba3fe5c229288473bf7d5869d5143b970e4d63616e92e` |
| [strings-utf8-tw](https://rust-lang.tw/book-tw/ch08-02-strings.html) | `.txt` 1–216行 | `16b0ad69268d299aa0c7c9970eda8160350c4fd6990945feddb91537754f0c3c` |
| [python-assignment-en](https://docs.python.org/3/reference/simple_stmts.html) | `.txt` 51–102、160–188行 | `ebe6a282a327e84d1e923ea1587026f9a84e5fe4230cfd02bfbf3efe81e6e4f2` |
| [python-assignment-tw](https://docs.python.org/zh-tw/3/reference/simple_stmts.html) | `.txt` 25–103行 | `33bb89000e894d0822c47bca8f4903de1f1ed4a6fbe9180f1760295ff12597f0` |
| [python-data-model-en](https://docs.python.org/3/reference/datamodel.html) | `.txt` 18–85行 | `15788284760e272adbdfada375360bfa129f9a97c638508cf0d68f0efe65ab78` |
| [python-data-model-tw](https://docs.python.org/zh-tw/3/reference/datamodel.html) | `.txt` 17–39行 | `646c6fabea137a4d1a0c1cff7bf199c0cc710ca17621cf82aff0b30b9df662d2` |
| [mdn-const-en](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/const) | `.txt` 1–107行 | `88b981b6bad9cbe64db967464740f749adc0f18f279574633f51cb832d52e10d` |
| [mdn-const-tw](https://developer.mozilla.org/zh-TW/docs/Web/JavaScript/Reference/Statements/const) | `.txt` 1–95行 | `e7def87988e3e7a31c569591dded5d8d3de1f9e6dfd91fe2f8d519756753eb21` |

## 採用、調整與延後

- Book ownership（EN／TW 4.1）：採用指派、參數、回傳的責任交接。3.2 使用只有 i32 欄位的 Order 與 i32 對照，不從來源的 String／clone 開始；不拿 stack／heap 分類當 Copy 規則。3.8 才接回文字資源清理。
- Book borrowing（EN／TW 4.2）：採用函式借讀後 owner 可續用、最後使用前後的限制。3.5 明分 scope 與 reference 必須有效的區間，不把來源的 reference scope 一律當成 lexical scope；另以真實 fixture 分開 reborrow 與 mutable reference move。不提前展開 thread、data race 或 lifetime annotation。
- Book slice（EN／TW 4.3）：採起點／長度與原資料對應。3.7 使用數值 array 的中段、整段、空段、不同長度；不搬 first_word 所需 String、as_bytes、iter、enumerate、reference pattern 或 borrowed output。&str 到3.8才出現。
- Book struct（EN／TW 5.1）：採 tuple 的位置角色改為命名欄位、定義→實例→修改。來源 User 的 String 不帶入3.1；pub struct／pub field 在自己的數值 module 完整教。struct update／tuple struct／unit struct留待有需要處。
- Book method（EN／TW 5.3）：採自由函式與 method 的同工作比較、三種 self receiver、associated function、Self 與 new 命名。省去來源的 Debug、幾何轉題與 C++ 指標語法；self 不被教成另一套 ownership 規則。
- Book strings（EN／TW 8.2）：3.8只用 new、push_str、as_str、literal／&str、owned String；中文 UTF-8 整段追加。編碼細節、字形／scalar／byte 與文字切片界線留在5.3。來源 to_string、From、+、format!、chars／bytes、Vec表示均不提前載入。
- Python 官方 assignment EN／TW §7.2及§7.2.1：採名稱重新綁定與 augmented assignment 依型別可能原地修改或產生另一物件。選讀以 list alias 和 str 重新綁定作直接對照，不概括成「全是 pass by reference」。
- Python 官方 datamodel EN／TW §3.1：採 immutable／mutable 是物件的性質、物件 type／identity 建立後不變。選讀沒有把 immutable str 與 Rust immutable binding 視為同一規則，也沒有引入深度凍結旁支。
- MDN const EN／TW全文：採 const 固定 binding，object 屬性仍可修改。明說 MDN 的 immutable reference 用語在此不是 Rust &T；比較分成重新指派、內容修改、借用權限三軸。沒有使用 Object.freeze 或 Rust Cell／RefCell。

舊 repo 的 ch02-ownership 正常例、答案與失敗例，以及 ch03-data 的 strings／strings_solution 和兩份 String 失敗例，均已親讀原檔後複製成自足的 intro-ownership targets，再加入 snippets region。新建 public_model、shared_answer、mutable_reference_move、text_order、mutability_comparison 與兩份外語完整程式，補足本章自己的教學閉包。沒有以舊 package library／test 已可編譯作為新前綴成立的理由。

## 尚未聲稱的證據

英文與繁中版本不保證同步；本章只依本次實讀內容作以上取捨。本文是作者來源與事實核對，不是獨立首讀或頁面驗收。實際 repo source、預期錯誤與輸出已在 pinned toolchain 下另驗；外部原文圖與外部程式均未執行、未做視覺驗證。
