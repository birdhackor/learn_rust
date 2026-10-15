# 11.3 用 Drop 觀察 ownership 的收尾

第 3.2 節已知道，仍由局部變數持有的值會在離開 scope 時結束；第 11.2 節的 Box 也負責自己管理的內容。現在讓自訂資料在清理時印出 id，追蹤先後順序。這只是讓收尾工作可見，沒有自行實作記憶體釋放。

## Drop 提供自訂收尾工作

**destructor** 指值結束時的清理動作。Rust 會處理值已經擁有的欄位；若還需要額外工作，可以實作標準的 **`Drop` trait**。它由 prelude 提供，必要方法是 `drop(&mut self)`：借改即將清理的值，完成自訂工作，回傳 unit。

下面的 Tracked 只保存 i32 id。`impl Drop for Tracked` 沿用第 6.1 節的 trait 實作形式，但這項方法由 Rust 的清理流程呼叫：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/drop_scope.rs:tracked"
```

本節仍使用沒有 library 的 intro-type-stages。完整設定如下：

```toml
--8<-- "examples/intro-workspace/type-stages/Cargo.toml"
```

完整程式先建立外層 id1，再在內層依序建立 id2、id3：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/drop_scope.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example drop_scope --locked
```

```text
建立外層=1
建立內層=2、3
清理 id=3
清理 id=2
內層已收尾，外層仍是=1
main準備結束
清理 id=1
```

正常離開內層 block，局部變數依宣告的反序清理，所以先3、再2。外層 first 仍持有 id1，直到 main 結束才清理。

view 最後讀取之後，還印出了「main準備結束」，才清理 first。**reference 的最後使用與 owner 的清理位置不同**：借用完成，讓後續能做其他合法存取，不表示原值立刻被清理。

## 提早清理，要把 owned 值交出去

標準函式 **`std::mem::drop(value)`** 接收值、回傳 unit。非 Copy 的值 move 進去，在函式返回前完成正常清理，呼叫者不能再使用原值。用 `use std::mem::drop;` 引入後可直接寫 drop；它是普通函式，沒有 `!`。

下面仍只有 Tracked 與 main，先 move 給 second，再顯式清理：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/drop_early.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example drop_early --locked
```

```text
建立資料=1
接手資料=1
清理 id=1
清理完成後，繼續工作
```

first 交給 second 時，值只換了 owner，沒有當場清理。`drop(second)` 才交出並清理這份值；main 結束時不會再清理同一份資料。

對 i32 或 shared reference，傳入 drop 的是 Copy 副本。因此 `drop(&value)` 不會清理原 value；它只收到一份 reference 值。

### trait method 不能由程式直接呼叫

Drop 的方法只收到 `&mut self`，不是接手整份值。若讓一般流程直接呼叫收尾方法，原值卻仍由呼叫者持有，後續使用與自動清理的責任就不再清楚。Rust 禁止直接呼叫它；提早收尾使用接收 owned 值的 std::mem::drop。

以下完整負例得到 **E0040**，保存的 `.expected` 也完整列出：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/direct_drop.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/direct_drop.expected"
```

本節負例都可放進第 1.2 節的獨立專案，執行 `cargo check`。下面另一份得到 **E0505**：view 在 drop 後還要使用，因此原 value 不能先被交走。把最後一次借讀移到 drop 前，才符合原本的 borrowing 規則。

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/drop_while_borrowed.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/drop_while_borrowed.expected"
```

## 自訂工作之後，owned 欄位仍會清理

再把兩份 Tracked 交進 Pair 的 first、second 欄位。Pair 的 Drop 先讀欄位、顯示自己的收尾工作；這項方法結束後，Rust 繼續清理 owned 欄位。

**struct 欄位按型別定義中的欄位順序清理**，與局部變數的反序不同。下面的 first、second 是 Pair 欄位名，不是前面例子的局部變數：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/drop_fields.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example drop_fields --locked
```

```text
建立Pair=10、20
Pair先收尾，欄位仍是10、20
清理欄位 id=10
清理欄位 id=20
```

一般持有 String、Vec 或 Box 的 struct，不需要為了它們的正常清理再手動重做一遍；這些欄位本來就有自己的清理責任。Drop 用於所需的額外工作。本節觀察的是正常離開 scope 與顯式 drop 的流程。

## 有 Drop 的值，保持完整交接

第 6.3 節的 Copy 允許隱含複製；Rust 不允許同一型別同時實作 Copy 與 Drop。即使欄位只有 i32，下面完整負例也得到 **E0184**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/copy_with_drop.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/copy_with_drop.expected"
```

有 Drop 的值仍可以**整體 move**，如 first 交給 second。但不能直接 move 出它的非 Copy 欄位，留下不完整的值等待自訂收尾。以下 Labelled 的 Drop 要讀 label；即使 main 擁有整份值，也不能先取走 String 欄位，會得到 **E0509**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/move_field_with_drop.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/move_field_with_drop.expected"
```

只需要讀文字時，改用 `value.label.as_str()` 借讀；需要交接時，可以交出整份 Labelled。Drop 內讀 i32 id 則能 Copy 數字，沒有把欄位移空。

## 練習與完整答案

建立 id1、id2 兩份 Tracked，先 drop 第一份，再讀第二份 id，最後印出「main準備結束」。預測 id1 在顯式 drop 時清理，id2 在 main 結束時清理。

完整答案沿用本頁 manifest，沒有額外 helper：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/drop_solution.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example drop_solution --locked
```

應先看到 `清理 id=1`，再看到第二份仍是2，最後才 `清理 id=2`。下一節回到 borrowed 介面：自訂型別如何借出裡面的內容，而不接手它？

[下一節：用 Deref 借用內容](11-04-lesson.md)
