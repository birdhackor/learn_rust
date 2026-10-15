# 逐頁語意引用清單

每筆保留原文與行號；`references.json`另有逐mention目標及核對狀態。數字版本、程式時間／token位置與純圖內方位已另存排除清單。

## docs/chapters/01-01-lesson.md

全文實讀1–90；SHA-256 `f67a72cfbbbd2557c856aa9efc5284a42b8eb8feb899ec2994949c12d03c6275`；引用位置11、mentions 11。

- L3（checked）`第一章` → 01-01、01-02、01-04、01-06、01-07：如果你已經能用 Python 寫變數、函式、條件與迴圈，就有足夠的背景開始這份教材。你不需要先學 C 或 C++。我們會先做一個小型數字運算程式，接著把重複的計算寫成函式，並認識執行時使用的記憶體。讀完第一章，你會知道一份 Rust 程式如何從文字變成能執行的工具。
- L5（checked）`第 2 章` → 02-07：Python 讓你很快把想法寫成程式；當程式開始管理較多資料、重複使用同一份資料，或同時執行幾項工作時，另一個問題會變得重要：哪些錯誤能在程式真正開始工作前被發現？Rust 把許多檢查放在**編譯**階段。編譯器閱讀原始碼、檢查它是否符合語言規則，再產生電腦能執行的程式。檢查通過不代表計算邏輯一定正確，因此還要比對實際輸出與預期答案；第 2 章再學如何把這些比對寫成測試。
- L7（checked）`後面` → ：後面的閱讀路線會從資料如何表示、由誰管理、哪些操作允許共用資料，走到檔案工具、同時執行的工作與完整實戰。現在先把編寫、檢查、執行這個循環建立起來；每一步需要的新概念都會先解釋，再放進程式。
- L17（checked）`後面` → 10-01：| `cargo` | 管理專案與建置工作：建立資料夾、呼叫編譯器、執行程式及測試。後面需要別人提供的程式庫時，也由它管理。 |
- L27（checked）`以下` → 01-01：先開啟終端機。以下指令從 Rust 官方網站下載安裝程式，再交給 `sh` 執行；`|` 把左邊下載到的內容送給右邊：
- L54（checked）`下列` → 01-01：到 [Rust 官方安裝頁](https://www.rust-lang.org/tools/install)下載並執行 `rustup-init.exe`。依安裝程式提示安裝 Visual Studio 的 C++ 建置工具；這提供 linker 與系統程式庫，不表示你必須先學 C++。完成後重新開啟 PowerShell，接著使用下列相同的版本檢查指令。
- L68（checked）`下一節` → 01-02：本教材的程式採用 Rust **edition 2024**，並以 Rust **1.99.0** 驗證。edition 是一組語言規則的版本，與工具的發布版本不同；下一節會在專案設定中看到它。教材 repo 會固定工具版本，讓範例的檢查結果容易重現。自行建立練習專案時，安裝近期 stable 即可；若要完整重現 repo，請保留 repo 的 `rust-toolchain.toml`。
- L74（checked）`下一節` → 01-02：先確認你能回答：安裝版本由哪個工具負責？真正檢查 Rust 原始碼的是誰？平常建立和執行專案會呼叫誰？答案依序是 `rustup`、`rustc`、`cargo`。下一節會用 Cargo 建立一個只印出一行文字的程式，親眼分辨「檢查通過」與「真的執行」。
- L81（checked）`如下` → 01-01：參考操作如下；實際的版本號依你所在的專案與安裝狀態而定：
- L88（checked）`上面` → 01-01：`rustc` 讀取並檢查 Rust 原始碼、產生編譯成果；Cargo 根據專案設定安排建置，再呼叫 `rustc`。若第一行找不到指令，先完成上面的安裝檢查，還沒到修改程式的階段。
- L90（checked）`下一節` → 01-02：[下一節：第一個可執行程式](01-02-lesson.md)

## docs/chapters/01-02-lesson.md

全文實讀1–87；SHA-256 `d9e129971cb9ec8234de1046a936c6f2be99b09396059c2c1a2ff81a38651c22`；引用位置10、mentions 11。

- L3（checked）`以下` → 01-02：這一節要讓終端機顯示 `Hello, Rust!`，並分清「檢查成功」和「程式已執行」。請先在教材 repo **外面**選一個練習資料夾，打開終端機。以下指令都在終端機輸入，不是 Rust 程式碼。
- L18（checked）`後面` → 17-01：要顯示一行固定文字，寫 `println!("文字");`。雙引號中直接寫出的文字叫**字串 literal**；literal 就是直接寫在原始碼裡的值。`println!` 將文字顯示在終端機並換行；`!` 表示這是 **macro**，一種在編譯時處理的特殊語法呼叫。現在只需依這個形式使用它，macro 的內部做法留到後面。行尾 `;` 結束這項顯示工作。
- L20（checked）`下面` → 01-02：把 `src/main.rs` 完整改為下面內容並保存。先看入口外框，再看它唯一的一項工作：
- L28（checked）`剛才` → 01-02：在剛才的 `intro-hello` 資料夾執行：
- L56（checked）`下面` → 01-02：現在打開 `Cargo.toml`。這份設定叫 **manifest**，使用 TOML（Tom's Obvious, Minimal Language）格式：`[package]` 開始一個設定區段，下面每行以 `欄位 = 值` 指定設定。
- L62（checked）`剛才` → 01-02：`name` 是 package 名稱，`version` 是你這個程式的版本，`0.1.0` 不是編譯器版本。`edition = "2024"` 選擇 Rust 2024 這組語言規則；edition 與工具的發布版本各有用途。本節的三個欄位已是完整可用的設定，不需要其他設定才能跑剛才的程式。
- L64（checked）`下一節` → 01-03：`cargo new` 可能另外產生空的 `[dependencies]` 區段。它用來列出需要另外取得的程式庫；這次沒有，因此留空或移除皆可。`println!` 已由 Rust 標準函式庫提供。Cargo 還會產生 `Cargo.lock`，由它記錄建置選用的 package 版本，下一節會說明怎麼使用。
- L70（checked）`如下、上面` → 01-02：完整參考答案如下，仍可放進同一個 `src/main.rs`，沿用上面的完整 manifest：
- L85（checked）`剛才` → 01-02：兩個 `println!` 各加一次換行。若看不到第二行，先確認有保存，再確認終端機位於剛才修改的 package。
- L87（checked）`下一節` → 01-03：[下一節：選對 Cargo 專案與範例](01-03-lesson.md)

## docs/chapters/01-03-lesson.md

全文實讀1–123；SHA-256 `03ae122568dbb6c36812815148adb19bd337481c37421f3c82f5163a97bdc5c1`；引用位置10、mentions 13。

- L11（checked）`以下` → 01-03：確認這裡同時有 `Cargo.toml`、`docs` 與 `examples`，這就是**教材 repo 的根目錄**。repo 是 repository 的簡稱，這裡指整份教材的檔案集合；若解壓時改了資料夾名稱，`cd` 後面也用你實際的名稱。以下路徑都從這裡開始。
- L15（checked）`上一節` → 01-02：上一節建立的 `intro-hello` 是一個 **package**：Cargo 以一份 `Cargo.toml` 管理它的名稱、版本與程式。**crate** 則是 Rust 編譯器一次處理的一個編譯單位。現在的 `src/main.rs` 是一個 binary crate 的起點，編譯後能執行；一個 package 也能包含供其他程式使用的 library crate，但本章先不建立它。
- L17（checked）`如下、上一節` → 01-03、01-02：教材把第一個 package 放在 `examples/intro-workspace/hello/`。其完整設定與入口如下，與上一節相同：
- L39（checked）`下面` → 01-03：下面是教材入門 workspace 的**完整設定**，檔案在 `examples/intro-workspace/Cargo.toml`：
- L47（checked）`下列` → 01-03：第二個成員 `greeting/` 使用下列完整 manifest 與程式：
- L67（checked）`上一節` → 01-02：`hello/examples/two_lines.rs` 保存上一節的第二個階段。Cargo 把 `examples/` 內的這種程式視為可選的**範例 target**；target 是你要求 Cargo 建置的特定成果。每個範例都有自己的 `main`，是獨立的 binary crate。
- L93（checked）`下一節、上面` → 01-04、01-03：下一節才選 `steps/examples/` 裡的數字範例。上面介紹的這幾個 package 都沒有 `src/lib.rs`，選擇一個範例時，也不會連帶編譯一份包含後續概念的 library。
- L105（checked）`上面` → 01-03：到這裡，每個選項都各有工作：路徑找 workspace、`-p` 選成員、`--example` 選範例、`--locked` 檢查版本鎖定。本書 repo 還有較大的維護用 workspace；本章指令都明確指向上面這一份入門 workspace。
- L109（checked）`如下、同頁` → 01-03：請選 `intro-greeting` 的 `two_lines` 範例，預期依序印出「你好，Rust！」與「開始今天的練習。」。這份完整程式如下，設定沿用同頁的 `greeting/Cargo.toml`：
- L123（checked）`下一節` → 01-04：[下一節：型別與數值運算](01-04-lesson.md)

## docs/chapters/01-04-lesson.md

全文實讀1–83；SHA-256 `cd61ce46e34365921fe5bca0f86918ab567f29638fe57a9db203c3ab653f04cc`；引用位置5、mentions 8。

- L3（checked）`以下、上一節` → 01-04、01-03：接下來要算一筆訂單：單價 120 元、數量 3，商品總額應是 360。先替材料命名，再讓程式算出答案。以下指令與後續本章指令都在教材 repo 根目錄執行，使用上一節的 `intro-steps` package。
- L35（checked）`第 1.7` → 01-07：有些材料只需要非負整數。`u8` 的範圍是 0 到 255，`u32` 的範圍是 0 到 4,294,967,295；`u` 表示不帶正負號，`i` 表示可帶正負號。名稱中的數字是表示空間的 bit 數，第 1.7 節會把 bit 和記憶體連起來。選較小型別不會讓超出的值自動擴大：例如 `u8` 不能保存 `256`。
- L37（checked）`後面` → 05-03、10-04：`bool` 只有 `true` 與 `false` 兩個值，適合保存「是否備妥」；`char` 保存一個 Unicode 純量值，用單引號，例如 `'甲'`。Unicode 是表示文字的通用編號系統；一個 `char` 不一定等於螢幕上一個完整符號，文字細節後面再談。雙引號的 `"甲"` 是字串 literal，與這個單引號值的型別不同。
- L51（checked）`下面、前面、同頁` → 01-04：下面是同一份完整原始碼，放在 `steps/examples/types.rs`；同頁前面的短片段都來自這個檔案：
- L83（checked）`下一節` → 01-05：[下一節：mutability 與 shadowing](01-05-lesson.md)

## docs/chapters/01-05-lesson.md

全文實讀1–78；SHA-256 `88627f3d2734717abff22fc4929ec6c8a24cfe33c7975a7d5b4a1cd60ec671c1`；引用位置5、mentions 6。

- L7（checked）`上一節、下面` → 01-04、01-05：上一節的 `let total: i32 = 360;` 是 **immutable** binding：初始值指定後，不能再經由這個 binding 重新指派。下面是完整的預期失敗程式，並非可執行答案：
- L25（checked）`以下` → 01-05：`mut` 允許重新指派，不會讓型別改變。以下完整程式刻意把 `3.5` 指派給已確定是 `i32` 的變數，所以得到型別不符的編譯錯誤：
- L41（checked）`剛才` → 01-05：第三個宣告又建立一個新 binding，這次型別是 `f64`，值為 `4.0`。這是換了一個 binding，所以可以選不同型別；與剛才試圖把浮點數指派給原有 `i32` 的失敗情況不同。三個宣告都沒有 `mut`，各自建立後仍不能重新指派。
- L76（checked）`前兩步` → 01-05：輸出依序是 180、170、340。前兩步重新指派同一個 `i32`；最後一步以 `let` 建立新的 immutable binding。
- L78（checked）`下一節` → 01-06：[下一節：函式、expression、block 與 scope](01-06-lesson.md)

## docs/chapters/01-06-lesson.md

全文實讀1–120；SHA-256 `53f814748fb6aa665fd39796da94a51becda827dbbfd083b66c8d40492babb1b`；引用位置7、mentions 7。

- L17（checked）`上面` → 01-06：函式的 `{ ... }` 是一個 **block**，也就是大括號圈起的一段工作。block 最後的不帶分號 expression 會成為它的值；函式本體的這個值就交回呼叫者。因此上面兩個參數決定這次乘法的材料，尾端決定交出的答案。
- L29（checked）`上面` → 01-06：Rust 用 **unit 型別 `()`** 表示沒有額外內容的結果，這個型別只有一個值，也寫作 `()`。函式省略 `-> ...` 時，回傳型別就是 `()`；上面的 block 沒有尾端值，因此結果是 unit。它仍是值與型別，不能拿來當 `i32` 相加。第一個程式中的 `main` 也是這種情況。
- L60（checked）`下面` → 01-06：下面完整的預期失敗程式，簽名承諾回傳 `i32`，本體卻得到 `()`：
- L88（checked）`下面` → 01-06：這兩個 block 組成下面的完整程式：
- L98（checked）`下面` → 01-06：輸出依序是含運總額 380、內層數量 4、外層數量 3。若想確認 scope 限制，下面是完整的預期失敗程式；最後一行無法找到 `item_total`：
- L104（checked）`下一節` → 01-07：這是編譯時對名稱範圍的檢查。下一節再觀察函式呼叫前、中、後的資料如何銜接，不能只從名稱能否使用，推定實際記憶體一定放在哪裡。
- L120（checked）`下一節` → 01-07：[下一節：程式執行時，電腦如何保存資料](01-07-lesson.md)

## docs/chapters/01-08-lesson.md

全文實讀1–111；SHA-256 `56108b46f7f6fb886ed76d048052239d0688dd8227a5ec7140292200d9a40584`；引用位置10、mentions 11。

- L11（checked）`下面` → 01-08：下面先保存整個 tuple，再以這兩種方式取得材料：
- L21（checked）`第 3 章` → 03-01：這裡的格子名稱由我們約定；`order.0` 本身不會告訴讀者「這是單價」，所以先解構成有意義的名稱會更容易讀。第 3 章會再學替資料欄位正式命名的做法。
- L27（checked）`前面、以下` → 01-02、01-06、01-08：型別寫成 `[i32; 3]`，表示「三個 `i32`」。這裡的分號分開元素型別與項數，與前面結束工作那種行尾分號的用途不同。選位置時，用 `prices[索引]`；索引是從 0 開始的號碼，所以三項的合法索引為 0、1、2。以下先選出第一項、第三項：
- L35（checked）`後面` → 03-06：`prices.len()` 是**方法呼叫**：在一個值後面用點接操作名稱，再加括號。這個方法回傳元素項數，本例是 3；空括號表示這次不必再提供其他輸入。現在先會使用 array 的 `len` 即可，後面才學如何為自己的資料定義方法。
- L47（checked）`第 5 章` → 05-04：`[i32; 3]` 與 `[i32; 4]` 是不同型別；同一個 array 不能靠多加一項變成四項。若將來要保存會增減的清單，需要另一種資料型別，第 5 章再教。
- L81（checked）`下面` → 01-08：下面是完整的預期執行失敗程式。`selected_price` 是我們在這裡定義的函式：收一個 `usize` 索引，回傳所選的 `i32`。`main` 刻意傳入 3，讓索引在函式執行時超出範圍：
- L87（checked）`第 1.2` → 01-02：若想重現，可以把這份完整內容放進第 1.2 節自建專案的 `src/main.rs`，保留當時的 manifest，再執行 `cargo run`。錯誤重點會包含：
- L93（checked）`第 1.6` → 01-06：程式不會印出 `價格=...`。與第 1.6 節的名稱找不到、回傳型別不符相比，這份程式的型別與名稱都合法，卻在執行時因這次索引失敗。編譯器若能直接判斷某次索引必定超界，也可能提早拒絕它；不能以「能編譯」推定每次索引都有效。
- L95（checked）`第 2 章` → 02-01、02-02：這個檢查避免程式把 array 範圍外的資料當成元素。現在先學會辨認邊界：長度 3，最後合法索引 2。第 2 章會用條件判斷，在選取之前決定應該怎麼做。
- L111（checked）`下一章` → 02-01：[下一章：流程控制](02-01-lesson.md)

## docs/chapters/06-01-lesson.md

全文實讀1–133；SHA-256 `5df389994db35de562be09374b10686ef2b11774eb70893f2200d4d159141d0a`；引用位置14、mentions 17。

- L3（checked）`第 3.6` → 03-06：第 3.6 節的訂單可以由單價與數量算出金額。現在還有一種組合商品，只保存整組價格。兩者的資料形狀不同，接下來卻都要回答「金額是否至少 300」。我們希望這項門檻規則保存在一處，而每種資料仍依自己的欄位取得金額。
- L5（checked）`第 5.1` → 05-01：本節沿用非負的小整數材料，乘法不超過 `i32` 範圍；不追加運費或缺少報價的政策。先讓兩種資料提供共同操作，再用第 5.1 節的泛型呼叫它。
- L23（checked）`第 3.6` → 03-06：`trait Charge { ... }` 宣告這項契約；`pub` 讓其他 module 可以使用。裡面的 `fn amount(&self) -> i32;` 是方法簽名，`&self` 依第 3.6 節代表 shared 借讀，實作於 `Order` 時借讀 `Order`、實作於 `Bundle` 時借讀 `Bundle`。現在不寫 body，而在簽名後加分號，留給各型別提供做法：
- L33（checked）`第 3.6` → 03-06：第 3.6 節的 `impl Order` 為型別定義自己的操作。現在的 **`impl Charge for Order`** 額外說明：這份實作遵守 `Charge`。方法簽名要符合 trait，body 則依 `Order` 的欄位相乘。`Bundle` 的實作以自己的 `price` 提供相同意義的結果：
- L43（checked）`第 5.1` → 05-01：第 5.1 節的泛型函式只保存、交回 `T`，不能假定每個 `T` 都能相乘或顯示。現在要在泛型 body 裡呼叫 `amount`，必須先寫出這項要求。
- L53（checked）`第 3 章` → 03-03、03-04、03-05：這裡的泛型 reference 型別 `&T`，仍依第 3 章的 borrowing 規則使用。兩次呼叫共用同一份門檻判斷，各自選用所傳型別的實作：
- L64（checked）`第 2.6` → 02-06：`mod charge_support;` 沿用第 2.6 節的規則，從同層的 `charge_support/mod.rs` 載入模組；該資料夾沒有 `main.rs`，不會成為另一個 Cargo 範例。
- L66（checked）`以下` → 06-01：透過點號呼叫 trait 方法時，提供該方法的 trait 也要在目前 scope 中。所以下面不只引入資料型別與函式，還有 `use crate::charge_support::Charge;`。這讓編譯器找到方法；它沒有取得資料 ownership，也沒有下載依賴。
- L89（checked）`下面` → 06-01：下面是完整的預期編譯失敗程式：保留 trait 定義，卻從函式的 `T` 移除 `: Charge`。
- L95（checked）`第 1.2` → 01-02：放進第 1.2 節的獨立專案執行 `cargo check`，會得到 **E0599**：泛型 `T` 沒有可用的 `amount`。錯誤發生在檢查泛型定義時，不必等某筆訂單執行才知道。bound 表達 body 實際需要的能力，不是替計算結果作擔保。
- L105（checked）`第 2.7、前面、下面` → 02-07、06-01：前面已列完整 `charges.rs`；下面是它實際載入的完整 `charge_support/mod.rs`。其中兩項測試沿用第 2.7 節的 `cfg(test)`、`test` 與 assertion，只比較數字與 bool：
- L121（checked）`下面、同頁` → 06-01：以原價 360、折扣 80，確認金額 280，300 門檻為 false、280 門檻為 true。下面是完整答案；它載入的是同頁完整的 `charge_support/mod.rs`，manifest 也沿用本頁：
- L131（checked）`下一節` → 06-02：輸出 `折扣後=280，達300=false`。答案也以原有 Order、Bundle 核對同樣金額，三種資料都使用 `Charge` 的操作。加入新的實作者之後，共同門檻函式保持原來的工作。下一節用標準 trait 讓整筆資料可以顯示與比較。
- L133（checked）`下一節` → 06-02：[下一節：Debug、PartialEq 與 derive](06-02-lesson.md)

## docs/chapters/06-02-lesson.md

全文實讀1–141；SHA-256 `f10544e1c8c5bba31e648e597ffa8fe05c8c48305bfaba692cdea1d8033be3f7`；引用位置16、mentions 16。

- L3（checked）`前面` → 03-01、03-06：前面顯示訂單時，總是分別選出金額、數量。若要一次檢查整筆摘要，或判斷兩份摘要是否相等，就需要型別提供相應能力。標準函式庫已有 `Debug` 與 `PartialEq`；這次先選定我們希望它們對欄位做什麼，再請編譯器產生實作。
- L9（checked）`第 2.7` → 02-07：對兩個整數欄位的摘要，我們希望一起看見型別名、欄位名與各自的值。寫在型別前面的 **`#[derive(Debug)]`**，就是要求編譯器替這個型別產生 `Debug` 實作。`#[...]` 延續第 2.7 節的 attribute 語法；`derive(...)` 列出這次希望產生的 trait。
- L11（checked）`下面` → 06-02：產生這份實作時，每個欄位也要有 `Debug`；`i32` 已有。因此下面可以逐欄顯示，再合成一份完整摘要：
- L27（checked）`下面` → 06-02：若移除 attribute，欄位仍各自能顯示，但整個型別沒有 `Debug`。下面是完整的預期編譯失敗例，會得到 **E0277**：
- L41（checked）`下面` → 06-02：一個 derive attribute 可以用逗號列出多個 trait。下面同時產生 `Debug` 與 `PartialEq`，因此 `assert_eq!` 可以比較整筆資料：前者供 assertion 失敗時顯示左右值，後者判斷是否相等。這些比較會借讀左右兩邊，不會把摘要 move 走，比較後仍可使用。
- L62（checked）`後面` → 06-02：接著要讓摘要帶有文字標籤。先把後面會用到的標準規則說清楚：`String` 已提供 `Debug` 與 `PartialEq`，比較的是文字內容；兩份獨立建立、內容同為 `Rust` 的 String 會相等。`&str` 也能使用這兩項能力，仍以所借文字的內容判斷相等。
- L64（checked）`下面` → 06-02：`Debug` 顯示文字時會包含引號，與原來 `{text}` 直接顯示內容的方式不同。下面用已教的 `String::new`、`push_str`、`as_str` 建立兩份文字及一份視圖，再比較它們：
- L74（checked）`前面` → 06-02：`usize` 和前面的整數一樣，已有 `Debug` 與 `PartialEq`。shared reference 在被借型別有相應能力時，也能以 `Debug` 顯示被借值、以 `PartialEq` 比較被借值。因此兩份分別借自不同變數、內容同為 2 的 `&usize`，比較仍相等：
- L84（checked）`第 5.2` → 05-02：第 5.2 節的 `Option<T>` 是 enum。標準函式庫在 `T` 有 `PartialEq` 時，也提供 `Option<T>` 的相等比較：兩個 `None` 相等；`Some` 與 `None` 不相等；兩個 `Some` 要看裡面的值是否相等。`T` 有 `Debug` 時，整份 Option 也能 Debug 顯示，例如 `Some(2)` 或 `None`。
- L86（checked）`下面` → 06-02：`Option<usize>` 因而能直接比較。`Option<&usize>` 也符合條件：成功時 reference 比較它借到的數值，缺值時仍是 None。下面的 `borrowed` 與另一份 `Some(other_ref)`，雖然 reference 來源不同，借到的值都為 2，所以相等：
- L92（checked）`後面` → 06-05：這讓後面查詢「有沒有取得值、取得的是不是 2」的測試能直接比較整份結果。它沒有把缺值換成零，也沒有略過缺值時的檢查。
- L96（checked）`下面` → 06-02：下面是以上三段的完整程式，沒有其他 helper：
- L114（checked）`如下` → 06-02：本節每個 example 都是獨立完整檔，不載入 library。共同的完整 member manifest 如下：
- L124（checked）`下一節` → 06-03：檢查原金額仍280、新金額300、兩份標籤文字相等，整筆摘要卻不相等。最後以 Debug 顯示兩筆。這一題還沒使用下一節的複製方法，兩份文字都由你明確建立。
- L139（checked）`下一節` → 06-03：標籤也有 Debug 與 PartialEq，才能讓整份摘要產生這兩項實作；改一個欄位後，整筆相等的結果就改變。下一節再保留原摘要，明確建立另一份可修改的值。
- L141（checked）`下一節` → 06-03：[下一節：Clone 與 Copy](06-03-lesson.md)

## docs/chapters/06-03-lesson.md

全文實讀1–98；SHA-256 `5a2de0ca59151643d07bfed212b014e36f275975cdbd9074d283d5aede346807`；引用位置9、mentions 11。

- L3（checked）`上一節` → 06-02：上一節已能比較兩份摘要，但兩份文字都是各自建立。現在想保留原摘要，再建立一份可修改的摘要。資料若含有 String，普通指定會 move；這次需要一項明確建立新值的操作。
- L9（checked）`上一節` → 06-02：對含 String 與 i32 的摘要，可以用 `#[derive(Clone)]` 產生實作。它逐欄呼叫該欄位的 Clone，再組成新摘要，因此每個欄位都必須提供 Clone。這裡的兩種欄位都符合；上一節的 Debug、PartialEq 也保留，供顯示與比較。
- L11（checked）`第 5.2` → 05-02：Clone 已在第 5.2 節介紹的 prelude 中，不必像自訂 Charge 額外 use。先看完整例子的前半部做什麼：建立 original、呼叫 `original.clone()` 成為 duplicate，再只向 duplicate 的 label 追加「 副本」。新舊標籤會分別是 Rust、Rust 副本。
- L19（checked）`第 3.5、下面` → 03-05、06-03：下面的完整程式先觀察 String 摘要的獨立修改，再建立 `Vec<&str>` 借讀 original 的標籤，複製這份清單。兩份清單都借到 original 的 Rust；原標籤必須在這些借用使用時保持有效，仍受第 3.5 節的限制：
- L39（checked）`第 3.2` → 03-02：再回到只有兩個 i32 欄位的摘要。第 3.2 節說過：整數已有 Copy，自訂 struct 不會因欄位都是整數就自動 Copy。
- L43（checked）`下面、剛才` → 06-03：產生 Copy 時，所有欄位都必須是 Copy；型別也必須提供 Clone。因此下面同時列出 `Clone, Copy`。反過來，有 Clone 不代表有 Copy，剛才帶 String 的摘要就是例子。
- L63（checked）`下面` → 06-03：下面是完整的預期編譯失敗程式，企圖為帶 String 的摘要產生 Copy：
- L96（checked）`下一節` → 06-04：這次只改副本金額，文字仍相等；原資料與新資料各自保留。下一節回到 trait 的定義，讓不同實作者可以交回不同型別的內容。
- L98（checked）`下一節` → 06-04：[下一節：associated type](06-04-lesson.md)

## docs/chapters/06-04-lesson.md

全文實讀1–146；SHA-256 `aa304cf004199a6d7fe5925854539614d1781f80072f40f77e3fff4dfea53cba`；引用位置8、mentions 10。

- L7（checked）`第 5.1` → 05-01：本節重新定義第 5.1 節形狀相同的泛型 Holder，value 欄位採用 T。SwitchState 則只保存 enabled 的 bool。型別與欄位公開，讓外面的呼叫者能建立它們：
- L33（checked）`第 5.1` → 05-01：Holder 的實作使用 **`impl<T> Extract for Holder<T>`**。前面的 `<T>` 宣告型別參數，接著用它表示要實作的 `Holder<T>`；這與第 5.1 節泛型 impl 先宣告 T 再使用相同，只是這次實作的是 trait。
- L45（checked）`第 6.1、上面、下面` → 06-01、06-04：範例根以 `mod associated_support;` 載入上面模組；方法呼叫也把 Extract 引入 scope，沿用第 6.1 節的規則。下面是 main 的第一段：
- L79（checked）`下面` → 06-04：下面完整的預期失敗程式，卻把 Output 為 bool 的 SwitchState 交給它：
- L107（checked）`第 5.3` → 05-03：其中三項測試核對數字／bool 輸出、乘二規則，與 String 交回後仍能追加使用。最後一項把 `Rust 教學` 的 byte 長度核對為11，沿用第 5.3 節的 UTF-8 長度規則。
- L128（checked）`下面` → 06-04：下面是完整答案，沿用本頁 manifest 與完整 associated_support；開頭的兩項檢查也確認原有來源仍符合契約：
- L144（checked）`下一節` → 06-05：現在同一項 trait 操作既能保留各來源自己的輸出型別，也能由特定函式進一步要求數字輸出。下一節讓標準 collection 用鍵來查值。
- L146（checked）`下一節` → 06-05：[下一節：HashMap 的建立與查詢](06-05-lesson.md)

## docs/chapters/06-05-lesson.md

全文實讀1–147；SHA-256 `2b15a18fcedcfe1d602caaddbb570b6e921f776a67ea932f49f965a7d3a947f5`；引用位置12、mentions 14。

- L5（checked）`後面` → 06-06：本節先用 `HashMap<String, usize>`：map 擁有名稱文字，對應值為非負整數。同一個相等鍵只保留一筆對應；後面才把這些操作連成掃描清單的分組函式。
- L9（checked）`第 6.2` → 06-02：第 6.2 節已用 PartialEq 比較內容。**`Eq`** 在相等比較上另外承諾每個值都與自己相等；它沒有新增方法。HashMap 用這種規則認出同一個鍵。String、str、i32、usize 的標準實作都符合本節所需的 Eq。
- L19（checked）`稍後` → 06-05：`insert(key, value)` 需要 mutable 借用 map，取得傳入鍵與值的 ownership，回傳 `Option<usize>`。若原本沒有相等鍵，新增後回傳 None；若原本已有，稍後會看到回傳舊值的情況。String 因而 move 進 map，usize 則可 Copy。
- L21（checked）`第 3.8` → 03-08：先準備一個只負責建立文字的小 helper。`owned_text` 全部使用第 3.8 節已教的操作；它把借來的文字拷進新 String，再交回：
- L27（checked）`下面` → 06-05：下面第一次加入 Rust→1；match 回傳的 None，顯示沒有舊次數：
- L71（checked）`第 6.2` → 06-02：這裡的 `assert_eq!` 使用第 6.2 節已教的 usize 能力。缺鍵並沒有因查詢而新增。
- L77（checked）`下面` → 06-05：下面完整的預期編譯失敗例，先取 found，中間插入 Python，最後才讀 found，會得到 **E0502**：
- L83（checked）`第 3.5` → 03-05：若工作是先顯示舊值再插入，把最後的 if let 讀取移到 insert 前。借讀的最後使用完成後，便能再 mutable 借用，沿用第 3.5 節的順序規則。
- L89（checked）`第 6.2、下面` → 06-02、06-05：下面完整程式先把 Rust 的2加成3，再觀察不存在的 JavaScript。`Some(&3)` 的比較依第 6.2 節：Some 的 reference 比較借到的數值。
- L108（checked）`下一節、同頁` → 06-06、06-05：這些 targets 只載入同頁的 map_support，其完整內容只有 owned_text。沒有載入下一節的分組函式。完整 member manifest 與查詢根檔案：
- L145（checked）`下一節` → 06-06：輸出 `Rust=Some(7)，Python=Some(5)`。取代值、借改值、缺鍵三種情況已各自可觀察；下一節才把它們連成完整的分組工作。
- L147（checked）`下一節` → 06-06：[下一節：分組、owned 鍵與自訂鍵](06-06-lesson.md)

## docs/chapters/06-06-lesson.md

全文實讀1–138；SHA-256 `c22dc2544ddca5f26d854f450028df0fc315ecd114d8db8b9a9ced81efbce549`；引用位置15、mentions 15。

- L3（checked）`上一節` → 06-05：這次輸入依序是 Rust、Python、Rust，要得到 Rust出現2次、Python出現1次。上一節的 insert 會取代值，不會自動累加。現在用 get_mut 分辨已有名稱與新名稱，讓同名標籤累積到同一筆結果。
- L7（checked）`第 3.7` → 03-07：函式只借讀 `&[String]`，交回新的 `HashMap<String, usize>`。沿用第 3.7 節走訪 borrowed slice 的方式，這次元素換成 String：`for label in labels` 每次取得一份 **`&String`**，沒有把 owned 文字移出輸入。
- L31（checked）`上一節` → 06-05：本節沿用上一節的 owned_text helper 建立輸入；其完整內容仍只有建立 String 的工作：
- L58（checked）`第 6.2` → 06-02：測試直接比較 get 的 `Option<&usize>`，已由第 6.2 節教過其 Debug 與 PartialEq：None與Some不同，Some中的reference比較借到的數字。這裡不需要用長度或假數字代替完整查詢結果。
- L60（checked）`第 1.6` → 01-06：最後一項測試特別把輸入放進內層 block，再把 count_labels 回傳的 owned map 交給外層 counts。這沿用第 1.6 節的尾端 expression；離開內層後，labels與其文字結束，map自己的鍵仍保留。
- L62（checked）`下面` → 06-06：下面是**完整** `grouping_support/mod.rs`，包括函式與全部三項測試：
- L72（checked）`上述` → 06-06：三項分別檢查同名累加、空清單沒有鍵，以及輸入scope結束後結果仍可使用。這個 target 載入的模組只有上述 map_support 與 grouping_support，不會隱含載入其他章的 library。
- L78（checked）`第 6.2` → 06-02：第 6.2 節的 derived PartialEq 逐欄比較；**derived Eq** 要求各欄位都有 Eq，才能保證每個組合與自己相等。**derived Hash** 則使用各欄位的 Hash 資訊，欄位也必須有 Hash。
- L82（checked）`下面` → 06-06：這次查詢借用完整的同型別鍵，傳入 `&same`。下面分別只改類別、只改地區，讓兩個欄位都參與鍵身分的效果可見：
- L98（checked）`下面` → 06-06：只有地區1、類別2有值3。若少了Hash，即使型別能相等比較，也不能完成HashMap所需的鍵查找。下面是完整預期編譯失敗例：
- L104（checked）`第6.1` → 06-01：編譯器以 **E0599** 指出insert的trait條件未滿足，其中缺少GroupKey的Hash。這和第6.1節缺少bound一樣，都是使用某項操作時沒有提供所需能力。
- L114（checked）`前面` → 06-06：前面已列grouping的根與兩個完整helper，以及自訂鍵的完整程式。接著將「每個標籤加一」改為「每筆庫存加上數量」。
- L120（checked）`下面` → 06-06：下面是完整答案，沿用本頁的manifest與完整map_support，不修改count_labels：
- L136（checked）`下一章` → 07-01：共同的查詢／缺鍵／更新結構保留，累加材料由固定的1換成本筆quantity。到這裡，trait能力、標準比較與複製契約都用在了完整資料整理工作；下一章再描述函式交回reference時的來源關係。
- L138（checked）`下一章` → 07-01：[下一章：lifetime annotation與來源](07-01-lesson.md)

## docs/chapters/10-01-lesson.md

全文實讀1–150；SHA-256 `ee3b7ee380d8110caf1e433c684190f6b12389bba3477e5b9b5a1fff21acd1ab`；引用位置16、mentions 20。

- L3（checked）`前面` → 02-06、02-07：前面的程式使用標準函式庫與自己寫的 library。接下來要讀檔案、處理命令列參數，也會使用別人維護的程式庫。這一節先回答：**Cargo 如何知道到哪裡找程式庫、需要哪些功能，以及使用哪一組版本？**
- L5（checked）`第 2.6` → 02-06：本節的 package 是 `intro-cargo-setup`，位於 `examples/intro-workspace/cargo-setup/`。它只有一個數字程式，會使用第 2.6 節已完成的計價 library；本節不改變運費規則。
- L21（checked）`接下來會` → 10-01：`std` 是工具鏈提供的標準函式庫，不必在 manifest 加入名為 `std` 的 dependency。接下來會設定的 `clap` 與 `tempfile` 則是第三方 package：
- L23（checked）`第 10.6` → 10-06：- `clap` 用來解讀命令列參數，第 10.6 節才會使用它的 Rust 介面。
- L24（checked）`第 10.8` → 10-08：- `tempfile` 用來準備測試中的暫存檔案與資料夾，第 10.8 節才會呼叫它。
- L30（checked）`後面` → 10-01：**feature** 是 library 提供的具名功能開關。`clap` 的 `derive` 功能支援後面要學的宣告式參數設定，所以本例明確請求 `features = ["derive"]`。這個功能名稱屬於 `clap`，不會替我們自己的 package 定義一個同名 feature。
- L36（checked）`以下、下面、第一章` → 10-01、01-02：以下是本節**完整** `cargo-setup/Cargo.toml`。前三個欄位仍是第一章已教的 package 名稱、版本與 edition。依賴有不只一項設定時，用 TOML 的**行內表格** `{ 欄位 = 值, 欄位 = 值 }` 放在同一組；下面的本機依賴與 `clap` 設定就是這種形式：
- L48（checked）`上述` → 10-01：若真的要在 manifest 寫精確要求，字串才改成 `"=4.6.7"`。本頁實際 manifest 保留相容版本要求，由 lockfile 保存這次真正選到的版本。0.x 的相容範圍有另一套界線，這裡不把上述 major 規則直接套過去。
- L50（checked）`下一節` → 10-02：[docs.rs](https://docs.rs/) 提供 library 的 **API（Application Programming Interface，應用程式介面）** 文件。API 是給其他程式使用的公開介面，例如公開函式、型別與方法。crates.io 回答「有哪些版本可取得」，docs.rs 回答「這個版本提供什麼公開功能、如何使用」。例如 [clap 4.6.7 的功能清單](https://docs.rs/crate/clap/4.6.7/features) 可核對預設功能與 `derive`；呼叫特定 API 前，還要閱讀它的輸入、輸出與使用條件，下一節會實作一份這樣的文件。
- L54（checked）`第 2.6、以下` → 02-06、10-01：在 Rust 程式中，package 的連字號換成 library crate 名稱的底線，這是第 2.6 節已用過的規則。以下完整 `cargo-setup/src/main.rs` 從 `intro_modules_basic::pricing` 呼叫公開函式：
- L81（checked）`下列` → 10-01：在**自己的練習副本**可以用下列指令重建本頁兩個 registry 設定。已有同名依賴時，它會更新現有項目，不會新增第二個 `clap`：
- L88（checked）`前面` → 10-01：這是刻意修改依賴設定的工作，不要求維持原 lockfile。完成後，再用前面的 `cargo check ... --locked` 確認 manifest 與解析紀錄相符。
- L102（checked）`下列` → 10-01：`cargo update` 則刻意重新選擇符合 manifest 要求的版本。下列指令只在練習副本嘗試更新 `clap`，仍可能連帶改動相關依賴：
- L114（checked）`第 2.6、以下` → 02-06、10-01：本頁真正使用的是第 2.6 節保留的最小 library，不會連帶載入本書後續章節。以下三份檔案就是這項本機依賴的設定與 library 來源；它的 binary 與其他範例沒有被本節指令選中。
- L140（checked）`以下` → 10-01：完整設定答案就是以下 manifest；保留完整欄位，方便逐項對照：
- L150（checked）`下一節` → 10-02：[下一節：閱讀 API 文件與測試公開介面](10-02-lesson.md)

## docs/chapters/10-02-lesson.md

全文實讀1–160；SHA-256 `394da63f7182a1c977beae9f11180ce370e1efe18851f4fbb0964a5dca4fb79a`；引用位置13、mentions 16。

- L3（checked）`第 2.7、上一節` → 02-07、10-01：第 2.7 節已把規則寫成測試。現在再問：**另一個 crate 能否依公開文件使用 library，文件中的例子是否真的正確？** 本節用 `intro-cargo-docs` 回答，資料夾是 `examples/intro-workspace/cargo-docs/`；上一節的依賴設定快照仍保留原樣。
- L9（checked）`第 2.7` → 02-07：**文件註解 `///`** 將文字附在下一個程式項目上。第 2.7 節的 `//` 是普通註解；多出的第三個 `/` 讓 Rust 的文件工具 **rustdoc** 能將這些文字整理為 API 文件。本例把它放在公開函式 `pair_total` 前面。
- L15（checked）`上一節、稍後` → 10-01、10-02：先看這份函式及其文件。它使用上一節已宣告的本機 library；完整檔案稍後也會列出引入 `total_with_shipping` 的 use。
- L37（checked）`剛才` → 10-02：若環境沒有圖形瀏覽器，可以開啟入門 workspace 預設輸出位置的 `examples/intro-workspace/target/doc/intro_cargo_docs/index.html`。若另設 Cargo 輸出位置，則到相應的 `doc/intro_cargo_docs/index.html`。找到 `pair_total`，應看到剛才的行為、限制與使用例。
- L51（checked）`如下` → 10-02：本節的檔案關係如下：
- L62（checked）`第 2.7` → 02-07：第 2.7 節的 library 同檔測試用 `crate::...` 從 library 自己的根找項目。這裡若在 `public_orders.rs` 寫 `crate::...`，起點則是**這份測試 crate**。因此完整 `tests/public_orders.rs` 使用 library 名稱：
- L70（checked）`第 2.7` → 02-07：`#[test]` 的意義仍與第 2.7 節相同。這裡不另包 `#[cfg(test)] mod tests`，因為 Cargo 已只在測試工作中建置這份測試 target。
- L82（checked）`第 2.7` → 02-07：library 內另外保留第 2.7 節已教的同檔測試形式：
- L100（checked）`如下、上面` → 10-02：本節 package 只有 library，沒有另一份會先載入命令列功能的 binary。完整 `cargo-docs/Cargo.toml` 與 `src/lib.rs` 如下；`tests/public_orders.rs` 已在上面完整列出：
- L110（checked）`第 2.6` → 02-06：唯一的本機依賴是第 2.6 節的計價 library，其完整設定與載入來源也保存在本頁：
- L152（checked）`以下` → 10-02：這份答案的本機依賴仍是本頁已完整列出的 `modules-basic`。執行以下指令，應有兩項 doc tests 通過：
- L158（checked）`下一節` → 10-03：現在文件不只告訴使用者如何呼叫，也以實際檢查保留選定的公開行為。下一節將把資料來源從程式內的數字改為檔案內容。
- L160（checked）`下一節` → 10-03：[下一節：檔案文字與具體 I/O 錯誤](10-03-lesson.md)

## docs/chapters/10-03-lesson.md

全文實讀1–180；SHA-256 `0ef86d599bc6a201968cededd1b0971904d3b0a957545558e6654170f626deca`；引用位置17、mentions 18。

- L3（checked）`第 8 章` → 08-05、08-06：第 8 章已能解析文字、保留具體錯誤，再由呼叫者決定是否繼續。現在文字來自檔案：我們要讀出一份筆記，保留換行與中文；檔案不存在，或內容不是 UTF-8 時，也要得到可處理的失敗。
- L13（checked）`第 10.5、下面` → 10-05、10-03：目前的工作目錄，就是程式解讀相對檔名的起點。下面的 `fixtures/note.txt` 因而指這個資料夾中的檔案，不是 Rust 原始碼檔案所在位置。第 10.5 節會進一步處理路徑型別與工作目錄的 API；此刻先固定起點，讓讀取對象明確。
- L19（checked）`下方` → 10-03：| `fixtures/note.txt` | 下方的兩行文字，最後也有換行 | 取得完整 UTF-8 文字，長度 9 bytes |
- L30（checked）`第 5.3` → 05-03：依第 5.3 節的 byte 規則，Rust 的四個 ASCII 字母加換行占 5 bytes，「中」加換行占 4 bytes，共 9。換行在 Rust 的字串 literal 中寫成 `\n`，所以完整內容可以表示為 `"Rust\n中\n"`。檔案沒有替我們分成 Rust 的字串清單；先讀回一份完整文字。
- L38（checked）`第 8.2` → 08-02：結果型別是 **`io::Result<String>`**，等同 `Result<String, io::Error>`。這延續第 8.2 節的 type alias：`io::Result` 替 I/O 工作固定錯誤型別為 **`io::Error`**，角括號只需要指定成功型別。它仍然是已教的 Ok／Err，沒有新的結果容器。
- L47（checked）`下面` → 10-03：下面的 caller 每次處理一個結果，再繼續下一個檔名。成功時以 `{:?}` 顯示借來的文字，讓換行以 `\n` 看見，而非在終端機直接換行：
- L65（checked）`上述` → 10-03：不存在與格式不合法都沒有成功文字可供讀取，但原因不同。Err 沒有自動停止程式；這個 caller 選擇逐筆報告。若檔案是合法的空文字，則得到 `Ok` 中的空 String，不能把它當成上述失敗。
- L69（checked）`剛才` → 10-03：剛才需要的是整份文字，所以一次呼叫就能完成。另一項需要是**把讀出的文字接在既有前綴之後**。先保留一個開啟的檔案，再指定文字要追加到哪裡，能把這兩份狀態分開。
- L73（checked）`第 6.1` → 06-01：**`std::io::Read`** 是讀取來源的 trait，提供讀取資料的操作。File 已實作它；沿用第 6.1 節的規則，先 `use std::io::Read;`，才能以點號呼叫其 methods。
- L101（checked）`第 8.3` → 08-03：**`std::io::Write`** 是寫入出口的 trait。File 已提供它；先引入 Write，再用第 8.3 節的 **`write!`** 格式化文字到這個出口。這次 `write!` 的結果是 **`io::Result<()>`**：Ok 表示這次寫入完成，Err 保存 I/O 原因。它不固定寫到終端機，也不自動加換行；換行來自傳入文字。
- L103（checked）`第 8.3` → 08-03：第 8.3 節的 `write!(f, ...)` 寫到 Formatter，回傳 fmt::Result；這裡寫到 File，回傳 io::Result。macro 的形式沿用，出口與失敗型別則由各自的能力決定。
- L117（checked）`以下` → 10-03：以下測試只讀教材提供的範例檔，沒有寫入共享檔案。Cargo 在執行 package 的測試時，以 package 根目錄作工作目錄，所以同樣的 `fixtures/...` 指到同一份材料。
- L119（checked）`第 6.2` → 06-02：ErrorKind 已有 Debug 與 PartialEq，能沿用第 6.2 節的 `assert_eq!` 比較類別。io::Error 本身不提供這項相等比較；因此先處理 Ok／Err，再比較 kind，不直接拿完整 I/O Result 做相等檢查。
- L146（checked）`前面` → 10-03：完整 `src/main.rs`，包含前面的讀取呼叫者與兩項測試：
- L158（checked）`前面` → 10-03：`write_note.rs` 已在前面完整列出。實際輸入材料只有開頭的三份檔案：`note.txt` 的完整文字已列出，`empty.txt` 是 **0 bytes、完全沒有內容**；`not-utf8.bin` 的完整 byte 序列只有 **255** 一項（十六進位 FF），沒有換行。這是原始 bytes，不能在文字編輯器輸入三個字元 `255` 來代替。教材已提供這個檔案，不需讀者先操作尚未介紹的 binary API。
- L176（checked）`下一節` → 10-04：目前我們把整份合法文字保存成 String。下一節改成固定大小的 byte buffer：讀取不一定一次填滿，也可能在一個 UTF-8 scalar 中間停下，要依回傳長度判斷哪些資料有效。
- L178（checked）`下一節` → 10-04：[下一節：bytes 與 buffer](10-04-lesson.md)

## docs/chapters/10-04-lesson.md

全文實讀1–206；SHA-256 `3fc73a9915ecedebe2589e981615aed0271a906fdfafe7bd8741367a20bf2171`；引用位置18、mentions 22。

- L3（checked）`前節` → 10-03：前節把完整 UTF-8 文字一次保存成 String。現在希望分批讀取檔案，或處理不一定是文字的資料。讀取來源每次交出多少資料，要由結果告訴我們；不能要求每次剛好填滿，也不能把 buffer 原本留下的內容混入新結果。
- L5（checked）`前節` → 10-03：本節使用 `intro-io-bytes` package。從 repo 根目錄執行 `cd examples/intro-workspace/io-bytes`，接著都在此處執行本節指令。前節材料位於旁邊的 io-text 資料夾，`../io-text/fixtures/note.txt` 的 `..` 表示先回上一層，再進入 io-text；起點仍是目前的工作目錄。
- L9（checked）`先前` → 10-04：**`u8`** 是 8-bit unsigned integer，也就是不保存負數、可表示 0 到 255 的整數型別。這份範圍剛好保存一個 byte 的值。它有 Copy、Debug 與數值比較能力，能沿用先前數字的讀取、顯示與檢查。
- L11（checked）`第 5.4` → 05-04：因此 `[u8; 4]` 是四個 byte 值的 array，`&[u8]` 借用一段 byte 資料，`&mut [u8]` 則允許修改借來的那段。`Vec<u8>` 擁有可增長的 byte 清單；使用第 5.4 節的 new／push／as_slice，就能保存分次收到的資料。
- L17（checked）`第 10.3` → 10-03：第 10.3 節已引入 Read trait。現在選用其 **`read(&mut buffer[..])`** method：mutable 借用來源與一段可填入的 byte 空間，回傳 **`io::Result<usize>`**。成功的 `n` 表示本次讀入多少 bytes，這些新資料位於 **`buffer[0..n]`**；符合契約的來源不會回傳大於 buffer 長度的 n。
- L19（checked）`以下` → 10-04：先用 ASCII 的 `ABCDE` 觀察長度，因為每個字母恰是一個 byte。假設來源第一次交出 ABC、第二次交出 DE；同一個四格 buffer 可以留下以下狀態：
- L33（checked）`第 6.1、下面` → 06-01、10-04：下面把流程放進 library 的 read_all。`R: Read` 沿用第 6.1 節的 trait bound；`reader: &mut R` 只借用來源來推進讀取，不接手它。
- L49（checked）`第 2.6` → 02-06：File 已實作 Read，所以相同 library 函式可以接收真實檔案。`Ok(mut file)` 的 pattern 先取出 File，再把接收它的 binding 宣告為 mutable，供 `&mut file` 使用。依第 2.6 節的 package／crate 規則，`intro-io-bytes` 的 library 路徑從 `intro_io_bytes` 開始：
- L59（checked）`前節` → 10-03：得到 `取得=9 bytes` 和 `完整文字="Rust\n中\n"`。如果換成前節的 not-utf8.bin，byte 讀取仍可成功，但文字轉換失敗。開啟、讀取與文字格式是三個不同步驟，各自保留失敗。
- L67（checked）`剛才` → 10-04：**`impl Read for ShortReader`** 提供 read 的具體做法，簽名遵循剛才的契約。每次從 copied=0 開始，直到 buffer 滿、達到本次上限，或原資料用完；每交一個 byte，就更新 copied 與 position。空 buffer 不進入 while，回傳零；原資料結束也回傳零。它不把任何舊 buffer 值算成新資料。
- L73（checked）`第 8.5、後面` → 08-05、10-04：為了建立原資料，再選用 **`str::as_bytes()`**：只借出該文字的 UTF-8 byte slice `&[u8]`，不複製或解析內容。沿用第 8.5 節的 ToOwned，這份 byte slice 的 `to_owned()` 建立 `Vec<u8>`，複製 byte 值供 ShortReader 擁有；只複製 reference 本身則沒有得到 owned 資料。ToOwned 也由 prelude 提供名稱，本例仍明寫 `use std::borrow::ToOwned;` 方便對照來源，後面可直接使用這個已教的 method。
- L101（checked）`第 10.3` → 10-03：第 10.3 節的 write! 負責把格式化文字寫出。現在處理 byte slice，選用 Write 的兩項 methods：
- L112（checked）`以下` → 10-04：以下 ShortWriter 的出口就是記憶體中的 bytes。write 每次最多接受兩個 bytes；accepts_data 為 false 時則不再接受，回傳零。flush 沒有中間緩衝需要推送，所以直接 Ok。write_all 是 Write 已提供的方法，會依這份實作的 write 反覆完成工作；我們不用再實作一份 write_all。
- L158（checked）`前面、下面` → 10-04：`src/lib.rs` 與 `src/main.rs` 已在前面完整列出；下面列出三份 example 的完整內容，也包含它們實際載入的測試。它們都只依賴本頁的 read_all library，沒有隱藏 helper：
- L180（checked）`如下` → 10-04：真實檔案來自相鄰 io-text 的三份教材材料。note.txt 完整內容如下，兩行末尾都有換行：
- L186（checked）`上面` → 10-04：empty.txt 是0 bytes；not-utf8.bin 的完整 byte 序列為單一255（十六進位FF），沒有換行。檔案已提供，不要以三個文字字元255取代原始byte。記憶體 ShortReader/ShortWriter 的資料、事件與預期答案則完整保存在上面的Rust來源中。
- L192（checked）`第 10.3、前節` → 10-03：驗證前節三個已存在的檔案：note 是 9、empty 是 0、not-utf8 是 1。這次只數 bytes，不要求資料是 UTF-8，因此最後一份也能成功；和第 10.3 節的文字讀取需求不同。
- L204（checked）`下一節` → 10-05：[下一節：原生路徑與 CLI](10-05-lesson.md)

## docs/chapters/10-05-lesson.md

全文實讀1–218；SHA-256 `4e6a04dc914b90e51355003dc6bfc0f7734dfa2f3a3ff3e003733be2d1fe55e8`；引用位置19、mentions 31。

- L3（checked）`前兩節` → 10-03、10-04：前兩節使用固定檔名讀取筆記。現在要讓呼叫程式的人指定檔案，成功就交出原文字，失敗則回報原因與結束狀態。這份需要還包含一項區別：**檔名是作業系統接受的路徑，檔案內容才是這個工具要求的 UTF-8 文字**。
- L5（checked）`第 9.4、第 10.3、10.4、以下` → 09-04、10-03、10-04、10-05：本節使用 `intro-path-cli` package；從 repo 根目錄執行 `cd examples/intro-workspace/path-cli`，以下指令都在這個資料夾執行。先把路徑與原生字串讀清楚，再接上 第 9.4 節已教的 Iterator 與 第 10.3、10.4 節的 I/O。
- L26（checked）`第 8.3` → 08-03：顯示時選用 **`path.display()`**：它交回供格式化使用的 `std::path::Display` 物件，實作 第 8.3 節的 Display trait；它借用原路徑，**不是產生一份 String**。若路徑不能完整表示成 Unicode，這項顯示可能有失真，所以供人閱讀，不拿顯示文字重新建立檔名或作唯一識別。
- L57（checked）`下面` → 10-05：into_string 的 Err 中是**原生資料本身**，不是新增的一段錯誤訊息；caller 仍可以保存它或轉成 PathBuf。下面先借用再交成路徑，最後另用一份固定 OsString 觀察 owned 文字轉換：
- L69（checked）`第 10.3、第 8.5` → 10-03、08-05：第 10.3 節的 fs::read_to_string 不只接受文字檔名，也接受 **`&Path`**。它的輸入需要能借成 Path，這是 第 8.5 節的 `AsRef<Path>` 關係；本節明確傳 `filename.as_path()`，不靠把原生路徑先轉成 String。
- L81（checked）`第 9.4` → 09-04：**`env::args_os()`** 建立標準型別 `ArgsOs` 的 iterator，其 **Item 是 owned OsString**。第 9.4 節的 next 每次取得 Some 中的一筆，結束是 None；不必先收集成 Vec，再冒著索引越界去取參數。
- L85（checked）`第 6.2、8.3` → 06-02、08-03：先在獨立 `arguments` example 以具體 enum 保存兩種用法失敗；Debug／PartialEq／Display 都沿用第 6.2、8.3 節：
- L103（checked）`第 8.1、以下` → 08-01、10-05：PathBuf 與 ArgumentError 都已提供 PartialEq／Debug，所以 Result 的比較符合 第 8.1 節已教的條件。以下測試保存合法中文檔名、少參數與多參數三個結果：
- L111（checked）`如下、接下來的` → 10-05：`examples/arguments.rs` 的完整程式如下。main只用固定參數取得路徑，測試則保留三種規則；它沒有先載入接下來的標準出口或結束狀態程式：
- L123（checked）`第 10.3、10.4` → 10-03、10-04：`io::stdout()` 交回 Stdout，`io::stderr()` 交回 Stderr。它們是操作對應標準出口的物件，都提供 第 10.3、10.4 節的 Write，可以以 mutable binding 呼叫 write!／write_all 並處理 io::Error。它們沒有替程式選擇該輸出哪些內容。
- L125（checked）`第 10.4、下面` → 10-04、10-05：Stdout 可能先把資料留在行程共用的 buffer；連到終端機時通常以換行為推送邊界。這次要保留原筆記，不能假定最後一定有換行。因此 write_all 成功後，還要呼叫 第 10.4 節已教的 flush，檢查緩衝資料能否交給標準出口，再選成功狀態。正常結束時的自動清理不能替 main 回報這一步的錯誤；flush 成功也沒有承諾永久儲存。Stderr 預設沒有這份緩衝，下面仍直接檢查每次診斷寫入的結果。
- L141（checked）`第 8.3、8.5` → 08-03、08-05：接著讓主要 binary 的 main 取得 arguments、呼叫相同的 parse_filename，再用 &Path 讀取。format／to_string 都沿用第 8.3、8.5 節，Path 的 display 只進診斷；操作真正使用的是原生 Path。先 write_all 原文字的 bytes，再 flush；兩步都成功才回傳成功狀態，沒有加標題或額外換行：
- L157（checked）`以下` → 10-05：以下是 Bash 中的觀察方式：**`>`** 把 stdout 存到指定檔案，**`2>`** 把 stderr 存到另一個檔案，既有輸出檔會被清空；本次只使用兩個專用 captured 檔案。`echo $?` 顯示上一個指令的結束狀態，要立即放在 run 後面。
- L170（checked）`前面` → 10-05：本節 package **沒有 library**。前面的 path_types、os_strings、working_directory 與 arguments examples 都是獨立crate，只編譯各自完整程式；最早的路徑例子不會連帶載入參數與輸出處理。
- L178（checked）`如下` → 10-05：主要 binary 的完整 `src/main.rs` 如下，保存相同的 ArgumentError、parse_filename與測試，再接上標準出口與main。沒有隱藏helper或未列出的module：
- L186（checked）`前面、如下` → 10-05：前面三份路徑／字串example已各自全檔列出。真實檔案操作沿用相鄰io-text的材料：note.txt完整文字如下，兩行最後都有換行；empty.txt是0 bytes；not-utf8.bin只有原始byte255（十六進位FF），沒有換行。missing.txt刻意不存在。
- L198（checked）`上一段、先前` → 10-05：這與先前example測的是相同規則；實際stdout、stderr與狀態仍要依上一段分別核對。
- L214（checked）`下一節` → 10-06：我們已經能完整處理一個原生檔名與 CLI 輸出。下一節加入第三方參數解析 library，處理選項與說明；仍先看它接受什麼、交回什麼，以及何時真的是用法錯誤。
- L216（checked）`下一節` → 10-06：[下一節：clap 與 owned 設定](10-06-lesson.md)

## docs/chapters/10-06-lesson.md

全文實讀1–130；SHA-256 `1a66bbad509d726af4d6c6b8697c854ce1878fba9dd23b3311f22d471130a99c`；引用位置17、mentions 21。

- L3（checked）`第 10.5` → 10-05：第 10.5 節的工具只接受一個檔名，手寫 next 已足以完成它。接下來要為文字搜尋準備兩個必要資料：要找的文字、要讀的檔名；呼叫者還可以要求忽略大小寫，並查詢用法與版本。
- L5（checked）`第 10.9` → 10-09：這些規則可以繼續手寫，但需要自己維護位置、開關與說明之間的對應。**clap** 是 command-line argument parser（命令列參數解析 library），這次讓它依 struct 欄位建立規則、產生說明，再交回 owned 設定。搜尋行為留到 第 10.9 節；本節只驗收參數如何成為設定，以及顯示請求如何交給呼叫者。
- L7（checked）`以下` → 10-06：本節使用 `intro-clap-stage` package。從 repo 根目錄執行 `cd examples/intro-workspace/clap-stage`，以下指令在此執行。
- L11（checked）`下面` → 10-06：先讀呼叫方式。下面是預期交給程式的三筆參數，程式名稱之外還有兩項必要資料與一個開關：
- L23（checked）`第 10.5` → 10-05：query 是文字，必須能成為 UTF-8 String；filename 保存 PathBuf，可保留原生檔名。clap 解析 filename 不會替我們開檔案或保證檔案存在，仍保留 第 10.5 節「路徑表示與真正 I/O 分開」的關係。
- L29（checked）`第 10.1、以下` → 10-01、10-06：沿用 第 10.1 節的依賴與 feature 契約，選用 clap 的 derive feature。版本要求仍是範圍，實際解析版本由 Cargo.lock 保存。以下是本節完整manifest，name/version/edition仍明寫，不繼承後續的工程設定：
- L35（checked）`第 6.2` → 06-02：**`clap::Parser`** 是將命令列資料解析成設定型別的 trait。**`#[derive(Parser)]`** 依欄位和 attributes 生成相關實作；第 6.2 節已教的 derive 仍是編譯時工作，並不是啟動程式時先執行某個 Parser 函式。
- L54（checked）`第 10.5` → 10-05：input 必須是 `IntoIterator<Item = T>`，元素 T 要有 `Into<OsString> + Clone`：每筆能交成原生字串，並能依 Clone 的契約建立副本。第 10.5 節已教 OsString 的 Clone；它也能以 Into 交回相同型別，因此 owned OsString array 與 args_os 都符合。這裡的第一筆仍是程式名稱，**把完整序列交給 clap，不自行先 next 略過**。
- L67（checked）`下面` → 10-06：下面先在 library 內測試。兩個成功情況分別核對開關省略與啟用；失敗保存用法判別；顯示請求則驗收正常結果。測試不依說明文字的整份排版，也不直接比較 clap::Error：
- L77（checked）`第 10.5、上一節` → 10-05：現在將 args_os 的完整序列交給相同方法，成功就以 第 10.5 節的 Stdout／Write 顯示三個設定。這份程式是設定檢查工具，所以輸出欄位標示；不是已完成的文字搜尋結果。show_settings寫完三行後，再交回flush的結果，沿用上一節「寫入與推送都成功才回報完成」的政策。
- L81（checked）`第 10.5` → 10-05：有些範例使用 `Parser::parse` 或 `error.exit()` 直接結束行程。本節選 try_parse_from 加正常返回的 ExitCode，讓 library 可測試、呼叫邊界清楚，也保留 第 10.5 節已教的 scope 清理。
- L100（checked）`第 10.5` → 10-05：help 的 stdout 列出兩個必要位置參數與已宣告開關，version 顯示 query-settings 0.1.0；兩者結束狀態為 0。沿用 第 10.5 節的分流保存與緊接 `echo $?`，少一個位置參數或改用 `--unknown` 時，可觀察 stdout 空白、stderr 有用法訊息、狀態為 2。
- L102（checked）`第 10.8` → 10-08：這裡解析成 Settings、回傳顯示請求／用法錯誤、真正寫出訊息，是三件不同的工作。讓它們分開，純參數測試就能先保存規則，main 再處理行程介面；第 10.8 節會測真正的輸出與狀態。
- L106（checked）`如下` → 10-06：本節library只包含已教的Settings與四項解析測試，完整 `src/lib.rs` 如下：
- L112（checked）`前面、上面、下一題` → 10-06：manifest與完整main已在前面列出。執行上面的`--lib`只驗證library；接上main後的CLI指令才編譯並執行真正出口。下一題example也會建置這份library，但沒有提前載入JSON或搜尋模組。
- L126（checked）`下一節` → 10-07：我們已把輸入規則接成可獨立持有的設定，並區分資訊顯示與真正的用法失敗。下一節從檔案內容建立另一種資料模型：將 JSON 文字讀成 owned Rust 資料。
- L128（checked）`下一節` → 10-07：[下一節：JSON 與 Serde](10-07-lesson.md)

## docs/chapters/10-07-lesson.md

全文實讀1–240；SHA-256 `4199ad8b74b917794bfa2cdea3fc92c95a7ecf582b3d606393fb463f6a93526e`；引用位置22、mentions 29。

- L3（checked）`第 10.3` → 10-03：第 10.3 節已能讀到完整 UTF-8 文字。現在筆記不只是一段內容，還要保存標題、數值清單與是否封存，讓下一次程式啟動能讀回這些角色。
- L17（checked）`下面` → 10-07：JSON 的 **object** 以大括號保存名稱與值，名稱要以雙引號包住，冒號後是值，逗號分隔各項。**array** 以方括號保存依順序排列的值。下面的對應就是我們希望讀回的資料：
- L36（checked）`上述` → 10-07：**`serde_json`** 則是選定 JSON 格式的實作，使用模型提供的上述能力，將文字與模型接起來。Serde 的能力並不限於 JSON；在本節使用它們，是為了讓相同欄位規則有明確的 JSON 編碼與讀回方式。
- L38（checked）`第 10.1、下面` → 10-01、10-07：沿用第 10.1 節的 dependencies／版本要求／features，serde 的 **derive feature** 提供生成這兩項 trait 實作的能力。下面是本節完整manifest；版本仍是範圍要求，實際選定版本由入門 workspace 共用的 Cargo.lock 保存：
- L44（checked）`第 6.2` → 06-02：`#[derive(Serialize, Deserialize)]` 的安排和第 6.2 節相同：讓編譯時的生成工作依欄位建立實作。Serde 已替 String、`Vec<i32>`、bool 提供必要能力，所以這個模型的全部欄位都能參與。
- L54（checked）`第 7.3` → 07-03：這是 **owned model**：欄位中沒有 reference。標題與清單屬於 Notebook，不需要像第 7.3 節的 TextView 一樣，繼續依賴原輸入 String 的存活。
- L65（checked）`第 8.5` → 08-05：這裡的 from_str 是 serde_json 提供的泛型函式，依 Deserialize 能力解碼；模型不需要另實作 第 8.5 節的 FromStr。
- L67（checked）`第 7.3` → 07-03：from_str 對結果型別 T 的要求是 `Deserialize<'a>`，其中 `'a` 代表借入的 JSON 文字來源。這是 trait 使用 lifetime 參數的形式，延續第 7.3 節在型別契約中表達來源的規則。**它不要求每個結果都保存輸入 reference**；我們生成的是欄位全為 owned 的 Notebook，所以成功後可以讓原文字結束。
- L71（checked）`第2.6、以下、後面` → 02-06、10-07：本節package沒有library。`basic_json` example以 `pub mod json_core;` 載入以下完整root；裡面只宣告model與codec，沒有先載入後面的檔案錯誤封裝。pub沿用第2.6節的module公開規則；這些名稱仍屬於各自的example crate，不會因此替package建立library：
- L77（checked）`第2.6` → 02-06：json_core的codec module沿用第2.6節的公開函式分工，把這個模型的編解碼放在一起：
- L83（checked）`接下來的` → 10-07：使用 String 作輸入時，先以 as_str 明確借出 &str。接下來的程式讀取教材檔，解析、檢查角色，再借用模型產生 JSON；開檔失敗和 JSON 失敗各在自己的步驟處理：
- L106（checked）`第 10.9` → 10-09：values 的排版改成多行，但順序與值相同。這個例子用標題訊息協助核對，不是對外只交出 JSON 的完整 CLI；第 10.9 節會把程式邊界接起來。
- L116（checked）`以下` → 10-07：所有教材JSON內容都在本頁，包含開頭的notebook.json，以及以下用來驗證格式、欄位與跳脫的完整材料。missing.json刻意不存在；not-utf8.bin的完整原始byte序列只有255（十六進位FF），沒有換行。
- L158（checked）`第 8.5、先前` → 08-05、10-07：先前兩個步驟各自用 match。現在把「讀到 Notebook」整理成 helper：可能遇到 io::Error，也可能遇到 serde_json::Error。這是第 8.5 節的封裝場景，使用具體 **NotebookError**，兩個 variants 保存原錯誤。
- L160（checked）`第 8.3、後面` → 08-03、10-07：這份定義放在獨立的json_files module，只有本段的file_errors與後面的答案才會載入。它只 derive Debug，不比較完整錯誤。Display 借用其中的原因顯示，依第 8.3 節已教的 `&T: Display` 契約；From 實作則把各種原錯誤交進正確 variant：
- L166（checked）`第 8.6` → 08-06：read_notebook 的兩個 `?` 沿用第 8.6 節：讀檔失敗就以 From 封裝 Io，文字解析失敗則封裝 Json。第一個步驟沒拿到文字時，不會執行第二個：
- L182（checked）`以下` → 10-07：以下第一項測試把輸入 String 放在內層，原文字結束後仍使用 Notebook，再編碼與讀回，核對全部欄位。第二項使用真實的 UTF-8 教材檔，分別確認語法、缺欄位、未知名稱、錯型別與數字超出範圍都會被拒絕。第三項確認換行與雙引號的資料可以往返，沒有手寫一套跳脫規則：
- L204（checked）`前面、上面` → 10-07：前面的model、json_core root、basic_json、file_errors都已完整列出。完整codec包含上面的兩個函式與三項測試；完整json_files包含錯誤、From、讀取函式與一項測試：
- L220（checked）`第2.6、下面` → 02-06、10-07：三個examples的不同只在載入哪些模組：basic_json只宣告json_core，file_errors與下面的答案宣告json_core、json_files。資料夾內沒有main.rs的helper不會自行變成example target，沿用第2.6節的Cargo檔名規則。
- L236（checked）`下一節` → 10-08：現在文字格式、模型能力與具體錯誤已串起來。下一節要建立專用測試資料，啟動真正的 CLI，驗收它和呼叫者之間的完整介面。
- L238（checked）`下一節` → 10-08：[下一節：fixture 與真正的 CLI](10-08-lesson.md)
- L240（checked）`前文` → 03-08、05-04：延伸閱讀：[Serde：Using derive](https://serde.rs/derive.html)、[serde_json：from_str](https://docs.rs/serde_json/1.0.154/serde_json/fn.from_str.html)、[Rust Cookbook：Structured Data](https://rust-lang-nursery.github.io/rust-cookbook/encoding/complex.html)、[中文 Rust Cookbook：JSON原文](https://github.com/rust-lang-cn/rust-cookbook-cn/blob/master/src/encoding/complex/json.md)。後兩者還展示通用資料表示，本節先固定 owned 模型與前文已教的String／Vec。

## docs/chapters/10-08-lesson.md

全文實讀1–160；SHA-256 `301e423c4f1e2cd6541442c1bb47c0bf839b335031c6a9dec281ca57c1a4c2e7`；引用位置19、mentions 24。

- L3（checked）`前面` → 10-02、10-05、10-06：前面先測 library，再以命令列觀察輸出。現在要把「指定一份筆記，得到原內容；失敗有診斷與狀態」保存為可重複執行的測試，讓日後調整 main 也能發現出口或狀態被改壞。
- L11（checked）`第 10.5、第10.5` → 10-05：先沿用第 10.5 節的單檔筆記工具，這樣新增工作集中在測試工具。本package保存第10.5節已完成的ArgumentError、parse_filename與main完整快照，使用同樣的讀取與分流行為；不需要另外載入library。測試啟動的是**本 package 的 binary**，不是拿另一個 package 的執行檔路徑猜測。
- L20（checked）`第10.1` → 10-01：tempfile 是建立臨時檔案／目錄的 library。它只在測試中使用，放在已教的 dev-dependencies；版本要求與lock的分工仍和第10.1節相同。完整manifest只有基本package欄位與這項開發依賴：
- L26（checked）`第 10.5` → 10-05：這份受測 main 沿用第 10.5 節的輸出完成條件：write_all 後還要檢查 flush，再選成功狀態。沒有新的 Rust API：
- L34（checked）`後面` → 10-08：先選用 **`tempfile::tempdir()`**。它在系統的臨時位置建立新目錄，名字由 library 產生，回傳 `io::Result<TempDir>`；建立失敗就是 io::Error。**TempDir** 是管理這份目錄的 owner，**`directory.path()`** 只借出 `&Path`，供 join、讀寫與後面的子行程使用。
- L36（checked）`第11.3、上述` → 11-03、10-08：TempDir 正常離開 scope 時會嘗試刪除目錄及其中的內容，但自動清理遇到的錯誤會被忽略。如果需要知道清理是否成功，使用 **`directory.close()`**：它接手 TempDir，移除目錄，回傳 `io::Result<()>`。close 之後原 owner 已 move，不能再借它的 path；這是此型別的使用契約，這裡已可依上述行為操作；第11.3節再說明實作資源收尾的Drop trait。
- L38（checked）`第 10.5` → 10-05：不要只留下 PathBuf 就讓 TempDir 先結束。PathBuf 擁有的是路徑資料，**沒有接管目錄資源**；依第 10.5 節的區分，保存路徑不會讓檔案持續存在。測試將 owner 留到讀寫／子行程結束，才清理。
- L44（checked）`稍後` → 10-08：| `fs::write(filename.as_path(), text)` | 接收 `&Path` 與 `&str`，回傳 io::Result<()>；不存在就建立，存在就完整取代內容。也接受 `&[u8]`，稍後用來建立非法 UTF-8。文字會以其 UTF-8 bytes 寫入；資料符合 `AsRef<[u8]>` 的借用能力。 |
- L50（checked）`第 10.2` → 10-02：第一項 integration test 放在 tests/directory.rs，形式沿用第 10.2 節。先檢查材料，再以 close 確認清理，保存的 PathBuf 最後仍能表示原位置，但該位置已不存在：
- L96（checked）`第 10.4` → 10-04：捕捉到的bytes要先檢查UTF-8；這次選用**`std::str::from_utf8(bytes)`** 接收 `&[u8]`，檢查 UTF-8 後回傳 `Result<&str, Utf8Error>`，錯誤有 Debug／Display。這次只是借出 Output bytes 的文字 view，和第 10.4 節接手 Vec 的 String::from_utf8 不同；Output 仍保存兩份原 bytes，讓我們分項檢查。
- L112（checked）`下面` → 10-08：圖中箭頭是**操作先後**；外框是 TempDir 仍由測試持有的範圍。先讓 child 用完資料，才 close；下面留下的 PathBuf 只是位置表示，不能延長資源的存在：
- L118（checked）`下面` → 10-08：下面分別檢查不存在的檔案、非法 UTF-8、缺少 argument，還有**不存在的執行檔**。最後一項在自有目錄指定刻意沒建立的程式位置，應得到外層 NotFound；沒有 Output 或 child status 可供檢查。Output 也提供 Debug，若意外得到 Ok，測試可以在 panic! 中列出它。不依權限、特權帳號或外部服務碰巧失敗：
- L124（checked）`前面` → 10-08：先用已教的 `--test` 選cli target。前面的directory與這裡的cli各自就是一份integration test crate：
- L130（checked）`下面` → 10-08：cli 的四項主文測試，加上下面空檔練習的答案共五項；directory 有一項，共六項。CLI 每次都是真正啟動 binary，沒有拿 library 回傳值當成已驗收 stdout／stderr／status。本節沒有為main另加同檔測試；這不影響兩個已選integration targets的六項檢查。
- L134（checked）`前面、以下、下一題` → 10-08：前面已完整列出manifest、受測src/main.rs與tests/directory.rs。以下是完整tests/cli.rs，包含helper、四項主文測試與下一題空檔測試的完整答案：
- L140（checked）`上面` → 10-08：所有fixture都由上面的測試程式建立：成功檔案完整文字為`"Rust\n中\n"`；非法UTF-8完整byte序列為`[255]`；空檔完整文字為`""`。不存在的檔案與執行檔都刻意不建立。沒有額外下載、共享檔案或GitHub上的隱藏答案。
- L152（checked）`同頁` → 10-08：    本題沿用同頁完整manifest、main與tests/cli.rs的helper。`--test cli`後再接測試名稱，會只執行名稱符合的測試，其餘顯示filtered out；整份target仍會先編譯。
- L156（checked）`下一節、前面` → 10-09、10-05、10-06、10-07、10-08：我們已能準備專用材料、保留資源 owner、等待真正的 child，並分別驗收資料、診斷與狀態。下一節把前面可分開測試的參數、文字工作、JSON 與這份行程邊界組成完整 CLI。
- L158（checked）`下一節` → 10-09：[下一節：完整筆記搜尋 CLI](10-09-lesson.md)

## docs/chapters/10-09-lesson.md

全文實讀1–249；SHA-256 `65e47b59a1715e2928ed498294e693de554e246be011eeb00f191d674d295c55`；引用位置31、mentions 41。

- L3（checked）`前面` → 10-03、10-05、10-06、10-07、10-08：前面已分別完成原生檔名、參數設定、文字讀取、JSON 與真正的 child 測試。現在把它們接成可使用的文字搜尋工具：呼叫者指定要找的文字與檔案，工具交出符合的整行及原行號，也能改用 JSON 報告。
- L7（checked）`後面` → 10-09：本節使用 `intro-note-search` package。從 repo 根目錄執行 `cd examples/intro-workspace/note-search`，後面的 Cargo 指令以此為入口。
- L11（checked）`如下` → 10-09：`fixtures/note.txt` 有四行，內容如下：
- L29（checked）`第9.6` → 09-06：lines 沿用第9.6節：LF／CRLF 分行、不包含行尾、最後的行尾不增加一行。行號從 1 開始，**空行也占原行號**；結果只是沒有符合的空行，不重新編號已保留的行。文字模式在每筆後加入 LF，並不重建原檔案的行尾格式。
- L31（checked）`第10.4` → 10-04：行程結果也先固定：正常搜尋（包含零筆）、help／version 是 0；檔案／編碼／輸出失敗是 1；缺必要參數、未知開關或空 query 是 2。資料只進 stdout，診斷只進 stderr。讀取或用法失敗時尚未寫出資料；若寫 stdout 本身失敗，則可能已寫出部分 bytes，保留第10.4節的 Write 失敗邊界。
- L35（checked）`第10.1、10.6、10.8、以下` → 10-01、10-06、10-08、10-09：沿用第10.1、10.6–10.8節的依賴設定，所有版本要求與features都已教過。以下是本節完整manifest，package欄位明寫，實際版本仍由入門workspace共享Cargo.lock保存：
- L41（checked）`後面` → 10-09：Settings 保留兩個 positional 與 ignore_case，再增加本節的 json long bool flag。這兩個開關都只保存選擇，執行工作交給後面的 helper：
- L49（checked）`第9.5` → 09-05：Serde 已替 usize 提供編解碼能力，在 JSON 中表示整數；能讀回的數字仍受目前平台 usize 範圍限制。Serde 的 `Vec<T>` 在元素 T 有相應能力時也能編解碼，因此 Match 的 derive 先建立能力，Report 的清單才能使用它。Debug／PartialEq 同樣逐欄作用，Vec 比較內容的規則已由第9.5節教過：
- L57（checked）`第8.3、10.7` → 08-03、10-07：Display 沿用第8.3、10.7節的 borrowed 原因格式化。自訂 method **`status(&self) -> u8`** 只依 variant 交回我們剛定的 1 或 2，最後由 main 轉成 ExitCode；它不會自行終止行程：
- L63（checked）`第2.6、如下、下面` → 02-06、10-09：檔案分工如下。lib.rs以第2.6節的公開module宣告納入各部分；它另外提供read_report、write_report與run，將各層連接起來。下面會先逐項說明工作，再列出完整library root：
- L89（checked）`上述` → 10-09：忽略大小寫時，把 query 與每行都轉成上述比較表示，再 contains。例如 RUST 與 Rust 都變成 rust；一行 Trust me. 轉成 trust me. 也包含 rust，因為規則查的是片段。**只轉 query 而不轉行內容，兩邊仍可能不同。**
- L93（checked）`第8.6、稍後` → 08-06、10-09：validate_query 保存非空規則。search 與稍後的檔案入口都呼叫它，避免 library 的不同入口接受空 query。第8.6節已教同型別的 From 會原樣交回錯誤，因此這些 AppError 之間的 `?` 不需要另寫轉換。
- L95（checked）`第9.6` → 09-06：每次 enumerate 交出 index 與一行，index + 1 就是從 1 開始的原行號；只有符合時 push Match。index 對全部原行增加，沒有只數已保留的行。這個來源本來有行序，結果沿該順序保存，不用把 第9.6節的HashMap 任意順序問題帶進來：
- L101（checked）`下面` → 10-09：先核對純工作，不讀檔也不寫stdout。下面測試保存預設與ASCII選擇的差別、非ASCII邊界、空行／CRLF行號、原輸入結束後資料仍有效，以及空query的失敗。
- L103（checked）`稍後` → 10-09：測試中的Rust字串literal用 `\"` 表示內容的一個雙引號，`\n`仍表示已教的LF；例如 `"Rust \"筆記\"\n"` 產生的內容是Rust、空白、帶雙引號的筆記與LF。這是Rust source的表示；稍後Serde會另將這份資料編成JSON表示。
- L109（checked）`前面` → 10-09：這五項的預期來自前面的材料與規則，不由程式輸出反推答案。本節先保存測試，等完整library各module都說明後再執行；名稱篩選只決定執行哪些測試，同一份library的其餘module仍會編譯。
- L113（checked）`第10.7` → 10-07：render shared 借用 Report。文字模式依原序產生 `行號:原內容`，用已教的 format!／push_str 拼成 String；JSON 模式用第10.7節的 to_string_pretty，最後也加一個換行。
- L125（checked）`第10.4` → 10-04：write_report 先 render，再用第10.4節的 write_all 交給 `W: Write` 的出口。它既能接 main 的 Stdout，也能接已教的可控制 writer；不自行寫診斷。本函式完成把bytes交給writer，沒有替它flush；若writer有buffer，由仍持有它的caller在需要確認送出時處理flush。run 將 read_report 與 write_report 接起來，沿用同型別 AppError 的 `?`：
- L135（checked）`第10.6` → 10-06：沿用第10.6節的 try_parse_from：help／version 的 Err 是正常顯示請求；其餘用法訊息進 stderr、狀態 2。取得 Settings 後才呼叫 run。AppError 的診斷透過已教的 ToString／writeln! 交到 stderr，status 再由 ExitCode::from 建立數字結果。
- L137（checked）`第10.5` → 10-05：main持有Stdout，沿第10.5節在run成功後明確flush；這一步也成功，才回正常狀態。flush失敗用已有的AppError::Output保存io原因，再顯示診斷並回作業失敗1。library交付writer與真正CLI確認出口完成，各自有明確責任。
- L151（checked）`上面、以下` → 10-09：上面已全檔列出config.rs、model.rs、error.rs與main.rs。以下保存完整lib.rs，以及含同檔測試的search.rs、render.rs；最早的Cargo命令放在這些能力都已解釋之後，會編譯這份已完成的library。
- L173（checked）`前面、前幾節` → 10-09、10-01、10-02、10-03、10-04、10-05、10-06、10-07、10-08：完整manifest與唯一固定fixture note.txt已列在本頁前面。這份package會永久保留完整CLI，前幾節的最小快照也各自保留。
- L175（checked）`第10.8` → 10-08：現在可以先選library內的search測試，觀察五項通過；第10.8節已教名稱篩選，這裡仍會編譯全部library module：
- L189（checked）`第10.5` → 10-05：前兩次輸出對應開頭的行號規則；第三次 stdout 只有 JSON，matches 依序保存第 1 與第 4 行。沿用第10.5節的 `>`／`2>` 與緊接 `echo $?`，可以分別保存資料／診斷，觀察成功 0、缺檔 1、缺少 positional 2；`--help`／`--version` 仍是 0。
- L195（checked）`第10.4、下面` → 10-04、10-09：writer.rs 沿用第10.4節的自訂 Write 形式：RefusingWriter 的 called 記錄是否真正寫入；write 刻意回傳 ErrorKind 的 PermissionDenied（存取權限被拒絕）所建立的錯誤，沒有成功接收本次 bytes。這份可控制的 writer 不依賴真實檔案權限。它沒有中間 buffer，flush 直接 Ok。下面測試使用非空結果，保證 write_all 有工作，而不是拿空輸出誤當已測到 write：
- L201（checked）`第10.8` → 10-08：CLI tests 沿用第10.8節的 TempDir、Command、Output 與借用 UTF-8 view。execute 把測試準備的 OsString array 借為 slice，逐筆 arg；**不包含程式名稱**，因為啟動 child 時 OS／Rust 已安排 argv 第一筆。這與直接傳給 try_parse_from 的完整序列分開。
- L209（checked）`前面` → 10-09：JSON 的 child test 解析**整份 stdout**，再核對全部模型角色；如果有人在資料前面插入「正在搜尋」，它就不能成為這份合法 JSON：
- L223（checked）`上一節` → 10-08：測試中的所有材料也都在這份來源中：文字以完整Rust literal建立，非法UTF-8是單byte255，缺檔與缺少參數的情況則刻意不提供。TempDir與child的順序沿用上一節。
- L235（checked）`第9.5` → 09-05：使用本節 search 取得 Report，沿用第9.5節的 slice iter／fold 計算 matches 的筆數。輸入 `Rust\nPython\nRust Rust\n`，預期是 2：它數符合的行，不數片段總共出現幾次。query 與原文規則不變。
- L245（checked）`第11章` → 11-02、11-03、11-07：這個工具已能接受原生檔名、搜尋 UTF-8 文字、選擇文字／JSON，並留下從純工作到完整行程的驗收。之後擴充時，先決定新行為與已教前置，再修改對應層；第11章會進一步介紹資源收尾、smart pointer 與完整錯誤介面。
- L247（checked）`下一章` → 11-01：[下一章：型別大小與資源管理](11-01-lesson.md)

## docs/chapters/15-01-lesson.md

全文實讀1–169；SHA-256 `4d4b83c88f8ba6ccfcac7866d71b7a0730e55a2f5afed39f4e3d7f7ce8ef3b4b`；引用位置14、mentions 21。

- L3（checked）`第6.4` → 06-04：第6.4節的 Extract 接手來源，交出該實作指定的 Output。現在要保留來源，只借讀內容：普通 Vec 可以交出 slice，RefCell 則交出代表活躍借用的 guard。我們希望用同一項「借出視圖」操作取得兩者，但保留各自的結果型別與借用規則。
- L9（checked）`第11.9、第12.1、後面` → 11-09、12-01、15-02、15-04：Numbers 儲存普通 Vec；GuardedNumbers 儲存 第11.9節 已教的 `RefCell<Vec<i32>>`。後者仍用原本的 borrow guard 管理讀寫。本章從教材repo根目錄操作，使用no-lib package `ch15-unsafe-stages`。每份example只載自己明寫的module快照；本節不會載入後面的variance或unsafe契約。完整設定採用第12.1節已教的根workspace共同欄位：
- L15（checked）`如下` → 15-01：預設binary只提示選example，完整src/main.rs如下；本頁的命令都明選example：
- L25（checked）`如下` → 15-01：我們要選的兩種視圖如下。表中的 `'a` 表示某一次對來源的借用，尚不是固定的全域 lifetime。
- L32（checked）`第11.9` → 11-09：兩種結果都不複製清單。區別仍沿用 第11.9節：普通 reference 的使用由編譯器檢查，Ref guard 還負責維持 RefCell 的執行期狀態。共同函式不應偷偷把 guard 換成沒有這項責任的普通 reference。
- L44（checked）`第11.5、第11.6` → 11-05、11-06：第11.5節的 `T: 'a` 要求 T 內的借用符合 `'a`。現在把 T 換成實作型別 Self，**`where Self: 'a`** 表示這個 View 只在 Self 符合該有效性要求時使用。where 的位置與語意沿用第11.6節，只是這次放在 associated type 的宣告上。
- L72（checked）`第7.2、第7.1` → 07-02、07-01：`S: BorrowView` 表示來源提供這份 trait。回傳 `S::View<'_>` 使用實作選定的視圖；`'_` 使用第7.2節的placeholder，依第7.1節的唯一reference輸入規則，接到 source 的借用。函式只轉交視圖，不取得 S，不把結果統一成新的清單：
- L78（checked）`下面` → 15-01：這項 bound 只保證有 view 操作，沒有保證未知的 `S::View<'_>` 一定能索引、Deref 或追加。下面在具體來源上呼叫，結果型別才分別確定成 slice 與 Ref guard，因此可以使用它們已教過的讀取能力。
- L80（checked）`第2.6、前面、後段` → 02-06、15-01：這是這一步實際載入的完整gat/mod.rs，包含前面的import、模型、契約、實作與helper，尚未加入後段測試。`pub mod gat;`沿第2.6節從example根載入它：
- L103（checked）`第11.9` → 11-09：普通 slice 的最後使用在第一行輸出；之後原 Vec 能 push。Ref guard 則沿 第11.9節 顯式 drop，才解除活躍 borrow；writer 在短 block 裡追加，離開 block 後再借 shared guard。GAT 保留這些差別，沒有改變 guard 的清理規則。
- L133（checked）`第7.2` → 07-02：source 隨內層 block 結束，View 內的 slice 沒有取得 Vec 的 ownership。修正仍沿第7.2節：需要借用就保留來源，需要獨立資料就交出 owned 內容。GAT 本身沒有延長資料存活。
- L153（checked）`上述、下面` → 15-01：晚段完整快照的三項測試分別核對原 slice 資料、guard 清理前後的寫入權限，以及兩種空清單的視圖。測試使用 **`Result::is_err(&self) -> bool`**：結果是 Err 時回傳 true，Ok 時回傳 false；它只檢查 variant，不取得內部值。這讓 assertion 能直接確認「guard 仍在時，取得 writer 的嘗試被拒絕」。這時才把上述helper與三項測試合成gat_complete；完整module與根程式都在下面。它的main沿用已教的視圖觀察，測試模式不執行main：
- L165（checked）`下一節` → 15-02：GAT 把「實作選哪種視圖」與「本次借用有效多久」結合起來。下一節則問另一個問題：已有較長 lifetime 的資料，放進不同型別後，哪些替換仍然合法？
- L169（checked）`下一節` → 15-02：[下一節：型別中的合法替換](15-02-lesson.md)

## docs/chapters/15-02-lesson.md

全文實讀1–178；SHA-256 `e87a0ee7d3577ac6d9d472acb519b57ac60eac6e23d52cb2e8caa17eddba1cc0`；引用位置17、mentions 20。

- L3（checked）`第11.5` → 11-05：第11.5節 已看到：固定文字的 `&'static str` 能滿足較短的借用需要。但如果把 reference 放進 Cell，或者放進 function pointer 的參數，能不能仍然用同一方向替換？
- L7（checked）`如下` → 15-02：本節從repo根目錄操作，仍用no-lib package `ch15-unsafe-stages`。完整manifest與預設提示binary如下；每次明選example，不載完整舊library：
- L19（checked）`第7.2、第11.5` → 07-02、11-05：沿 第7.2節 與 第11.5節 使用 **`'long: 'short`**：`'long` 至少涵蓋 `'short`。若使用端只在 `'short` 需要讀文字，已有 `'long` 的有效性就足夠。
- L25（checked）`第7.2、以下` → 07-02、15-02：以下兩個 helper 的 body 都只交回原 reference。泛型 lifetime 的宣告順序沿 第7.2節，`'long: 'short` 寫在 `'long` 後，限制兩者的關係：
- L31（checked）`下面` → 15-02：能編譯的 body 核對「對所有符合這項 bound 的 lifetime，這個方向成立」。回傳沒有 clone 文字或數值，也不額外配置記憶體。下面的輸出只是觀察如何使用回傳結果，不以數字相同當作 lifetime 規則的證明。
- L33（checked）`如下、後面` → 15-02：這一步只載兩個reference helper；完整variance/mod.rs及根程式如下，尚未載入後面的callback契約：
- L53（checked）`第3.5` → 03-05：這裡的 mutable reference 修改的是原 amount。借用結束後能讀 amount，和 第3.5節 的借用有效性一致；沒有產生第二份數字。
- L67（checked）`前面` → 15-02：不是「mutable 就全部 invariant」。`&'a mut T` 對自己的借用 lifetime `'a` 是 covariant，所以前面的 shorten_mutable 合法；它對被借型別 T 則是 invariant。接下來用兩層 reference 看清楚第二項限制。
- L75（checked）`下面` → 15-02：下面是完整 compile-fail fixture，會得到 `E0597`，指出 local 無法滿足 saved 中 reference 的要求：
- L91（checked）`第11.9` → 11-09：第11.9節 的 Cell::set 可以從 `&self` 替換內部值。因此 `Cell<T>` 對 T 也是 invariant，不能因為操作時只有 shared reference 就當成只讀。
- L95（checked）`下圖` → 15-02：下圖中箭頭只表示假設允許改換型別後，短借用會被寫回哪個共用容器；禁止符號表示這條型別替換不成立，沒有實際執行失效存取。
- L99（checked）`下面` → 15-02：下面的 helper 即使按值接手 Cell，也不能直接把 `Cell<&'long str>` 改成 `Cell<&'short str>`。variance 是這個型別對參數的替換規則，不會因為本次 body 恰好只轉交值就改變：
- L115（checked）`第9.2` → 09-02：第9.2節 的 `fn(i32) -> i32` 描述可呼叫的 function pointer。現在把參數與回傳值換成已教的 reference。`fn(&'a str) -> usize` 接收符合 `'a` 的文字借用，回傳 byte 數；`fn() -> &'a str` 不接收參數，交回符合 `'a` 的文字借用。
- L126（checked）`下列` → 15-02：下列 helper 接手 function pointer，原樣交回；回傳型別也以 fn 開頭，所以完整箭頭右邊是另一個「可呼叫型別」，不是本次立即呼叫取得的 usize 或 reference。
- L148（checked）`前面` → 15-02：這一步的完整module保留前面兩個helper，再加入已教的兩個adapter；完整根程式只載它：
- L162（checked）`第11.9` → 11-09：寫 copied_cell，收到 `&Cell<&'long str>`，在 `'long: 'short` 的條件下回傳新的 `Cell<&'short str>`。使用 第11.9節 的 get 取得 Copy 的 shared reference，再用 Cell::new 建立獨立容器；不要直接回傳原 Cell。
- L178（checked）`下一節` → 15-03：[下一節：raw pointer與合法存取](15-03-lesson.md)

## docs/chapters/15-03-lesson.md

全文實讀1–173；SHA-256 `8dab364a2465746176fc776a154b507efc35ba3c4e0b89b69d9b19a96512f18e`；引用位置15、mentions 19。

- L3（checked）`第3.3、3.4、第11.2` → 03-03、03-04、11-02：第3.3–3.4節的reference 把存取能力與借用有效性放進型別；第11.2節的Box 另外管理 owned 資料與清理責任。接下來要認識一種較低層的表示：儲存資料位置，卻不由型別替我們完成相同的有效性檢查。
- L5（checked）`後面` → 15-04、15-06：這種表示叫 **raw pointer**。本節先在單一 thread 中，從已知的局部數字與 array 建立 pointer，分開觀察建立、檢查、讀寫三種動作。直接使用 reference 已能完成這些數值修改；這次採 raw pointer，是為了理解後面安全抽象與外部函式介面所要處理的責任。
- L7（checked）`以下、後段` → 15-03、15-04：本頁從repo根目錄操作，沿用no-lib package `ch15-unsafe-stages`。以下完整設定與提示binary都只用已教能力；每個raw example是獨立檔案，沒有自動載入後段contracts：
- L26（checked）`後面` → 15-03：這裡的 `*` 是型別名稱的一部分。後面表達式中的 `*pointer` 才是 dereference：存取 pointer 指向的那個位置。型別中的 mut 也不等於 pointer 變數本身的 binding 必須是 mutable；我們可以保留同一份 pointer 值，透過它修改資料。
- L38（checked）`以下` → 15-03：本例的 place 是仍存在、已初始化的局部 i32，因此可以清楚追蹤來源。另使用以下安全 API；它們都不讀取被指向的 T：
- L42（checked）`第5.1` → 05-01：| `std::ptr::null::<T>()` | 建立型別為 `*const T` 的 null pointer，不配置 T。turbofish 沿第5.1節。 |
- L46（checked）`第11.1` → 11-01：**alignment（對齊）** 限制某型別可從哪些地址正常存取。例如 alignment 為 4，就要求起始地址是 4 的倍數；這和 第11.1節的size 不同，size 說明佔多少 bytes。實際要求依型別與平臺查詢，不能從「地址非零」推出它已正確 aligned。
- L64（checked）`以下` → 15-03：對本節正常讀寫一個 i32，至少要有以下條件。將 pointer 轉成其他型別，或只加入 is_null 檢查，都無法代替這些理由：
- L76（checked）`下面` → 15-03：下面保持同一個 amount，只用一份 raw pointer。讀出的 i32 是 Copy 數值，可以放進 tuple 交出來顯示；沒有讓 println 在 raw 存取中另借同一個位置。`SAFETY` 註解記錄這次操作的依據，不是因為寫了註解就自動安全。
- L91（checked）`前面` → 15-03：unsafe block 中讀到80，將原位置改成100，再讀到100。之後不再使用 pointer 存取資料，才由 amount 讀回結果。正常執行支援這個有限例子的值變化；安全理由仍來自前面的來源、有效範圍與存取權限，而不是輸出剛好正確。
- L93（checked）`下面` → 15-03：如果省掉 unsafe block，下面完整 fixture 得到 `E0133`：raw pointer dereference 需要 unsafe block。
- L111（checked）`第11.4、前面` → 11-04、03-07：這裡第一次直接把 array reference 交給 slice 介面：當預期型別為 `&mut [i32]`，Rust 可將 `&mut values` 的 array reference 轉成借同一份元素的 slice reference；shared 的 `&values` 也可轉成 `&[i32]`。長度來自原 array，沒有複製元素或延長來源存活。這是語言提供的 array→slice coercion，與第11.4節的Deref coercion 是不同的轉換規則；前面的 `&mut values[..]` 仍可使用。
- L125（checked）`下面` → 15-03：本例的 i32 array 提供 `Debug` 與 `PartialEq`：`Debug` 可列出各元素；與同長度的 i32 array 比較時，每個位置的值都相等，兩份 array 才相等。因此下面可用已教的 `assert_eq!` 核對整份 array，不是只因 i32 有能力就假定任何容器都相同。
- L153（checked）`下一節` → 15-04：reference 的回傳 lifetime 還必須連到真正的有效來源。不能僅由 raw pointer 加上 `'static` 回傳型別，就讓局部資料延長存活。下一節會在有限 safe wrapper 中，把這些前置條件與輸出關係逐項整理。
- L173（checked）`下一節` → 15-04：[下一節：unsafe呼叫契約與安全抽象](15-04-lesson.md)

## docs/chapters/15-04-lesson.md

全文實讀1–287；SHA-256 `3f60eb8974d39620c01dd5719914f728739bfbca6fe7507ce091d712b78ecd4f`；引用位置30、mentions 34。

- L3（checked）`第15.3` → 15-03：第15.3節在每一次raw存取前確認來源與權限。現在希望把操作整理成可重用的介面：哪些條件仍要由caller確認，哪些可以由函式的輸入型別與檢查接手？最後再讓一份只讀借用跨thread，分開核對有效性與Send／Sync。
- L7（checked）`以下` → 15-04：本頁從repo根目錄操作，沿用no-lib package `ch15-unsafe-stages`。以下是完整manifest與預設提示binary；早期read、切分與晚段跨thread各有獨立contracts快照：
- L21（checked）`第15.3` → 15-03：先把第15.3節的i32讀取整理成read_i32。它收到 `*const i32`，回傳普通i32副本，不接手原資料或配置，也不交回reference。**呼叫前**必須確認：
- L30（checked）`下面` → 15-04：下面放在這份example的contracts module。`///` 是Rustdoc文件註解，放在item之前；`# Safety` 是文件中的小標，集中保存caller要遵守的契約。它與body前的SAFETY註解分工：前者說需要什麼，後者說這次操作如何滿足需要。
- L40（checked）`如下` → 15-04：這一步的完整contracts/mod.rs如下；根檔案的`pub mod contracts;`只載這份快照：
- L60（checked）`下面` → 15-04：若省掉**呼叫處**的unsafe block，即使這次pointer來自有效局部值，下面完整fixture也得到 `E0133`：
- L72（checked）`第15.3` → 15-03：這次缺的是unsafe函式呼叫的責任標記，與第15.3節直接dereference缺block的fixture不同。加block後仍要履行read_i32契約，不是讓任何pointer都可讀。
- L78（checked）`第15.3` → 15-03：**soundness（健全性）** 要求：合法的safe client使用這份介面，不能因此觸發UB。若safe client能把合法輸入交進去、導致實作違反記憶體契約，這份介面就是**unsound**。第15.3節已說明，UB不是普通的Err或固定會crash；測試剛好正常不能把它變合法。
- L86（checked）`第3.7` → 03-07：第3.7節的range上界不包含終點，所以左段是 `0..mid`，右段是 `mid..len`。mid可以是0或len，表示其中一段為空；mid超過len時，本節介面回None，不能先形成越界pointer。
- L88（checked）`如下` → 15-04：先試只寫兩份mutable索引借用，完整fixture如下：
- L106（checked）`以下` → 15-04：**`std::slice::from_raw_parts_mut(data, len)`** 是unsafe函式，收到 `*mut T` 與usize元素數量，回傳 `&mut [T]`。它不配置、clone或驗證這份資料；**建立reference時**就要滿足以下條件：
- L114（checked）`第7.1` → 07-01：| 回傳reference的有效性連到真實來源 | safe函式簽名沿第7.1節連到唯一的mutable reference輸入，不回傳任意static借用。 |
- L118（checked）`第15.3` → 15-03：這回應第15.3節的區別：可以形成端點，不代表能把端點當成下一項讀取。from_raw_parts_mut讓pointer變成reference，還多了回傳borrow整段期間的責任。
- L122（checked）`第15.3` → 15-03：`split_i32_at_mut` 是本節定義的safe函式；輸入、兩個回傳reference的有效性連到同一來源。實作先用 `use std::slice;` 引入module名稱。當mid太大，先回None；合法時才取得data、使用第15.3節的add和新的from_raw_parts_mut：
- L134（checked）`如下` → 15-04：這一步的完整contracts/mod.rs如下；根檔案的`pub mod contracts;`只載這份快照：
- L158（checked）`第13.1` → 13-01：第13.1節已教Send與Sync。現在補上它們的一項責任：它們是 **unsafe traits**，手寫實作時，作者必須維持編譯器不能完整核對的額外安全契約。**`unsafe impl`** 表示實作者承擔這份責任；它不替型別加入鎖、atomic更新或新的storage生命週期。
- L160（checked）`第13.1` → 13-01：raw pointer自身沒有Send／Sync。只知道它指向i32，不能推來源有效、所有存取都只讀，或其他路徑不在寫入。先沿第13.1節的bound helper確認這項限制：
- L174（checked）`第13.1` → 13-01：第13.1節已認識data race。對本節的普通i32，跨thread未經同步安排的衝突存取、且至少一方寫入，就會形成data race，屬於UB。兩個thread都只讀同一個普通i32沒有這類寫入衝突。接下來只提供第二種有限介面，不讓它變成共享可寫容器。
- L184（checked）`第7.4` → 07-04：ReadOnlyI32的new接收shared reference，建立這項關係；read沿read_i32取得Copy數值；original交回源自'a的shared reference，沿第7.4節已教的原來源回傳契約。兩個欄位都沒有pub，library外只能由new建立，不能自行配一個source與別份pointer。
- L186（checked）`第15.3` → 15-03：`&raw const *source` 用第15.3節的raw borrow operator，place是source已借到的原i32。它沒有先拿到其他來源的地址，也不建立mutable reference：
- L194（checked）`第15.3、下面` → 15-03、15-04：若source已結束，wrapper也不能留下來讀。下面完整fixture得到 `E0597`；它重現同一來源關係，read只用第15.3節已教的raw dereference，沒有繞過來源的reference欄位：
- L208（checked）`以下` → 15-04：對**這一份i32型別與已列出的全部介面**，可以承擔以下兩項實作：
- L215（checked）`第7.2、前面` → 07-02、15-04：沿第7.2節已教的placeholder lifetime，這裡在impl的型別位置寫`ReadOnlyI32<'_>`， 讓這份實作適用於其合法的借用lifetime，不要求來源變static。Send與Sync是沒有methods的marker traits，因而impl body是空的；安全責任由前面的型別性質及介面支持：
- L223（checked）`第13.4` → 13-04：用第13.4節的scoped thread保留局部來源：第一份工作以move接手moved，使用Send；第二份借用shared，使用Sync所提供的 `&ReadOnlyI32: Send`。兩份wrapper都由同一個amount建立，沒有複製另一份i32配置：
- L225（checked）`如下` → 15-04：這一步的完整contracts/mod.rs如下；根檔案的`pub mod contracts;`只載這份快照：
- L251（checked）`如下` → 15-04：再切在len，確認right長度0，沒有試圖索引空段。完整答案與它實際載入的完整切分快照如下：
- L265（checked）`下面` → 15-04：最後的完整快照保留六項tests分別檢查raw read不接手來源、不同範圍修改原元素、0／len兩端、空Vec、拒絕mid後資料未變，以及wrapper移交／共享讀取。這些是有明確答案的案例，可用下面明選的example測試檢查；它們不能涵蓋所有輸入、來源狀態或排程，也不由六項PASS宣佈實作已獲完整安全證明。
- L267（checked）`如下、先前` → 15-04：這份晚段contracts_complete才合併本節全部介面與原六項測試；先前的unsafe_read與safe_split不會因此載入它。完整source如下：
- L283（checked）`第17.6、接下來的` → 17-06、15-04：第17.6節會再教Miri與Loom的用途、操作及限制，補充可執行路徑或並行模型的查覈。本節先把契約寫足；接下來的C與外部函式介面，會再處理型別、配置與釋放責任。
- L287（checked）`下一節` → 15-05：[下一節：FFI所需的C基礎](15-05-lesson.md)

## docs/chapters/15-05-lesson.md

全文實讀1–302；SHA-256 `d7a229d638429477507f1ca2beb0d742c008be9380f724d527f8ec583d15f04a`；引用位置26、mentions 29。

- L3（checked）`第15.4` → 15-04：接下來要讓 Rust 呼叫 C 函式。**FFI（foreign function interface，外部函式介面）** 是跨程式語言呼叫函式的介面；要把它包成安全的 Rust API，就得讀懂對方的資料表示、存取條件與清理責任。第15.4節已練過這種契約分工，現在把來源換成 C。
- L5（checked）`下節` → 15-06：本節先做一個獨立的 C 程式：修改一個數值、加總數列、區分字串與 buffer，最後配置一份數列副本並釋放。你不需要先學過 C；下節才加入 Rust 的呼叫程式。
- L7（checked）`下方` → 15-05：範例選 **C11**，也就是 2011 年版本的 C 標準，使用一般作業系統上的 C compiler 與標準函式庫。下方指令適用於提供 `cc` 命令的 GCC／Clang 環境；作者在 Linux、GCC 14.2 驗證。所有範例都在單一 thread 執行。
- L32（checked）`下面` → 15-05：**`sizeof`** 是 C 的大小查詢運算子，結果型別是 size_t。下面的 `sizeof values` 得到整個 array 的 byte 數，`sizeof values[0]` 得到一項的 byte 數；相除得到元素數量 3。這兩個數的單位都是 byte，商才是項數。在本節固定大小型別下，sizeof 不實際讀取 values[0] 的值。
- L34（checked）`以下、後面` → 15-05：以下兩行會放在後面的 main 裡；所需型別由 header 提供：
- L50（checked）`第15.3` → 15-03：沿用 第15.3節的有效性觀念：在 dereference 前，pointer 必須來自仍存活的正確物件、符合 alignment，並且這次操作有讀寫資格。**NULL** 是標準 header 提供的 null pointer 表示，表示沒有指向可存取的物件；宣告了 pointer，或值不是 NULL，都不會自動完成存取條件。
- L58（checked）`以下` → 15-05：以下片段中，amount 先是 80，pointer 由它建立；呼叫期間 amount 還存在且可以寫入。target 是 pointer 值的副本，因此 `*target = value` 改的就是原 amount，結果為 100：
- L64（checked）`後面` → 15-05：離開 amount 所在的區塊後，它的儲存不再可用。C 沒有自動替 pointer 延長來源 lifetime，也沒有 Rust borrow checker 阻止你保存失效的 pointer；後面的範例都不讓 pointer 逃出來源的有效期間。
- L77（checked）`下面` → 15-05：| len 為 0 時不存取元素，也不做 pointer arithmetic | 下面迴圈一次都不進入；這份特定介面因而允許 NULL。 |
- L79（checked）`第15.4` → 15-04：NULL 不是空 array 裡的一項。上表的空輸入規則屬於本節 helper，不能概括成所有 C API 都接受 NULL，也不同於 第15.4節零長度 Rust slice 仍須 non-null／aligned 的條件。
- L81（checked）`下面` → 15-05：C 的 `for (起始; 條件; 更新)` 先執行起始，條件成立才跑 body，每輪結束後更新，再檢查條件。`<` 是比較；`+=` 把右側加到原值。下面從 index0 開始，每輪加一，只有 index 小於 len 時才讀 values[index]：
- L99（checked）`下面` → 15-05：**buffer（緩衝區）** 則是一段供處理的儲存空間，零可以是普通資料。下面的 uint8_t array 有三項，第二項雖是零，第三項 20 仍在範圍內：
- L113（checked）`下面` → 15-05：C 的 `if (條件) { ... }` 在條件成立時執行區塊；`!=` 表示不相等。比較結果是 0 或 1，條件為零視為不成立、非零視為成立。下面每輪都往下一項，不因元素為零而結束：
- L123（checked）`前面` → 15-05：前面 array 的儲存由所在區塊管理，區塊結束後就不能再用。現在希望練習另一種責任：向 C allocator 取得一塊儲存，由程式明確釋放。`<stdlib.h>` 提供 **`malloc(byte 數)`** 與 **`free(pointer)`**：
- L142（checked）`上面` → 15-05：這裡真的逐項複製到**另一份配置**。只把 owned 指派給另一個 pointer，得到的仍是同一配置的別名，沒有新增資料副本或一份可另行 free 的 ownership。函式接收 pointer 也不自動取得清理責任；上面兩個讀取 helper 都明訂不保存、也不釋放。
- L144（checked）`下一節` → 15-06：下一節會讓這種配置越過語言邊界。先記住目前契約：誰配置、誰負責呼叫對應的釋放函式、借用者何時必須停止存取；不能憑 pointer 的型別猜測清理規則。
- L163（checked）`前面` → 15-05：- **counts.c** 提供前面已讀過的三個 body。
- L166（checked）`如下、前面` → 15-05：完整 header 如下；本例每個 C source 都只納入它一次。`#include "counts.h"` 的雙引號寫法會先在目前 source 所在目錄找這個專案 header；前面的角括號寫法用於標準 headers：
- L172（checked）`前面、前面` → 15-05：counts.c開頭納入同一份header，再放前面三個定義，讓compiler也核對實作與prototype的型別一致。這是完整counts.c，不只前面的region：
- L178（checked）`如下` → 15-05：完整driver如下；它先釋放owned，再使用已算好的整數結果印出：
- L186（checked）`第12.2` → 12-02：第12.2節已分清 host 上的建置工作與最後程式的執行。這次用 **host C compiler** 編譯一個在同一臺 host 執行的 C 程式，還沒有 Rust 的跨語言建置步驟。
- L233（checked）`下面` → 15-05：下面的完整故意失敗程式，只少傳 sum_values 所需的 len：
- L255（checked）`前面` → 15-05：檢查程式用已教的 if／!= 比對結果，失敗回 EXIT_FAILURE，成功回 EXIT_SUCCESS。這次選空輸入、正負抵消、同一 array 只讀前段、buffer 內零值，以及 pointer 修改原值；每個案例都有前面契約可用，沒有越界或 overflow：
- L261（checked）`下面` → 15-05：cc 也可以一次接收兩份 C source，依序完成編譯與連結；下面沒有 `-c`，所以 `-o` 指定的是可執行檔。正常執行沒有輸出，回成功狀態：
- L298（checked）`第15.6` → 15-06：讀完這節，應能說明 C 函式讀寫哪些資料、長度從哪裡來、來源可以用到何時，以及誰負責釋放。第15.6節才把這些明確的條件帶入 Rust 呼叫與安全 wrapper。
- L300（checked）`下一節` → 15-06：[下一節：把C契約接成安全Rust介面](15-06-lesson.md)

## docs/chapters/15-06-lesson.md

全文實讀1–579；SHA-256 `ed5a9511f205772cacba2f95e4c4b5b25744e59c039fdd4c656e78d090e713a7`；引用位置35、mentions 46。

- L3（checked）`第15.5` → 15-05：第15.5節 已知道 C 函式如何收 pointer、如何決定處理範圍，以及 malloc 配置必須交回對應的 free。現在要讓 Rust 使用這些能力，同時讓普通 caller 不必自行處理 raw pointer。
- L7（checked）`第15.5` → 15-05：沿用 Linux x86_64、GCC14.2、Rust1.99／edition2024 的驗證環境，C 採 C11。C source 隨 package 提供，不需另外下載大型 C library；host C compiler 與標準函式庫的準備沿第15.5節。例子同步執行，C 不保存輸入 pointer，也不回頭呼叫 Rust。
- L11（checked）`後段` → 15-06：從repo根目錄執行。本節保留三份獨立階段package：`ch15-ffi-scalar`、`ch15-ffi-buffer`、`ch15-ffi-label`，分別在`examples/ch12-ffi/stages/`下的scalar、buffer、label。最後才使用永久完整的`ch12-ffi`。每份stage有自己的C/header/build/library，早期版本不載後段介面。先讓C提供版本號1。
- L13（checked）`第15.5、前節` → 15-05：第15.5節 已用過 uint8_t／int32_t。這次的 **`uint32_t`** 是 `<stdint.h>` 提供的恰好32 bits無號整數，對應 Rust u32；範圍是0到u32::MAX。這個 C 型別與前節 exact-width 型別一樣，需要 C 實作有提供；本節 target 有提供。
- L33（checked）`下方` → 15-06：把 raw declarations 放在 library 的 private `sys` module，src/lib.rs 以 `mod sys;` 載入src/sys.rs。sys 裡的函式標pub供父層wrapper呼叫，但sys本身沒有公開，library外的caller只能使用下方公有wrapper：
- L45（checked）`下面` → 15-06：這個wrapper的安全理由包含 C 的實作、正確宣告與相同target，而不只是「Rust body 放了unsafe」。下面的caller只使用safe API，執行前先完成下一段的建置設定：
- L53（checked）`第15.5、第12.2` → 15-05、12-02：第15.5節 已分開 C 的個別編譯與連結；第12.2節 已教 build.rs 在host執行，並透過輸出指令與Cargo合作。現在把這兩件事接起來。
- L55（checked）`第12.1、第12.2、第12.2` → 12-01、12-02：各份manifest的共同欄位沿第12.1節，`build = "build.rs"`明寫第12.2節已教的script入口。本節加入 **cc crate**，它是供build script使用的Rust函式庫，幫忙呼叫適合target的C compiler與建立archive；不是另一個C compiler，也不是執行example的工具。依第12.2節把它放在 `[build-dependencies]`，版本固定為本節驗證的1.8.0：
- L61（checked）`上述` → 15-06：**static library（靜態函式庫）** 把編好的object files裝成archive，連結時取所需的C機器碼放進最終程式。這裡要得到名為tutorial_c的archive，供Rust library呼叫上述C symbol。**symbol（符號）** 是object／linker辨識的函式名稱等入口；tutorial_version的body必須真的出現在被連結的C程式碼裡。
- L74（definite）`前五項、前節、如下` → 15-06、15-05、10-08：前五項組態操作可以依序串接；前節已學過的mutable設定與method呼叫在這裡連續使用。build.rs如下：
- L80（checked）`第12.2` → 12-02：兩個rerun-if-changed沿第12.2節：C body或header變動後重做。cc把object／archive放在Cargo提供的OUT_DIR；在本節GCC環境，archive檔名會是libtutorial_c.a。它也提供 **rustc-link-search** 讓linker知道搜尋目錄，以及 **rustc-link-lib=static=tutorial_c** 表示需要連結哪個archive。這些是build script給Cargo／rustc的資訊，不是宣告C函式簽名。
- L86（checked）`如下` → 15-06：這份scalar快照的完整七個檔案如下。C只含版本函式，Rust library也只含api_version；此時不會編譯未知的結果struct、CString或C-owned owner。
- L140（checked）`第15.5` → 15-05：1來自已連結的C body，不是Rust wrapper自行回傳的literal。若只有extern宣告、沒有連結C實作，仍會像第15.5節缺少counts.o那樣遇到找不到symbol的連結失敗。若C本身有語法錯誤，則在build script的C編譯階段就失敗；兩種問題都不能靠給Rust呼叫加unsafe解決。
- L144（checked）`第15.5` → 15-05：沿第15.5節的byte buffer `[10, 0, 20]`，這次想同時得到「零值1項、非零2項」。輸入零仍是資料，長度由caller明示，不當字串掃描。
- L148（checked）`如下` → 15-06：header中的結果型別與函式契約如下：
- L176（checked）`第8.4` → 08-04：safe wrapper不能要求caller自己保證「slice不太長」。slice的len是usize，C介面明選uint32_t；wrapper先用第8.4節的TryFrom檢查。這裡標準u32提供 `try_from(usize)`：可表示才回Ok，否則Err；不先截斷再傳。private helper checked_len把這項失敗映成CountError::TooLong，?在失敗時立即回去，不呼叫C：
- L182（checked）`第15.3` → 15-03：bytes的shared borrow提供有效範圍、初始化與呼叫期間的來源；as_ptr沿第15.3節取得pointer，不接手原buffer。C只處理這份完整長度且不寫入、不保存，故呼叫返回後borrow工作就完成。空slice也有可形成的data pointer，但C的len0分支完全不讀它，不製造空資料元素。
- L190（checked）`以下` → 15-06：這一步改選buffer快照。它保留已教version，再加入Counts與count_bytes；以下完整C/Rust/build設定沒有CString或配置工廠。
- L250（checked）`第15.5` → 15-05：第15.5節的C string需要末尾零，但Rust str保存長度且保證UTF-8，沒有承諾末尾零。這回讓C判定輸入是否完全等於Rust四個字元；不能直接把普通str pointer當成C string。
- L287（checked）`前兩步` → 15-06：這一步改選label快照，完整source保留前兩步，加入剛教的字串比較。所有呼叫材料都在example裡，不由helper偷偷造C字串。
- L343（checked）`上一段` → 15-06：前兩項是C比較的結果；第三項是Rust端格式檢查。若需求本來就是含零的一般資料，應用上一段pointer＋len介面，而不是強迫它符合C string。
- L347（checked）`前一段` → 15-06：前一段C借Rust的字串，現在反過來：C配置五個char，內容為Rust加末尾零，交給Rust使用，最後仍須呼叫這個C函式庫的釋放函式。
- L355（checked）`第15.5` → 15-05：C先用第15.5節相同的char array初始化，再malloc五bytes、檢查NULL；迴圈連末尾零一起複製，**完成全部初始化後**才return配置pointer。這裡交回的是data指向的malloc儲存，不是局部word的pointer；word離開scope不會使已複製的新配置結束。
- L369（checked）`第11.3` → 11-03：這時不能只用Box接手pointer，因為「看起來都指向heap」沒有證明Rust allocator與這個C釋放契約相符。本節也不把C malloc當作CString自己的配置。選一個專用 **Greeting owner**，私有欄位只保存這份原始pointer，由第11.3節的Drop呼叫指定C函式。
- L377（checked）`第7.4` → 07-04：| `as_c_str` | 借&self，回&CStr。只借原內容，回傳有效性依第7.4節連到self；不是static，也不延長owner。 |
- L384（checked）`如下` → 15-06：lib.rs用 `use std::ffi::CStr;`與c_char引入名稱，完整owner如下：
- L392（checked）`如下` → 15-06：正常流程如下：箭頭表示這次操作的順序。&CStr借的仍是第一步C配置；第三步另建的Rust String才是一份可在free後繼續用的內容副本：
- L400（checked）`前面` → 15-06：現在改選永久完整版`ch12-ffi`，目錄就是`examples/ch12-ffi/`；它提供前面全部safe API與這個Greeting owner。
- L404（checked）`第15.1、上方` → 15-01、15-06：字串測試用第15.1節已教的Result::is_err判斷格式失敗；expect所需的Debug由本節CountError／標準錯誤型別提供。所有helpers／tests都沿上方介面，不需要未知callback、allocator框架或跨thread實作：
- L453（optional）`第10.4、第11.3、上面` → 10-04、11-03、15-06：第一行仍借C配置；to_owned沿第10.4節建立獨立Rust String。drop(greeting)沿第11.3節接手owner、執行對應cleanup，之後只讀saved。輸出核對內容關係，配對釋放與borrow安全則由上面C body／Rust介面契約支持。
- L457（checked）`下列` → 15-06：下列fixture只確認指定編譯規則，不執行ABI錯配、失效pointer或double-free作示範。
- L483（checked）`前面` → 15-06：兩個fixture都在連結前被拒絕，不靠真的執行未連結的C函數。即使版本函式這次沒有額外輸入，宣告預設unsafe的規則仍在；日常caller用前面的safe api_version。
- L521（checked）`前面、下列` → 15-06：寫一個新的example，按下列步驟連起前面的safe APIs：
- L549（checked）`上面、如下` → 15-06：永久package也保留三份呼叫端供回顧；它們只使用上面已教的API，library路徑現在是ch12_ffi。完整來源如下：
- L579（checked）`下一章` → 16-01：[下一章：self-reference與地址穩定](16-01-lesson.md)

## docs/chapters/16-01-lesson.md

全文實讀1–168；SHA-256 `a02196df700ac59c39a0d5d0a73a457d54661a69110d145086deac890aca84a3`；引用位置16、mentions 21。

- L3（checked）`第14.3` → 14-03：第14.3節的 `controlled_work` 接手 owned `release` 收端，之後停在 `release.recv().await` 等放行。外層工作必須保存收端；這份 recv 等待又 mutable 借用它。等待尚未結束時，光說「收端還活著」並不夠：**裡面保存的借用，稍後還要找到同一份資料。**
- L13（checked）`14.3、下圖` → 14-03、16-01：下圖接回14.3的工作。兩個框都在同一份外層 future 中，向上的箭頭表示借用收端；這是資料關係，**不是編譯器承諾的 struct 欄位布局**：
- L17（checked）`下面` → 16-01：若整份狀態換了位置，內部保存的地址不會只因「變數還叫 release」就自動更新。下面暫時移除 channel，只保留 i32 和地址記錄，逐項看發生了什麼。
- L21（checked）`第3.2、第11.3、第7章` → 03-02、11-03、07-01、07-02、07-03、07-04：第7章的 lifetime 描述借用來源與可用範圍，不會延長原值存活；第3.2節的 move 轉交 ownership，第11.3節也看過 move 不等於當場 Drop。這些規則仍成立，卻不會替我們重寫值裡的 raw pointer。
- L23（checked）`稍後` → 16-01：這裡的**地址穩定性**，是某份資料在需要它的期間保留原位置。move 是 ownership 語意；是否真的搬動我們在意的資料，要看交接的是哪一層。稍後交換 Box owner，就會與交換 Box 裡的資料有不同結果。
- L29（checked）`第7.3、下面` → 07-03、16-01：用 `use std::mem;` 引入 module，便能寫 `mem::swap(&mut left, &mut right)`。如果欄位真的保存 reference，之後還要使用這份借用，borrow checker 會拒絕下面的整值交換。`BorrowedNote<'a>` 沿第7.3節保存 borrowed 資料：
- L45（checked）`第12.1` → 12-01：本章早期例子位於沒有 library 的 **`ch16-future-stages`**。完整 member 設定沿第12.1節繼承根 workspace 已教的共同欄位；不需要第三方依賴：
- L51（checked）`下方` → 16-01：完整預設入口只提示選例；下方命令都從 repo 根目錄執行，明確選定當下的 example：
- L59（checked）`第15.3、15.4` → 15-03、15-04：沿第15.3、15.4節，`ptr::null()` 先建立「尚未記錄」的 null pointer，`&raw const self.value` 取得當下欄位的 raw pointer；這裡都不解參考它。
- L63（checked）`如下` → 16-01：完整 `addresses/mod.rs` 如下。`new` 先不記自己的地址，因為回傳後還要交給 caller；`remember` 才記下目前位置；`address_matches` 只比較，不修復 saved：
- L95（checked）`第11.2` → 11-02：第11.2節的 Box 把 owned T 放在 heap 配置中；轉交 Box owner，不需要連那份配置一起搬走。現在先用 `Box::new` 放好 AddressNote，再記 heap 中 value 欄位的位置。
- L97（checked）`11.2` → 11-02：這次 swap 的 T 是 **`Box<AddressNote>`**，交換的是兩個 Box owner。`(*left).remember()` 先找到 Box 的內容再呼叫方法；`&*left` 借出內容，都是11.2已用的操作。這份 target 的完整本地 module 與入口為：
- L150（checked）`下面` → 16-01：本節完整測試快照含四項 tests：整值交換與明確重記、相同數字仍有不同位置、交換 Box owner，以及只換欄位數字。下面列全 module 與 root；它們沒有透過 saved 讀取資料：
- L164（checked）`下一節` → 16-02：回到 async，合法的內部借用除了來源活著，還需要保存它依賴的位置關係。日常 await 已由語言與 runtime 的介面配合，不要求 caller 自行建立 raw pointer。下一節認識 Pin／Unpin，看看怎樣把這份共同約定放進型別介面，也把 Drop 收尾納入責任。
- L166（checked）`下一節` → 16-02：延伸閱讀：[Async Book：Pinning](https://rust-lang.github.io/async-book/part-reference/pinning.html)、[中文 Async Book：固定](https://huangjj27.github.io/async-book/04_pinning/01_chapter.html)、[std Pin 的地址敏感狀態](https://doc.rust-lang.org/std/pin/index.html)。中文舊例含 raw 解參考與下一節 API；本頁用數字及地址比較建立動機。
- L168（checked）`下一節` → 16-02：[下一節：Pin 與 Unpin](16-02-lesson.md)

## docs/chapters/16-02-lesson.md

全文實讀1–212；SHA-256 `e23aa436d2fae0c6d9a0d80d17f053ae3bed168bc40a1cd42ea00fdce2a54aca`；引用位置16、mentions 18。

- L3（checked）`上一節` → 16-01：上一節交換整份80／120資料，地址記錄便對不上；只交換 Box owner，內部資料則留在原位置。但普通 Box 還允許移出內容。現在要讓其他程式處理地址敏感的資料，同時維持約定：**原值需要保留位置時，不能任意移出；回收或重用空間前，也要先在原位收尾。**
- L5（checked）`下一節` → 16-03：不同 async 工作可能有不同內部借用。共同的介面能讓使用端遵守同一約定，不必猜每份 future 裡有哪些 pointer。本節仍先用數字與地址觀察，下一節才呼叫 poll。
- L21（checked）`第13.1` → 13-01：**`Unpin`** 是 `std::marker` 中的 auto trait，沿第13.1節，由型別與欄位性質決定是否自動具備。它表示型別不依賴 pinning 保證。當 **`T: Unpin`**，Pin 可以交出普通 mutable 存取，不再額外禁止移出 T；ownership／borrowing 規則仍要遵守。
- L29（checked）`12.1、如下` → 12-01、16-02：本節仍使用無 library 的 `ch16-future-stages`。完整 manifest 與預設入口如下，沿12.1繼承根共同欄位：
- L39（checked）`下面、上一節` → 16-02、16-01：下面是完整的 i32 程式。取得 mutable reference 後，使用上一節的 swap：
- L55（checked）`上一節` → 16-01：上一節 AddressNote 的 i32 與 raw pointer 欄位都有 Unpin。因此只把它交給 `Box::pin`，仍不能依賴 Pin 禁止移出。現在另建 **PinnedNote**，保留數字與地址記錄，增加一個 marker 欄位。
- L83（checked）`第3.6` → 03-06：沿第3.6節 receiver 的用途，這裡寫 **`self: Pin<&Self>`** 或 **`self: Pin<&mut Self>`**，直接指定 method 接收的 wrapper。Self 仍是目前的 PinnedNote；第一種作 shared 觀察，第二種作受控修改，都沒有接手整份 PinnedNote。
- L85（checked）`如下` → 16-02：本模型的介面固定如下：
- L96（checked）`第11.3` → 11-03：pinning 不只管這次 method。對依賴它的原值，儲存空間在 **Drop 返回或 panic 前，不能先被回收或重用**；回收／重用前先在原位收尾，稱為 **Drop guarantee**。第11.3節的正常清理由 owner 安排；這裡多了位置要求，不是保證任何值最終都一定被清理。
- L100（checked）`下面` → 16-02：下面列完整模型、所有 methods、Drop 及逐處 SAFETY 理由；它始終不透過 saved 讀資料：
- L108（checked）`11.3` → 11-03：交換的是兩份 **`Pin<Box<PinnedNote>>`** owner；接著把 left 數字改140，再依11.3的 `mem::drop` 收尾。完整入口：
- L138（checked）`下面` → 16-02：下面的 LocalNote 只保存數字與 marker，沒有自引用；用它分開「不具 Unpin」「是否實際有自引用」及「存在哪一處」。value method 只需 shared Pin，沒有 unsafe：
- L192（checked）`下面` → 16-02：as_ref／as_mut 重新借用 owner，之後仍能再借。method 若接手 `Pin<&mut T>`，那份 mutable wrapper 本身不是 Copy；需要重複呼叫時，在 wrapper 上再次 as_mut，交出較短借用。下面第二個 test 以同一份 borrowed wrapper 反覆設負數和零，然後交接 owner。
- L208（checked）`下一節` → 16-03：現在 caller 能交入 `Pin<&mut T>`，讓依賴地址穩定的實作按約定工作；Unpin 型別也能使用相同介面。下一節用這個 receiver 讀真正的 Future::poll，再分清尚未完成、通知與再次執行。
- L210（checked）`下節` → 16-03：延伸閱讀：[std Pin 契約與 Drop guarantee](https://doc.rust-lang.org/std/pin/index.html)、[Pin methods](https://doc.rust-lang.org/std/pin/struct.Pin.html)、[Unpin](https://doc.rust-lang.org/std/marker/trait.Unpin.html)、[safe local pin!](https://doc.rust-lang.org/std/pin/macro.pin.html)。其他 projection API 與第三方工具不是下節必要材料。
- L212（checked）`下一節` → 16-03：[下一節：Poll、Context 與 Waker](16-03-lesson.md)

## docs/chapters/16-03-lesson.md

全文實讀1–209；SHA-256 `a95a5c1a963f7dce7c6be06c75e077f696a4dd6ffc01cc5d26f77155452e81d1`；引用位置15、mentions 22。

- L3（checked）`第14.2、上一節` → 14-02、16-02：第14.2節先取得 future，await 才取得它的 Output；上一節建立 `Pin<&mut T>` 的介面約定。現在把兩者接起來：一次檢查還不能交出答案時，怎樣留下通知位置？通知之後，又由誰真正再執行？
- L11（checked）`4.1、4.2` → 04-01、04-02：**`std::task::Poll<T>`** 是 enum，T 是工作完成時的結果型別。**`Poll::Pending`** 不帶 T，表示當下還不能交出 Output；**`Poll::Ready(value)`** 帶著完成的結果。沿4.1、4.2的 enum／match，使用端分別安排下一步。
- L13（checked）`14.2` → 14-02：Pending 不是 Option 的 None，也不是業務錯誤。沿14.2，若 Output 是 Result，`Ready(Err(...))` 仍表示工作完成，只是結果失敗；本節 Output 選 i32。
- L29（checked）`下面、稍後` → 16-03：現在讀標準 Future trait 的 poll 介面；下面只有宣告，完整實作稍後提供：
- L35（checked）`上一節` → 16-02：`self` 接手上一節已教的 mutable Pin 借用 wrapper，沒有接手整份 future。`cx` mutable 借用本次 Context；`'_` 請編譯器推導其中 Waker 的借用來源，不表示 static。外層 mutable 借用與 Context 內部來源，是兩項關係。
- L37（checked）`6.4、14.2、6.1` → 06-04、14-02、06-01：`Self::Output` 沿6.4與14.2，是實作選定的 associated type，不是值欄位；回傳的 Poll 描述這次檢查結果。`use std::future::Future;` 引入 trait，便能依6.1以點號呼叫它的 method。
- L47（checked）`13.6` → 13-06：沿13.6，WakeCounter 用 `Mutex<usize>` 保護短暫讀改寫，Arc 保活同一份計數。`new(initial_count)` 建觀察器；`notifications()` shared 借用後短暫 lock、讀副本、解鎖；wake 也只在短 lock 內加一。本節從0開始，只做有限幾次通知，不拿它作任意大次數的計數器。poison 的 expect 仍會 panic，不會修好資料。
- L59（checked）`稍後、剛才` → 16-03：先看 polling module 裡的完整觀察器部分；它用到的名稱來自剛才的 Arc、Mutex 與 Wake，整份 module 稍後列全：
- L77（checked）`稍後` → 16-05：**`waker.wake()`** 接手 owned Waker，執行通知回呼、回傳 unit。**`waker.wake_by_ref()`** 則 shared 借用 handle 通知，不消耗它；本章稍後會直接使用。兩者都沒有自己呼叫 Future::poll。Wake 的 default wake_by_ref 會 clone Arc，再呼叫 required wake，實作只提供 wake 也可以使用。
- L89（checked）`下面` → 16-03：下面是完整 polling module，含 imports、觀察器、普通 methods 與 Future 實作：
- L95（checked）`下一節` → 16-04：poll 只檢查、保存 handle 或交回答案，沒有阻塞等 release，也沒有忙等迴圈。release 和兩次 poll 目前都在同一個 main 依序進行；跨 thread 的狀態同步留給下一節。
- L127（checked）`下面` → 16-03：下面完整對照把數字改120，先 release 再首次 poll。相同模型的本地 module 和入口一併列出：
- L175（checked）`如下` → 16-03：四項 tests 核對通知不等於 poll、首次檢查前授權、最新 Waker，以及 owned clone 在原 Context 結束後仍能通知。完整 module 和 root 如下，沒有 executor helper：
- L205（checked）`第14.3、下一節` → 14-03、16-04：一般 await 把這項介面串進外層工作；driver 推進 future，future 交回當次狀態，通知提示需要再檢查。第14.3節已提交的 task 又可能在尚未 await JoinHandle 時繼續進展，不能把「handle 尚未 await」當作所有底層工作都沒動。下一節把手動 release 換成受控外部事件，處理登記與通知之間的同步。
- L209（checked）`下一節` → 16-04：[下一節：外部事件 Future](16-04-lesson.md)

## docs/chapters/16-04-lesson.md

全文實讀1–226；SHA-256 `812dd287942e8bdfe9835c8dafe5f0039b76bd2058c0c7c2a12937b411faa1d1`；引用位置17、mentions 18。

- L3（checked）`上一節` → 16-03：上一節 main 直接 release，再自行 poll。現在仍取得80，數值卻由另一個 worker 供應。main 保有 future，worker 保有一次供值 handle；不論誰先執行，main 都應拿到結果。
- L5（checked）`下一節` → 16-05：新問題是：**main 已看到「還沒結果」，事件卻在登記通知前到達，之後還有人叫它再檢查嗎？** 本節用 channel 與 join 控制兩種先後，不用 sleep 猜排程；通知怎樣接到 executor，下一節才處理。
- L9（checked）`13.6` → 13-06：worker 不持有 future，也不取得它的 mutable reference。沿13.6，兩者以 **`Arc<Mutex<State>>`** 共同保活同一份 State，再用 guard 操作。
- L15（checked）`下圖` → 16-04：下圖藍箭頭表示兩份 Arc 共同保活**同一個 Mutex**，沒有複製兩份 State；polls／completed 留在 main，不是共享欄位：
- L40（checked）`上一節` → 16-03：EventAmount 的 `is_completed()`／`poll_count()` 延續上一節，shared 借用後回傳 bool／usize 快照，不等待或取得 State 鎖：
- L46（checked）`稍後` → 16-04：自訂 **`event.send(value)`** 接收 i32、交回 unit，**取得整份 AmountEvent ownership**，所以同一 handle 只能供一次。它與 std channel 的 Sender::send 不同：前者把答案存進 State，後者稍後只用來把控制數值交給 worker。
- L72（checked）`上一節` → 16-03：沿上一節的 Option::take，事件端從 waiting 取出 owned handle，原槽成 None。內層 block 以這份 Option 作末尾值，交給外面的 waiting；guard 自己留在 block 裡，結尾 Drop 解鎖，之後才 wake：
- L78（checked）`13.6` → 13-06：通知回呼由 driver 提供，可能再取得 State 鎖。若持同一鎖去叫它，就可能回到13.6的等待循環：回呼等解鎖，持鎖端卻等回呼返回。先存值、取 handle、解鎖，再通知，接到通知的使用端也才能取得鎖觀察狀態。
- L88（checked）`如下、上一節` → 16-04、16-03：完整 Future 實作如下；Ready 後再次 poll 仍採上一節的明確 panic 政策，safe 呼叫不能因此允許 UB：
- L100（checked）`上一節` → 16-03：work 留在 main；State 的 i32／Waker 有 Send，Mutex 因而有 Send／Sync，Arc 能交給 worker。這些 owned 資料沒有短局部借用，符合普通 spawn 的 static bound。main 仍用上一節 WakeCounter，通知只計數，不安排 poll。
- L102（checked）`以下` → 16-04：以下是這份 target 實際載入的完整觀察器與完整 readiness module：
- L184（checked）`6.3` → 06-03：回呼先鎖 State、讀答案副本、解鎖，再鎖自己的觀察欄位存副本，不呼叫 poll。沿6.3，i32 有 Copy，`Option<i32>` 也有 Copy，所以讀取沒有取走 future 的答案。測試子 module 可讀父 module 私有 State；沒有把它公開給一般 caller。
- L186（checked）`下面` → 16-04：下面是完整快照，包含全部 imports、helper、四 tests 及 root：
- L206（checked）`前面` → 16-04：回呼實際讀到0，worker join 後，main 再 poll 仍取得0。這驗本實作的關係，沒有用一次通過推得所有同步都安全；一般理由仍是前面的同 guard 操作與鎖外回呼。
- L220（checked）`13.5` → 13-05：得到 `E0382`：第一次 send 接手 event，第二次不能再使用。這是自訂按值介面的政策，不是 std Sender 的限制；13.5 的 Sender::send 借用傳送端，仍可送下一筆。
- L222（checked）`下一節` → 16-05：本節 main 仍自行安排 join 與再 poll。丟棄 future 不會強制停止外部 thread，因此我們保留 handle 並 join；下一節把通知接到有限 executor 的再次檢查。
- L226（checked）`下一節` → 16-05：[下一節：小型 executor](16-05-lesson.md)

## docs/chapters/16-05-lesson.md

全文實讀1–382；SHA-256 `682ee2b551cda73f3589643ce3f9827570e368d10377759af42a9182fd0755e7`；引用位置19、mentions 28。

- L3（checked）`上一節` → 16-04：上一節已有外部事件 Future，main 卻仍先 join，再自行 poll。現在把「何時重新檢查」交給一個小型 **executor（執行器）**：它接手一份工作，先 poll；Pending 時等通知，收到後再 poll；Ready 時交回答案並停止。
- L5（checked）`第14.2` → 14-02：事件仍供80，同鎖登記規則不變。這次改的是 driver 的安排，不是 State 如何保存結果。第14.2節頂層 future 需要被推進的責任，現在由這個有限同步流程承擔。
- L11（checked）`13.5` → 13-05：沿13.5，**`mpsc::channel::<()>()`** 回傳 `Sender<()>`／`Receiver<()>`，buffer 保留尚未取走的訊息。wake 就算先於 recv，通知也能留待收取；空 buffer 且 Sender 仍存在時，recv 會 blocking 等訊息，不是一直重新 poll。
- L13（checked）`16.3` → 16-03：**ChannelWake** 回呼持有一份 `Sender<()>`，wake 只送 unit，不持有 future、不 poll。unit 有 Send，`Sender<()>` 有 Send／Sync，也沒有短局部借用，因此滿足16.3的 Waker::from 所需 bounds；不是要求 future 也移給 worker。
- L17（checked）`如下` → 16-05：早期仍用無 library 的 `ch16-future-stages`，完整設定與預設入口如下：
- L29（checked）`5.1、6.4` → 05-01、06-04：自訂 **`run_one<F: Future>(work: F) -> F::Output`** 沿5.1的泛型及6.4的 associated type：F 是這次呼叫的具體工作型別，bound 讓 driver 能呼叫 poll；`F::Output` 是該實作選定的結果型別。它接手 owned F，正常完成時交回 Output。
- L41（checked）`16.4` → 16-04：`StartAmount::new(work, start, value)` 接手已教的 EventAmount、控制 `Sender<i32>` 及數字。它只保存資料，不啟動 worker 或傳送。poll 先用同一 Context 檢查內層工作；首次 Pending 表示已完成16.4的通知登記，這時才 take 控制 Sender、send 數字，再回 Pending。start 成 None，後續不重送；內層 Ready 就交回相同數字。
- L47（checked）`下面、後面` → 16-05：下面是第一份 target 的完整本地 helpers。runner 只有通知回呼、run_one 與 StartAmount，尚未載後面的對照型別：
- L87（checked）`16.3` → 16-03：**Recheck** 只有 i32 與 bool，具備 Unpin。`new(value)` 保存數字，requested 起始 false；第一次 poll 先設 true，呼叫16.3的 **`cx.waker().wake_by_ref()`**，再回 Pending；下一次才 Ready。
- L163（checked）`第14章` → 14-02：它沒有一般取消／逾時、Tokio I/O 或 timer 支援，也沒有 thread pool。第14章選用 Tokio 的程式，仍需要相應 runtime 環境，不能任意換成這份 std-only driver。
- L167（checked）`前面、下面` → 16-05：現在地址模型、Pin、通知、事件與 driver 都已教完，可以使用原 **`ch13-futures`** package。前面的 stage 快照永久保留；下面改由完整 library 提供相同介面，名稱中的 ch13 是既有 package 名，不是本章號。
- L169（checked）`如下、後段` → 16-05：完整 manifest 與 library root 如下。root 一次宣告五個 module，因此到這裡才讓它參與編譯，不能只靠選早期 example 或篩測試名略過後段：
- L179（checked）`下方` → 16-05：五份完整 module 含原有18 tests：地址4、Pin3、通知4、事件4、executor3。最後三項分別核對登記後的外部通知、poll 內通知不丟失、沒有 Unpin 的工作；全部 helpers 與其 imports 都在下方，worker 仍 join。
- L209（checked）`前面` → 16-05：這些是前面三個對照與完整答案使用 library 的版本，沒有額外 helper：
- L236（checked）`前四節、以下` → 16-01、16-02、16-03、16-04、16-05：結果仍分別為80、120、0與練習120。其餘12個原入口也保留，行為對應前四節快照；以下收完整檔案，切換 package 時可核對實際路徑：
- L290（checked）`第12.1、下面` → 12-01、16-05：下面獨立 fixture manifest 以 path dependency 連完整 library，`[workspace]` 表示自己的 workspace 根；第12.1節的設定分工不變。它只放故意不能編譯的 examples，與正常 package 分開。完整設定、兩個程式及 sidecars：
- L319（checked）`16.2、16.4` → 16-02、16-04：兩條預期失敗，分別含 E0277、E0382。它們使用真正 library 的 public 介面，驗16.2的 mutable 取得限制及16.4的一次供值 ownership，沒有執行錯誤交換。
- L343（checked）`16.3、如下` → 16-03、16-05：最後保留同一 library 的故意失敗 integration test。完整 manifest、測試與診斷字串如下，政策仍是16.3已教的 panic：
- L359（checked）`1.3、12.1` → 01-03、12-01：這兩份獨立 fixture workspace 的 Cargo.lock 都由 Cargo 生成，保存解析結果；沿1.3／12.1的角色，不手動編輯。完整生成檔供對照：

