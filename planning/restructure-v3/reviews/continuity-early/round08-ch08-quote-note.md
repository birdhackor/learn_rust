# 第8章引文精度補記

Initial concept_checks.source_quote contains condensed own-word paraphrases presented in quotation marks, not consistently verbatim. Use this supplement for exact source quotations and original fields as paraphrase. C8-01 quoted table heading was already verbatim. Positions/understanding/findings unchanged; original files not rewritten.

初判封存後自查引文，保存此補記，未回寫原報。下列直接來自原始凍結副本，未讀他角結論、未使用後章補洞。

## Result資料與panic政策

docs/chapters/08-01-lesson.md:3

> 第 5.2 節已用 Result 保存除數零的失敗。現在要檢查批次件數：**0 到 100 都合法**，負數與超過上限各有不同原因。呼叫者收到幾筆資料，就要逐筆報告，不能因第一筆不合法而略過其餘資料。

docs/chapters/08-01-lesson.md:5

> 同一個驗證函式也可能用在另一種情況：內部流程已保證輸入合法，若這項保證被破壞，就停止正常工作。本節先完成逐筆報告，再看明確選擇停止的方式。

docs/chapters/08-01-lesson.md:13

> `checked_count` 接收 i32，交回 `Result<i32, CountError>`：合法時把原值放進 Ok，否則把原因放進 Err。建立 Err 本身不會顯示訊息、觸發 panic 或停止程式：

## unwrap與expect

docs/chapters/08-01-lesson.md:62

> **`unwrap()`** 與 **`expect(message)`** 是 Result、Option 提供的方法。它們以 `self` 接收整份結果，成功時交出裡面的值；若該結果型別不是 Copy，呼叫會消耗它，不能再用原名稱。`message` 是你提供的 `&str` 說明。

docs/chapters/08-01-lesson.md:64

> 標準 `Result<T, E>` 只有在 T、E **兩者都是 Copy** 時才有 Copy。本頁的 CountError 沒有 Copy，因此 `Result<i32, CountError>` 會 move 進這些方法；裡面的成功數字是 Copy，不代表整份結果也能 Copy。

docs/chapters/08-01-lesson.md:73

> Result 的兩個方法都要求錯誤型別有 **Debug**，即使這次預期只會取得 Ok，也要符合這項型別契約。CountError 已提供 Debug；Option 的 None 則沒有錯誤值需要顯示。

## Result比較與測試

docs/chapters/08-01-lesson.md:132

> 第 6.2 節已比較整份 Option。標準 **`Result<T, E>`** 在 T 與 E **兩者都有 PartialEq** 時，也能比較：Ok 與 Err 不相等；同為 Ok 比成功值，同為 Err 比錯誤值。T 與 E 兩者都有 Debug 時，整份 Result 也能 Debug 顯示。

docs/chapters/08-01-lesson.md:134

> 所以這裡的 `Result<i32, CountError>` 能使用 `assert_eq!`：PartialEq 判斷結果相等，Debug 供 assertion 失敗時顯示差異。比較借讀左右兩邊，沿用第 6.2 節的 assertion 契約。

## type alias

docs/chapters/08-02-lesson.md:3

> 上一節的驗證函式交回 `Result<i32, CountError>`。同樣型別若出現在數個函式簽名，重複寫完整名稱會把每個函式的工作擠到後面。現在希望替這個已知結果型別取一個短名稱，保留原有成功、失敗與 ownership 規則。

docs/chapters/08-02-lesson.md:7

> **type alias**（型別別名）是既有型別的另一個名稱。`type CountResult = Result<i32, CountError>;` 讓 CountResult 表示右邊那個型別；`type` 宣告型別名稱，不是在執行時建立一份值。

docs/chapters/08-02-lesson.md:21

> `short_name` 使用 CountResult，`original_name` 使用完整的 `Result<i32, CountError>`，可以直接把前者交給後者，不需轉換。這份結果沒有 Copy，指定會 move；後面只使用接收者 original_name，沒有因取別名而多複製一份結果：

## Display/Formatter/fmt::Result

docs/chapters/08-03-lesson.md:3

> 第 8.1 節用 Debug 顯示 `Negative(-1)`，也讓呼叫者用 match 寫出拒絕原因。若數個呼叫者都要給使用者相同的說明，可以把這項文字規則放在型別上，避免每個地方重寫一次。

docs/chapters/08-03-lesson.md:17

> **`Formatter`** 保存一次格式化操作所需的狀態，包括文字寫入的出口與格式設定。Rust 的格式化流程會提供這份狀態，不需要我們手動建立。

docs/chapters/08-03-lesson.md:28

> **`fmt::Result`** 是 `Result<(), fmt::Error>` 的 type alias，沿用第 8.2 節：它沒有建立另一種結果容器。`Ok(())` 表示文字寫入完成，沒有另外交回數字；`fmt::Error` 則代表寫入這次格式化出口時失敗。

docs/chapters/08-03-lesson.md:30

> CountError 保存的「件數不合法」與這項寫入失敗不同。顯示 Negative 本來就應成功產生「不能為負數」的句子，不能因為它是業務錯誤，就回傳 fmt::Error。Display::fmt 只在 Formatter 的寫入回報失敗時傳回該失敗。

