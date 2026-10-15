# 11.1 型別需要多大空間：Sized、DST 與常數泛型

第 3.7 節用 slice 借不同長度的資料，第 3.8 節用 `&str` 借文字。現在比較兩件事：三項 i32 array 本身需要 12 bytes；同一個 `&str` 型別卻可以借 5 bytes 的 Rust!，也可以借 13 bytes 的 Rust tutorial。型別大小與這次借到的資料大小，究竟在哪裡分開？

## Sized 說明型別本身的大小

`i32` 使用 4 bytes，`[i32; 3]` 使用 12 bytes。array 改成 `[10, 20, 30]` 或 `[80, 120, 200]`，都不改變這個型別需要的空間；把長度改成 2，則換成另一個型別 `[i32; 2]`。

**`Sized`** 是標準 trait，表示這個型別的大小在編譯時可確定。編譯器依型別判斷它，不需要我們寫方法或自行實作。一般區域變數與按值傳入的參數，需要這種已知大小的表示。Sized 由第 5.2 節的 prelude 提供，不用另外 use。

先用標準函式 **`std::mem::size_of::<T>()`** 查詢：它不接收值，要求 T 是 Sized，回傳 usize，單位是 bytes。`std::mem` 是標準函式庫處理記憶體相關工作的 module；這裡只查大小。`::<T>` 沿用第 5.1 節的 turbofish，明選要查的型別。

泛型參數 `<T>` 預設包含 `T: Sized`，所以以下 fixed_bytes 能把 T 交給 size_of：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/size_support/mod.rs:fixed-bytes"
```

例如 `fixed_bytes::<i32>()` 得到 4，`fixed_bytes::<[i32; 3]>()` 得到 12。查的是型別，不需要先建立一筆特定內容。

## DST 的資料大小要看實際借到的內容

**DST** 是 **dynamically sized type**，也就是大小不能只由型別在編譯時確定的型別。第 3 章的 str 與 `[i32]` 都是例子：str 可以有不同的 byte 長度，`[i32]` 可以有不同的項目數。

它們要與 `&str`、`&[i32]` 分開。reference 自身保存定位資料需要的資訊；對這兩種 slice，資訊包含位置與長度。這份表示有固定大小，所借的內容卻可以長短不同：

| 這次使用的型別 | 型別本身是 Sized 嗎？ | 本例要觀察的大小 |
| --- | --- | --- |
| `[i32; 3]` | 是 | 三個 i32 共 12 bytes。 |
| `[i32]` | 否 | 借兩項時是 8 bytes，借三項時是 12。 |
| `&[i32]` | 是 | reference 自身固定，與借了幾項分開。 |
| str | 否 | Rust! 是 5 bytes，Rust tutorial 是 13。 |
| `&str` | 是 | reference 自身固定，與文字多長分開。 |

標準函式 **`std::mem::size_of_val(value)`** 接收一份 `&T`，回傳被借資料的 bytes；它也接受 DST。要把這種能力放進泛型函式，需明寫 **`T: ?Sized`**，解除預設的 Sized 要求：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/size_support/mod.rs:borrowed-bytes"
```

`?Sized` 表示 T 可以是 Sized，也可以不是；並非要求 T 一定是 DST。參數使用 `&T`，所以函式收到的 reference 自身仍有固定大小；它沒有接手原資料，只交回一個獨立的 usize。

## 完整例子：分開查型別與資料

本節使用無 library 的 `intro-type-stages` package。所有設定如下，每個 example 只載入自己的 helper：

```toml
--8<-- "examples/intro-workspace/type-stages/Cargo.toml"
```

大小查詢的 helper 明確放在 `examples/size_support/mod.rs`。下面是完整檔案，只有兩個已教函式與一項測試；不載入其他節的型別：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/size_support/mod.rs"
```

完整根檔案 `examples/sizes.rs` 用 `mod size_support;` 載入它。part 借 array 後兩項；word 與 longer_word 都是 `&str`，內容長度不同：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/sizes.rs"
```

