# 6.2 用 associated type 指定一份實作的輸出型別

4.1 的 Charge 要求每個實作都交出 i32 金額，所以共同函式能直接比較數字。現在換一項需要：把一份資料來源交出去，取回它保存的內容。數字盒交出 i32，開關狀態交出 bool；我們希望共用「取得內容」這項操作，同時保留不同內容的型別。

這次不適合把輸出都寫成 i32，因為 bool 的意思仍應是 true／false；也不需要把每種內容轉成同一段文字。共同的是取出工作，不是結果型別。呼叫者仍要知道自己取得哪種資料，才可以繼續做適合的操作。

## 先保留兩種來源的資料形狀

Holder 沿用 3.3 的泛型資料盒，value 欄位的型別是 T。SwitchState 則有一個名為 enabled 的 bool 欄位；它與數字盒的欄位角色不同。型別與欄位公開的方法沿用 4.1，讓 library 外的範例能建立這些資料：

```rust
--8<-- "examples/ch06-iterators/src/associated.rs:models"
```

`Holder<i32>`、`Holder<String>` 與 SwitchState 都是具體型別。這裡的 Holder 並非只有數字；如果其中保存 String，取出工作還要把文字的 ownership 交回呼叫者。

## trait 裡留下輸出型別的位置

**associated type（關聯型別）** 是 trait 契約中的型別名稱，由每份實作決定它對應的型別。它不是物件中的值欄位，而是讓 method 簽名可以表達「依這份實作的選擇回傳資料」。

本例的 trait 叫 Extract，操作叫 extract。`type Output;` 宣告一個名為 Output 的 associated type，這裡還沒選具體型別。Output 是我們取的名稱，用來說明它的輸出角色，不是 Rust 固定關鍵字。

`Self` 沿用 2.5 的意義：實作此 trait 的型別。**`Self::Output`** 則指這份實作指定的 Output 型別。它與 `self.value` 不同：大寫 Self 的路徑找型別，小寫 self 的欄位存取找某個值中的資料。

`extract(self)` 接手來源，回傳它指定的輸出。選擇 self 是因為我們也要能取出 String 這類 owned 內容，不假設未知的 T 都能 Copy；這延續 2.2 的 move 規則。

```rust
--8<-- "examples/ch06-iterators/src/associated.rs:extract-trait"
```

這份契約要求兩件事：指定 Output 的型別，以及提供符合這個輸出型別的 extract body。單獨宣告 Extract 不會自動為任何資料建立取出方法。

## 在 impl 選定 Output

SwitchState 的選擇很直接：實作中寫 `type Output = bool;`，因此該實作的 `Self::Output` 就是 bool，body 交回 enabled。

Holder 需要同時處理不同 T。**`impl<T>`** 在 impl 開頭宣告型別參數，讓後面的 `Holder<T>` 與 body 都能使用 T；和 3.3 在泛型函式名稱後宣告 T 相同，型別參數要先有宣告，不能只在後面出現。

因此 **`impl<T> Extract for Holder<T>`** 表示為每個符合這個形狀的 `Holder<T>` 實作 Extract。`type Output = T;` 選擇和 value 欄位相同的型別。若 T 是 i32，輸出就是 i32；若 T 是 String，輸出就是 String。這裡沒有要求新的能力，body 只把接手來源中的值交出去。

```rust
--8<-- "examples/ch06-iterators/src/associated.rs:extract-impls"
```

| 來源型別 | 這份 Extract 的 Output | 取出的值舉例 |
| --- | --- | --- |
| `Holder<i32>` | i32 | 120 |
| `Holder<String>` | String | owned 的 Rust 文字 |
| SwitchState | bool | true |

對同一個 `Holder<i32>` 的 Extract 契約來說，Output 已由這份實作選成 i32；呼叫 extract 時不會重新選成 bool。associated type 保留的是這個來源與結果型別的關係，不是把未知型別放進執行期欄位。

### 先直接呼叫，觀察兩種結果

本章 package 名稱是 ch06-iterators；程式路徑的 library 名稱是 ch06_iterators，沿用 1.9 將連字號換成底線的規則。library 把上述內容放在 associated module，沿用 1.9 的 pub mod 分工。用點號呼叫 trait method 時，除了來源型別，也將 Extract 引入名稱範圍，和 4.1 引入 Charge 的方法相同：

```rust
--8<-- "examples/ch06-iterators/examples/associated_direct.rs"
```

```bash
cargo run -p ch06-iterators --example associated_direct --locked
```

```text
盒子交出數字=120
開關狀態交出布林值=true
```

number 的實作交出 i32，switch 的實作交出 bool。兩個來源都已被完整交出，後面只使用取回的 amount 與 enabled。即使 SwitchState 的欄位是 Copy 的 bool，也不表示整個自訂 struct 自動有 Copy 能力。

