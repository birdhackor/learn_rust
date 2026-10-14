# 6.3 用 Fn 系列描述共用函式需要的呼叫能力

6.1 已能保存並呼叫 closure。接著希望把「收到一份工作，再按約定執行」整理成共用函式：有的只執行一次，有的重複執行，有的只借用原工作。

這些共用函式不需要知道 closure 使用哪些外部變數，但必須知道它能接受什麼參數、交回什麼，以及用哪種方式呼叫。6.2 的 trait 與 associated type 關係，現在可以用在這份契約。

## 只需要一次呼叫：FnOnce

把其他程式提供、由共用函式負責呼叫的工作稱為 **callback（回呼）**。例如提供一份「交出 owned 文字」的工作，共用函式接手後執行它，取回文字交給呼叫者。

**`FnOnce`** 是 Rust 的可呼叫 trait，提供按值接手這份工作的呼叫能力。closure 的 body 可以在呼叫中移出保存的 non-Copy 內容，像 6.1 的 take。只知道 `F: FnOnce` 時，共用函式不能假定同一份 F 還能再用。

`FnOnce` 與稍後的 `FnMut`、`Fn` 都由 `std::ops` 提供，也已在 Rust 的 prelude 中，因此本節可以直接在 bound 使用這些名稱。這是這三個標準 trait 的名稱來源；自訂 trait 仍沿 4.1 以 `use` 引入。

這裡的 bound 採專用語法 **`FnOnce() -> T`**：空括號表示不接收參數，箭頭表示回傳 T。回傳型別對應這組 trait 的 associated type `Output`，和 6.2 一樣是該實作的輸出選擇；不用手寫標準函式庫內部的呼叫 method。

每個 closure 表達式都有自己的具體型別，通常不能在原始碼中寫出該型別的名稱。因此用泛型 `F` 表示工作型別，再用 trait 說清楚它能做什麼。下面 `T` 是結果型別，`run_once` 接手 task 並呼叫一次，交回取得的 T；沒有保存供後續第二次呼叫的 task。本節 library 新增 `callbacks` module，仍使用 1.9 的 `pub mod` 分工：

```rust
--8<-- "examples/ch06-iterators/src/callbacks.rs:once"
```

```rust
--8<-- "examples/ch06-iterators/examples/callback_once.rs"
```

```bash
cargo run -p ch06-iterators --example callback_once --locked
```

輸出 `共用函式交回文字=Rust`。原 closure 的工作依然是交出它持有的 String，新增的函式只是依這項契約執行，不必知道那份文字怎麼建構。

`FnOnce` 是最低呼叫能力，**不是每個符合它的 closure 都必定只能呼叫一次**。上面的 `|| 7` 沒有移出需管理的資源，也符合契約；共用函式選擇只依承諾使用一次。

## 需要重複呼叫並允許修改：FnMut

如果要讓計數工作連做兩次，只知道 FnOnce 就不夠。**`FnMut`** 提供透過 mutable reference 呼叫自身的能力，可以重複呼叫，並讓 body 修改保存的環境。

`FnMut() -> i32` 表示每次不接收參數、交回一個 i32。本例的 `call_twice` 接手工作後，參數寫 `mut task`，允許借用 task 作兩次呼叫；結果按照第一次、第二次的順序放進 tuple：

```rust
--8<-- "examples/ch06-iterators/src/callbacks.rs:mutable"
```

```rust
--8<-- "examples/ch06-iterators/examples/callback_mutable.rs"
```

```bash
cargo run -p ch06-iterators --example callback_mutable --locked
```

輸出 `兩次結果=1、2，原計數=2`。呼叫程式中的 tick 沒有直接被呼叫，只被交給函式，因此它的 binding 不需要 mut；接手之後要實際呼叫兩次的 task 才需要 mut。

call_twice 結束時沒有交回 closure，保存的 mutable borrow 也不再使用，因此外面可以讀 count。這裡是重複使用同一份計數狀態，不是建立兩份從零開始的工作。

下面故意要求同一份工作回傳兩個 String，但提供的 take 會移出唯一保存的文字。它只能提供 FnOnce 的按值呼叫，不能滿足 FnMut，所以得到 `E0525`：

```rust
--8<-- "examples/ch06-iterators/fixtures/compile-fail/callback_needs_repeat.rs"
```

把兩次呼叫需求改成一次，或重新設計成每次都能產生一份文字，是兩種不同的工作。只替參數加 mut，不能替已移出的資料補出第二份。

## 只有 shared access 也能呼叫：Fn

如果共用函式只收到對工作本身的 shared reference，它不能用 mutable receiver 呼叫。**`Fn`** 提供透過 shared reference 呼叫自身的能力，也能重複呼叫。