## write!、format!與借用顯示

docs/chapters/08-03-lesson.md:34

> **`write!`** 接收「寫入對象、格式字串、插入值」，把文字交給指定對象。對這裡的 Formatter，它交回 fmt::Result；它不固定向終端機輸出，也不自動加換行。

docs/chapters/08-03-lesson.md:42

> **`format!`** 使用相同的格式字串規則，產生一份 owned String。它不直接顯示到終端機，也不自動加換行。和 println 一樣，格式化會借讀插入的值；產生的新 String 則可獨立保存。

docs/chapters/08-03-lesson.md:48

> main 先用 Debug 檢查，再用 Display 顯示，最後用 format 保存完整訊息。這些操作只借讀 error，所以同一份值能依序使用：

## From/Into與i64

docs/chapters/08-04-lesson.md:11

> **`From::from`** 以值接收來源，直接交回目的型別，不回傳 Result。它的介面約定是一定成功、保留有意義的原資訊，而且轉換意義明確；可能失敗的驗證不能用 panic 假裝成這種轉換。

docs/chapters/08-04-lesson.md:15

> **`Into`** 從來源這一側表達轉換：`Into<i64>` 的型別參數是目的型別，`count.into()` 交出目的值。當目的型別有 `From<來源>`，標準函式庫就提供來源的 `Into<目的>`，不必再寫一份相同邏輯。這是由 From 得到 Into，不能反過來只因看見某份 Into 就假定一定有 From。

docs/chapters/08-04-lesson.md:17

> `.into()` 的方法名本身沒有指明目標，下面用 `let by_into: i64` 提供型別線索。兩種方式都以值接收來源；i32 是 Copy，所以原 count 仍可讀。

## 私有Count、TryFrom與取回值

docs/chapters/08-04-lesson.md:3

> checked_count 雖然檢查 0 到 100，成功後卻仍交回普通 i32。另一個函式只看見 i32，無法從型別辨認它是否已通過檢查。這次建立 Count：只有符合規則的數字才能由公開操作建立這個型別，接收 Count 的工作便能使用已驗證件數。

docs/chapters/08-04-lesson.md:37

> Count 的 value 仍是 i32。第 3.1 節分開型別與欄位的 pub；這次只公開 struct，**欄位沒有 pub**，因此 conversions module 外的呼叫端不能直接寫 `Count { value: -1 }`。公開的 `value(&self)` 只借讀並交回數字，沒有開放修改欄位。

docs/chapters/08-04-lesson.md:47

> 普通 i32 可能是 −1 或 101，轉成 Count 不能保證成功。**`TryFrom<i32>`** 因此要求 `try_from` 以值接收 i32，交回 `Result<Self, Self::Error>`：成功是目的型別，失敗則是這份實作選定的錯誤型別。

docs/chapters/08-04-lesson.md:67

> `i32::from(count)` 與目標為 i32 的 `count.into()` 都接手 Count。若只想看數字並保留它，就呼叫 `count.value()` 借讀。兩種操作取得相同數字，對 ownership 的作用卻不同。

docs/chapters/08-04-lesson.md:90

> 0、3、100 先得到 Count，再取回同一個數字。−1 與 101 根本沒有得到 Count；呼叫者回報失敗後仍繼續。這項有效性保證來自公開介面：建立時檢查，公開方法不提供破壞規則的修改。同一個 conversions module 仍可存取私有欄位，實作者新增操作時也必須維持規則。

## FromStr與parse

docs/chapters/08-05-lesson.md:3

> 第 8.4 節的來源已經是 i32。現在輸入換成 `"3"`、`"abc"`、`"101"`；要取得 Count，必須先把文字解析成整數，再檢查0到100的件數規則。兩個步驟的失敗要保留不同原因。

docs/chapters/08-05-lesson.md:9

> **`FromStr`** 是標準的解析 trait，名稱中的 str 指文字。它要求 `from_str(text: &str) -> Result<Self, Self::Err>`：借讀文字，成功時交出目的型別的值，失敗時交出實作選定的錯誤。Err 是第 6.4 節的 associated type，與 TryFrom 的 Error 同樣由 impl 指定，只是名稱不同。

docs/chapters/08-05-lesson.md:11

> 標準 i32 已實作 FromStr。本例使用十進位整數文字，至少需要一位數字，可在前面帶 `+` 或 `-`；不能有前後空白、其他字元，數字也必須能由 i32 表示。因此 `"3"`、`"+3"` 成功；空字串、`"abc"`、`"3 "`、超過 i32 上限的 `"2147483648"` 都失敗。

docs/chapters/08-05-lesson.md:69

> Count 只有 owned 的 i32，沒有 reference 欄位；此實作的成功結果不借用輸入文字。文字在這裡只用來取得數字，沒有把某份輸出reference的 lifetime 連回 text。

## ParseCountError與From封裝

docs/chapters/08-05-lesson.md:45

