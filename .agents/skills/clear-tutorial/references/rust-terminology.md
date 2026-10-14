# learn_rust：Rust 術語與語言指引

本文件供作者與技術審閱者使用，記錄本專案已與使用者對齊的用語。寫作與審閱仍依 [SKILL.md](../SKILL.md) 的教學準則；術語譯名或對照表不能代替正文的概念介紹。本文件不作首次閱讀者的先備解說或檢查答案。

## 抉擇原則

以正體中文解說技術，採用目標讀者熟悉的工程用語，讓讀者能對照 Rust 官方文件、程式與編譯器訊息。核心術語優先保留英文；變數、函式、型別、編譯器、記憶體等自然清楚的基本用語採中文。英文術語仍須在第一次需要時用中文教清楚它的意義、角色與當下用途。

同一概念的主要名稱保持一致。術語名稱與動作描述可以有不同寫法，例如 borrowing 是術語名稱，「借用這個值」是自然中文動作描述。

本專案的取捨參考侯捷〈科技翻譯面面觀〉的讀者導向原則：專業讀物可以部分原文術語為主要名稱，按需要補中文，並保持用語一貫。以下 Rust 用詞是本專案與使用者的約定，不是侯捷對 Rust 的逐詞規定。

## 已確認的用法

| 詞彙 | 本專案寫法 | 首次介紹與後續使用 |
| --- | --- | --- |
| stack、heap | 直接使用英文 | 用中文解釋當下涉及的記憶體概念，正文以 stack、heap 為主要名稱 |
| ownership、borrowing、move | 術語名稱保留英文 | 按需要補所有權、借用與值的移轉語意，後續仍以英文為主要名稱 |
| reference、pointer | 英文為主 | 首次補中文意思，如 reference（參照）、pointer（指標），再解釋它們的用途 |
| mutable、immutable | 直接使用英文 | 可寫「變數預設是 immutable」，不要求另加中文譯名；仍須解釋能否重新指派或修改什麼 |
| generics | 使用「泛型」 | 首次寫「泛型（generics）」，後續使用泛型 |
| trait、lifetime、closure、slice、crate | 核心術語保留英文 | 按需要補中文意思，並依教學順序介紹概念，後續統一名稱 |
| 變數、函式、型別、編譯器、記憶體 | 使用中文 | 不因核心術語採英文而將所有敘述改成英文 |
| Rust 關鍵字、API、型別與識別字 | 保留原文與大小寫 | 使用程式碼格式，例如 `match`、`impl`、`Vec<T>`、`Copy`、`Clone`、`Drop` |

首次介紹以實際閱讀路線為準：前文已完整教過的術語可沿用或引用，不必每頁重複加括號。獨立入口按其讀者背景補足必要意義。附上中文意思或英文名稱，不代表已完成概念教學。

## 動作與句法

借用、轉移、實作、修改、配置與釋放等動作，優先使用自然中文；需要點明 Rust 特定機制或操作時保留術語名稱，例如「此處發生 move」。

- 「函式透過 reference 借用這個值。」
- 「這次呼叫會取得該值的 ownership。」
- 「為這個型別實作 trait。」
- 「這個變數預設是 immutable。」

不要求把句子寫成「函式 borrow 這個值」或「implement 這個 trait」。一般英文術語使用正常文字；程式中的實際名稱、型別與語法使用程式碼格式，保留必要的大小寫區別。

## 名稱背後的概念區別

以下供作者與技術審閱者核對。正文在真正需要時教足必要意義，不因這份清單而提前插入所有進階概念。

- **lifetime 與 scope：**區分 reference 的有效範圍與名稱的作用範圍。不能把 lifetime 一概說成執行期經過的時間，或假定它永遠等於整個 lexical scope；lifetime annotation 也不會延長值的存活時間。
- **reference 與 pointer：**談 `&T`、`&mut T` 時使用 reference；談 `*const T`、`*mut T` 時使用 raw pointer，介紹共同性質時可用 pointer 作較廣的名稱。優先使用 shared reference、mutable reference 描述借用型態；`&T` 的限制有 interior mutability 的例外，不能概括成底層資料永遠不可修改。
- **move、`Copy` 與 `Clone`：**保留名稱與大小寫，按型別說明具體行為。move 不等於一定搬動 heap 資料；clone 的行為取決於型別，不能一概說成深層複製。
- **crate 與 package：**保留不同名稱，不將兩者一律寫成「套件」。需要時介紹 package 與其 crates 的關係。
- **術語名稱與比喻：**中文意思、比喻與既有語言的類比都須接回 Rust 的實際規則，不能用譯名或類比替代定義。

## 新詞與維護

其他詞彙先按抉擇原則、已建立的用語與官方名稱處理。method、binding、literal、pattern、tuple、variant 等尚未逐項確認的詞，不把先前討論中的建議當成使用者已選定的固定譯名。有多種合理寫法且會明顯影響全文風格時，集中少數關鍵詞向使用者對齊，再將結果更新於本文件；可依既有規則決定者直接沿用。

更新這份指引不代表已改寫既有教材。教材改寫與驗收依使用者指定的範圍及 skill 流程另行執行。同步上游 skill 時保留本文件及入口連結，並維護 `UPSTREAM.json` 的本地調整紀錄。

## 參考來源

- [侯捷〈科技翻譯面面觀〉公開轉載](https://groups.google.com/g/yizhe/c/-g3nVY4L47E)：讀者定位、專業讀物的原文術語與譯名原則。
- [侯捷〈英中繁簡編程術語對照〉備份](https://www.limoritakeu.tech/2017/11/20/translation-terms/)：譯名表作為參考與術語一貫性。
- [Rust Book：Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)、[Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)、[Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)。
- [Rust Reference：Pointer types](https://doc.rust-lang.org/reference/types/pointer.html)、[標準函式庫：Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)。
