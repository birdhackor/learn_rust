# 5.3 char 與 UTF-8：安全取得一段文字

第 3.8 節已能建立 String，並透過 &str 借讀整段內容。現在想從 `A中🦀` 只取出「中」。數值 slice 的區間按元素編號；UTF-8 文字的區間則按 byte 位置，還必須保持有效的文字分界。

## 先分清 byte、scalar 與字形

第 1.4 節的 **char** 保存一個 **Unicode scalar（純量值）**，也就是 Unicode 中可用來表示文字的一個基本數值。char literal 用單引號，例如 `'中'`；雙引號的 `"中"` 是一段文字的 &str，兩者型別不同。

**UTF-8** 將一個 scalar 編成 1 到 4 個 bytes。這是文字內容的編碼規則，不是說 Rust 的 char 型別本身占 1 到 4 bytes。**str 的 len() 回傳 UTF-8 的 byte 數量，型別為 usize**，不直接回答有幾個 scalar。

人眼看到的一個完整字形，也可能由多個 scalar 一起形成，例如字母 e 與結合重音符號。因此本節的「文字分界」指 UTF-8 scalar 的完整分界，沒有保證剛好是一個完整顯示字形或一個語言中的字。

`A中🦀` 的編碼資料如下。byte 位置從 0 開始；這些長度是本例各 scalar 的資料，不是所有中文或符號一律相同的長度：

| 內容 | byte 值 | byte 位置 | 完整區間 |
| --- | --- | --- | --- |
| A | 65 | 0 | `0..1` |
| 中 | 228、184、173 | 1、2、3 | `1..4` |
| 🦀 | 240、159、166、128 | 4、5、6、7 | `4..8` |

整段有 8 bytes，合法分界是 0、1、4、8。range 沿用第 3.7 節的「包含起點、不含終點」；圖中的 crab 對應第三項 🦀：

![A、中、crab的完整區間分別為0..1、1..4、4..8；1..2只留下中的第一個byte](../assets/ch03/utf8-boundaries.svg)

`1..4` 涵蓋「中」的全部三個 bytes。`1..2` 雖然仍在總長度內，卻切斷它的編碼，不能成為一段有效的 str。

## str 的 get：成功借讀，失敗回傳 None

**`text.get(start..end)`** 在接收者為 &str 時，按 byte range 取得文字。回傳型別是 **`Option<&str>`**：成功時 Some 保存借來的文字視圖；失敗時回傳 None，不為這項範圍失敗而 panic，也不另建 String。

合法 range 必須同時滿足：起點不大於終點、兩端都不超過 byte 長度、兩端都在 UTF-8 scalar 分界。空 range 也要符合分界：`8..8` 在結尾，得到有值的空文字；`2..2` 位於「中」內部，仍是 None。

下面函式收到 &str 與兩個 usize。match 的 Some(part) 取得 &str，借讀原文字；None 分支顯示沒有取得有效區段。它只顯示，不把借來的文字交回外層：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/utf8.rs:show-range"
```

對 owned String，先用已教的 as_str() 取得 &str，再使用文字方法。例如 `text.as_str().len()` 先借內容，再查 byte 長度；`text.as_str().get(1..4)` 則先借內容，再選區段。借來的文字仍受第 3.5 節規則約束，使用時原資料必須有效，不能同時衝突地修改。

## 完整程式與執行

本頁使用沒有 library 的 intro-data-steps。main 用單引號建立 char，也用 String 保存整段內容；char 沿用第一章的文字顯示方式：

```toml
--8<-- "examples/intro-workspace/data-steps/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/data-steps/examples/utf8.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data-steps --example utf8 --locked
```

```text
一個 char=中
UTF-8 長度=8 bytes
0..1：有文字「A」
1..4：有文字「中」
4..8：有文字「🦀」
1..2：不是有效文字區段
8..8：有文字「」
4..9：不是有效文字區段
```

1..2 切斷 scalar，4..9 超過總長度，兩者都沒有取得文字。Option 本身沒有保存是哪一種範圍錯誤；合法空段則仍是 Some，只是其中的 &str 長度為零。

## 直接 range 索引的失敗契約不同

也能以 `&text[start..end]` 借讀一段 str，但它要求相同的合法 range；不合法時會 panic。單一整數索引不能用來取得「第幾個 char」，也不是取得一個 byte 的文字接口。

下面是完整的預期執行失敗例。它先借用 String 的 &str，再直接索引 `1..2`：

```rust
--8<-- "examples/intro-workspace/data-steps/fixtures/runtime-fail/utf8_boundary.rs"
```

放入獨立練習專案執行，會得到包含 `not a char boundary` 的 panic。get 把相同的範圍失敗保存為 None，讓呼叫者決定下一步；直接索引則不提供這份結果。

## 練習：交回長度，保留缺值

寫 `selected_byte_len(text: &str, start: usize, end: usize) -> Option<usize>`。先以 get 借到區段；Some 時交回 Some 中的 byte 長度，None 時仍交回 None。交回的是 owned 的 usize 數字，不是文字 reference。

用同一份 A中🦀 檢查：1..4 得到長度 3，8..8 得到長度 0；1..2、2..2 與4..9都沒有取得區段。不能把失敗一律改成 0，否則合法空文字與失敗又混在一起。

答案將函式放在這個 target 自己的 text module，沿用第 2.6 節的 main.rs＋text/mod.rs 規則。完整設定就是本頁 manifest，不載入後續清單程式。

### 先說明保留 match 的工具設定

Clippy 的 **manual_map** lint 會建議把這種 match 改成較短的 Option 操作。本節正在對照 Some 與 None 兩條路，而且較短形式需要後面的新語法，所以刻意保留已教的 match。沿用第 2.7 節，只在 selected_byte_len 前加 `#[allow(clippy::manual_map)]`，不關掉其他檢查。

測試也保留這兩條路：有值時先檢查 usize 長度，再交出 bool 表示確實取得值；None 交出 false，由 assert! 檢查。這樣不會在意外得到 None 時略過長度檢查，卻讓整個測試通過。無效區段的測試則用已知 i32 0/1 表示取得與否，不直接比較整個 Option。

### 完整答案與四項測試

`examples/utf8_answer/text/mod.rs`：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/utf8_answer/text/mod.rs"
```

`examples/utf8_answer/main.rs`：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/utf8_answer/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data-steps --example utf8_answer --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-data-steps --example utf8_answer --locked
```

```text
1..4：取得 3 bytes
1..2：無效區段
8..8：取得 0 bytes
```

四項測試分別驗證中文長度、合法空段、切斷 scalar，以及越界。現在能把 byte 位置與取得結果連起來；下一節的清單索引則按元素計數，須和文字 range 分開。

[下一節：Vec](05-04-lesson.md)