從 repo 根目錄執行：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example sizes --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example sizes --locked
```

在本教材驗證的 64-bit 環境，輸出是：

```text
i32 型別的 bytes=4
三項 array 的 bytes=12
slice reference 自身的 bytes=16
借來的兩項資料 bytes=8
str reference 自身的 bytes=16
短文字的資料 bytes=5
長文字的資料 bytes=13
```

16 是這個目標平台的 reference 大小，不是每個平台都必須得到的數字。關鍵是：`fixed_bytes::<&str>()` 查 reference 本身；`borrowed_bytes(word)` 的參數對應 `&T`，此時 T 是 str，查到的才是文字的 5 bytes。長文字換成 13 bytes，reference 型別本身不變。測試也用整段、部分、空文字確認資料大小跟著借用範圍改變。

String、Vec 的內容也能增長，但它們仍是 Sized：它們本身保存管理另一處資料的資訊，這份管理資訊有固定大小。不能從「內容可變長」推論型別一定是 DST，也不從 Sized 推論值一定放在 stack。

## 只加 `&`，不會自動解除泛型要求

下面完整失敗例把 borrowed_bytes 寫成 `<T>`，少了 `?Sized`：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/sized_default.rs"
```

放進第 1.2 節的獨立練習專案執行 `cargo check`，得到 **E0277**。word 的型別是 `&str`，對應參數 `&T` 時，T 是 str；str 不符合這個函式預設的 Sized 要求。只接收 reference，沒有自動改掉泛型參數的 bound。

前面成功版本的 `T: ?Sized` 才表達接受 DST 的需要。完整 `.expected` 如下：

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/sized_default.expected"
```

## 常數泛型：把 array 長度也變成參數

slice 已能讓函式借不同長度的資料。若希望簽名保留**完整 array 型別與固定長度**，又不想為長度 2、3 各寫一份函式，就可以將長度也設成泛型參數。

**常數泛型（const generics）** 讓參數代表編譯時選定的值。`<T, const N: usize>` 中，T 是型別參數，N 是 usize 型別的常數值參數。`[T; N]` 在型別位置使用 N，每次選定 T 與 N 後，仍是一個大小固定的 array 型別。

下面 fixed_len 只借 array，交回它的 N。參數位置的 `_` 沿用第 4 章的忽略 pattern：不替輸入取名稱，但它仍須符合 `&[T; N]`。

```rust
--8<-- "examples/intro-workspace/type-stages/examples/array_support/mod.rs:fixed-len"
```

`fixed_len(&counts)` 可由 array 型別推導 T 與 N。也能寫 `fixed_len::<i32, 3>(&counts)`，依宣告順序明選型別 i32 與常數值 3；第二項不是另一個型別。

N 不會等到執行時才變成不同長度。本例仍以普通函式呼叫回傳、顯示 N；「參數在編譯時選定」不等於「這個普通函式只在編譯時執行」。

完整 helper `examples/array_support/mod.rs` 如下，與 size_support 分開：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/array_support/mod.rs"
```

完整呼叫者沿用本頁 manifest，不需要其他來源：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/const_generics.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example const_generics --locked
```

```text
由 i32 array 推導 N=3
由 bool array 推導 N=2
明選 i32 與常數 3，得到 N=3
只借 array，原資料仍可讀=10
```

兩種元素型別與兩個長度共用同一份定義，原 array 也沒有被交走。slice 統一處理借來的一段；常數泛型則把固定長度保留在這份完整 array 的型別裡。

### 明選的 N 必須符合輸入型別

下面完整失敗例明選 N=3，卻傳入長度 2 的 array：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/const_generic_length.rs"
```

執行 `cargo check` 得到 **E0308**：預期 `&[i32; 3]`，實際是 `&[i32; 2]`。Rust 不會在執行時替它補一項或改選 N。完整 `.expected` 如下：

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/const_generic_length.expected"
```

## 練習與完整答案

寫 `array_bytes<T, const N: usize>`，借完整 array，以 N 乘上 `size_of::<T>()` 回傳它需要的 bytes。array 的大小等於元素大小乘上項目數，因此這個乘法適用。T 仍有預設的 Sized 要求。

驗證兩項 i32 是 8 bytes、三項 bool 是 3 bytes、空 i32 array 是 0；把 i32 array 的第一項由 80 改成 200，大小仍是 8。這裡改內容，沒有改型別或 N。

完整答案沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/array_bytes_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example array_bytes_answer --locked
```

```text
i32 array 修改前後的 bytes=8、8
bool array 的 bytes=3
空 array 的 bytes=0
```

現在能分開固定型別大小、DST 的實際資料大小，以及保留 array 固定長度的參數。下一節用這個區別，處理一個型別又包含自身時的存放問題。

[下一節：Box 與遞迴資料](11-02-lesson.md)