## 泛型函式把輸出關係帶回呼叫者

現在來源已提供共同操作，可以像 4.1 一樣寫 bound：`S: Extract` 表示來源 S 必須實作 Extract。**`S::Output`** 表示這個型別參數所選實作的輸出型別。這讓函式簽名保留「傳入哪種來源，就取回它自己的輸出」的關係：

```rust
--8<-- "examples/ch06-iterators/src/associated.rs:extract-generic"
```

extract_value 只需要知道來源能提供 extract；它不讀取 value 或 enabled 欄位，也沒有把不同內容強制換成同一型別。來源如何取出值由各實作處理，泛型函式保留共同的轉交操作。

這時不能在泛型 body 任意把結果乘二。S: Extract 只保證有取出操作，沒有保證 S::Output 是數字。若特定工作確實只處理數字，就要把輸出要求也放進契約。

## equality bound 限制的是輸出型別

**associated type equality bound** 以 `Extract<Output = i32>` 表示「除了實作 Extract，它的 Output 必須是 i32」。這裡角括號中的名稱是 trait 宣告的 associated type，等號指定必須相同的型別；不是傳入名為 Output 的資料值，也不是讓呼叫者替現有實作改選型別。

`S: Extract<Output = i32>` 因而讓函式確定 extract 的結果能使用 i32 的乘法。下面的材料仍採小整數，乘二不超過 i32 範圍：

```rust
--8<-- "examples/ch06-iterators/src/associated.rs:numeric-only"
```

把通用取出與數字工作放在同一個呼叫程式觀察：

```rust
--8<-- "examples/ch06-iterators/examples/associated_types.rs"
```

```bash
cargo run -p ch06-iterators --example associated_types --locked
```

```text
同一函式取得數字=120、布林值=true
數字結果乘二=240
```

前兩次使用同一個 extract_value，取得各自型別的答案；第三次則使用限制輸出為 i32 的 double_number。不是所有符合 Extract 的來源都符合這項數字要求。

下面故意把 SwitchState 交給 double_number。它確實實作 Extract，但 Output 是 bool，因此不能編譯：

```rust
--8<-- "examples/ch06-iterators/fixtures/compile-fail/output_bound.rs"
```

錯誤碼 `E0271` 指出 associated type 不符合相等要求，預期 i32，實際為 bool。修正要依工作需要：這是數字工作就提供數字來源；這是通用取出工作就使用沒有數字限制的函式。不能把 bound 改掉，卻仍假定所有結果能乘二。

## 用測試分別核對型別與值

型別註記 i32／bool 及上述故意失敗範例，讓編譯器核對契約。接著仍要用測試檢查取出與數字處理是否交回正確值；trait 本身不會證明商業邏輯。

下面還包含一份 String 來源。文字從 String 變數 move 到 Holder，再由 extract_value 交回；原 Holder 已消耗，取回的文字仍能追加並繼續使用。追加後的 `Rust 教學` 有 11 bytes，用 3.5 已教的 as_str／len 核對，不要求尚未引入的整份自訂資料比較。

```rust
--8<-- "examples/ch06-iterators/src/associated.rs:associated-tests"
```

```bash
cargo test -p ch06-iterators --locked
```

本節有三項測試，分別驗證來源與輸出值、數字限制下的計算、owned 文字交回後仍能使用。它們不代表所有來源都應採 self 接手；這次取出 owned 內容的需要才是該 receiver 選擇的理由。

## 練習：從兩欄來源取出第一欄

建立 `FirstPair<T, U>`，包含 first 與 second 兩個欄位。為它實作 Extract：Output 選 T，extract 接手整份來源並交出 first；second 沒有交回，隨這次來源結束。

先保存 first 120、second true，在取出前顯示第二欄，再使用既有 extract_value，確認第一欄結果為 i32 的 120。另建立 first 80、second false 的來源，交給既有 double_number，確認得到 160。不要修改兩個共用函式，也不要在泛型實作中假定 T 必定能做數值運算。

完成後可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch06-iterators/examples/associated_solution.rs)，或執行 `cargo run -p ch06-iterators --example associated_solution --locked`。

你現在能分辨型別參數的來源、實作選定的 associated type，以及只接受特定輸出的 bound。接下來若一項介面同時要說明輸入與輸出，就可以讀懂其中的輸出型別關係，而不只把 method 名稱視為黑盒。

延伸閱讀：[Rust Book：Associated Types](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#defining-traits-with-associated-types)、[簡體 Rust Book：關聯型別](https://kaisery.github.io/trpl-zh-cn/ch20-02-advanced-traits.html#关联类型)。這些段落用後面才會教的標準介面作例子，本節先採資料來源，必要解釋都在正文。