本例的加法工作只讀取保存的 base，適合這項要求。**`Fn(i32) -> i32`** 表示呼叫時收到一個 i32，回傳一個 i32；不是宣告名為 i32 的環境欄位。

`apply_shared` 收到 `&F`，保留呼叫者的工作，而不接手它。這裡選 Fn 的理由是只有 shared access；只是「需要呼叫多次」的工作，通常 FnMut 已足夠：

```rust
--8<-- "examples/ch06-iterators/src/callbacks.rs:shared"
```

```rust
--8<-- "examples/ch06-iterators/examples/callback_shared.rs"
```

執行 `cargo run -p ch06-iterators --example callback_shared --locked`，先得到 `13`，保留原工作再呼叫得到 `14`。

三種能力的接收方式，對應 2.5 已教的 receiver 角色：

| trait | 呼叫如何使用這份工作 | 這份契約允許共用函式做什麼 |
| --- | --- | --- |
| `FnOnce` | 接手自身值，類似 self | 能作按值的一次呼叫，不假定原工作仍可用 |
| `FnMut` | mutable borrow，類似 &mut self | 能反覆呼叫並操作可修改的環境 |
| `Fn` | shared borrow，類似 &self | 只有 shared access 時仍能呼叫 |

符合 Fn 的工作也符合 FnMut 與 FnOnce；符合 FnMut 的也符合 FnOnce。因此 call_twice 能接受不修改環境的 `|| 7`，run_once 也能接受前兩種能力的工作。這是可提供能力的包含關係，不是三個互斥分類。

還要保留 6.1 的區分：`move` 只規定按值 capture。擁有 String 但只讀它的 closure 仍可有 Fn；把 String 移出 body 才使這個例子只能有 FnOnce。Fn 描述 shared receiver 能力，不保證每次回傳相同值，也不把所有可能間接可達的資料都宣稱成永遠不變。

## 已有普通函式，可以用 function pointer

有時工作已經是一個普通函式，不需要重新寫一個會捕捉環境的 closure。**function pointer（函式指標）** 是保存可呼叫函式位置的值；型別以小寫 **`fn(i32) -> i32`** 表示，指定輸入與輸出。

`fn` 在這裡是具體型別，`Fn` 則是能力 trait，兩者不同。可以把普通函式名稱指定到這個型別，編譯器會轉成對應的 function pointer。不捕捉環境的 closure 也能作這種轉換；pointer 本身沒有位置保存任意外部環境資料。

這種 function pointer 提供 Fn、FnMut、FnOnce，所以也可交給前面的共用函式。下面先直接呼叫兩個 pointer，再將其中一個借給 apply_shared：

```rust
--8<-- "examples/ch06-iterators/examples/function_pointer.rs"
```

執行 `cargo run -p ch06-iterators --example function_pointer --locked`，三個結果依序是 `4`、`6`、`4`。

若 closure 需要 capture base，就不能只用同樣的 pointer 型別保存它；以下範例得到 `E0308`。這時使用接受 Fn 等 trait 的泛型介面，而不是把環境偷偷塞進 function pointer：

```rust
--8<-- "examples/ch06-iterators/fixtures/compile-fail/function_pointer_capture.rs"
```

## 練習：轉換數列，同時累計處理次數

寫 `sum_with`，收到 numeric slice 與 callback，逐個取得元素交給 callback，把回傳的 i32 相加。callback 可以修改保存的狀態，所以選用這節相應的 bound 與參數 mut；迴圈仍用已教的索引與 while。

輸入 `[1, 2, 3]`、外部 factor 為 `10`，callback 每次把 visits 加一，再回傳元素乘 factor。確認總和 `60`、visits `3`；空 slice 總和為 `0`。所有材料仍採小整數，算術不超出範圍。

可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch06-iterators/examples/callback_solution.rs)，或執行 `cargo run -p ch06-iterators --example callback_solution --locked`。`cargo test -p ch06-iterators --locked` 另檢查 owned 結果、mutable capture 的連續狀態與 shared 呼叫介面。

現在共用函式可以依需要接受 callback，而不替呼叫者決定如何保存環境。下一節反過來讓函式建立並交回工作，處理 closure 型別如何在回傳介面中表達。

延伸閱讀：[Rust Book：Function Pointers](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html#function-pointers)、[簡體對應章](https://kaisery.github.io/trpl-zh-cn/ch20-04-advanced-functions-and-closures.html#函数指针)、[std：FnOnce](https://doc.rust-lang.org/std/ops/trait.FnOnce.html)、[FnMut](https://doc.rust-lang.org/std/ops/trait.FnMut.html)、[Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html)。