> 兩者都保存原錯誤值，讓呼叫者能用 pattern 分類，不是事後猜測訊息文字。Display 在原原因前加上步驟說明；impl 裡的 Self 代表 ParseCountError，所以 `Self::Integer` 就是其 Integer variant。

docs/chapters/08-05-lesson.md:53

> 第 8.4 節的 From 也能封裝錯誤。把 ParseIntError、CountError 放進對應 variant，一定能保留原原因，不需新的驗證。下面兩份實作以值接收原錯誤，把其 ownership 交給外層錯誤：

docs/chapters/08-05-lesson.md:63

> `type Err = ParseCountError;` 選定解析失敗的型別，所以這裡的回傳型別實際是 `Result<Count, ParseCountError>`。先解析 i32，失敗就以 ParseCountError::from 封裝並依第 2.2 節早退；成功後才呼叫 Count::try_from 驗證件數。

## AsRef與Borrow差別

docs/chapters/08-05-lesson.md:95

> 若只想知道文字的 byte 長度，就不需要先解析成數字。**`AsRef<T>`** 描述便宜、一定成功的借用式轉換；`as_ref(&self)` 借讀接收者，交回 `&T`。`AsRef<str>` 因而交回 &str，不建立新 String。

docs/chapters/08-05-lesson.md:101

> AsRef 也能只投影一個欄位。下面 Label 的完整身分由 id、text 兩欄決定；PartialEq／Eq／Hash 沿用第 6.6 節的 derive 規則。它的 `AsRef<str>` 只借出 text。`as_ref(&self) -> &str` 的輸出依第 7.4 節 receiver elision 連到這次 self 借用，因此結果使用時仍須能借讀 Label。

docs/chapters/08-05-lesson.md:117

> 兩個 Label 的 id 是1與2，所以完整值不同；借出的 Rust 文字卻相等。這正是 AsRef 與第 6.5 節 `Borrow<str>` 的差別：Borrow 作鍵查詢時要求借用表示保留 Eq／Hash 的同一個身分。只借 text 會把這兩個不同 Label 當成同一個文字鍵，因此不能把此欄位投影當成符合那項承諾的 Borrow 實作。

## ToString/ToOwned

docs/chapters/08-05-lesson.md:121

> **`ToString`** 的 `to_string(&self)` 借讀值，交回owned String。標準函式庫替有Display的型別提供它，不必自己實作ToString。這與 `format!("{error}")` 一樣保存其Display文字，單一值不必再寫格式字串。

docs/chapters/08-05-lesson.md:123

> **`ToOwned`** 從借來的資料建立owned產物。associated type **`Owned`** 指定產物型別；本節只使用以下兩份標準契約：

docs/chapters/08-05-lesson.md:130

> 這兩種產物不再借用原資料，可獨立保存、修改；原資料也不因此失去ownership。第 6.3 節的Clone則從借讀T取得另一個T，行為仍依型別而定。只複製&str的reference資訊，仍借原文字，與此處的str→String不同。

docs/chapters/08-05-lesson.md:149

> 兩份新增產物與來源可各自使用，不只是多了一個reference。這些具體契約也沒有承諾任意型別的to_owned都會移除其內含借用；要看產物實際保存什麼。

## Result的?及From

docs/chapters/08-06-lesson.md:3

> 第 8.5 節把文字解析分成「取得整數」與「驗證件數」。任何一步失敗，都把具體原因交給呼叫者。現在縮短這項重複的錯誤傳回工作，同時保留兩件事：第一個失敗發生後不做後面的步驟，呼叫者仍可選擇報告後繼續下一筆。

docs/chapters/08-06-lesson.md:17

> **`?`** 是放在結果後面的運算子。對這裡的 Result，它做兩種工作：

docs/chapters/08-06-lesson.md:22

> | `Err(error)` | 以 From 轉成目前函式的錯誤型別，將 Err 提早回傳 | 不執行此函式餘下步驟，回到呼叫者 |

docs/chapters/08-06-lesson.md:24

> 因此使用前要先讀**目前函式的回傳型別**，並確認來源錯誤能轉成它需要的型別。上一節已實作兩份 From：ParseIntError 放進 Integer，CountError 放進 Range；所以這裡能回傳同一種 ParseCountError。

docs/chapters/08-06-lesson.md:38

> 縮寫減少的是「遇到錯誤就轉換並回傳」的重複match，沒有改錯誤分類，也沒有自動列印訊息或呼叫panic。

## Option的?

docs/chapters/08-06-lesson.md:84

> `?`也可用於這種回傳Option的函式：Some時取出內容，None時從目前函式提早回傳None。這裡沒有Err或錯誤轉換；函式簽名必須允許這項缺值回傳。

docs/chapters/08-06-lesson.md:86

> `values.get(1)`的結果是`Option<&i32>`。Some時?交出reference，`*value`讀取數字，再用Some交回owned i32；None時直接交回None，不執行後面的Some：

docs/chapters/08-06-lesson.md:108

> 零筆、一筆都沒有索引1，兩筆才取得20。None沒有附帶原因，也不會因為外層想要Result，就自動轉成任意Err。若需要某種錯誤原因，先明確match缺值並建立那個錯誤。

