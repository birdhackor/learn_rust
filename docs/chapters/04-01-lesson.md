# 4.1 用 enum 保存一筆資料的不同形態

前一章的 Order 同時保存單價與數量。現在收到的報價卻可能是「尚未提供」、「有金額」或「被拒絕，附上一個代碼」。只用數字 0 表示沒有報價，會把缺少資料和真正報價為零混在一起。我們需要保存的是哪一種情況，以及那個情況附帶的資料。

## 同一型別，一次選一種形態

**enum** 是列出幾種可能形態的自訂型別，每種形態稱為 **variant**。一個 enum 值在當下只屬於其中一個 variant。struct 描述一筆資料同時有哪些欄位；enum 描述這筆資料是幾種情況中的哪一種。

先把報價定義成 Quote，它需要的三種形態與資料安排分別是：

| variant | 表示的情況 | 附帶資料 |
| --- | --- | --- |
| `Missing` | 尚未提供 | 沒有額外資料 |
| `Amount(i32)` | 已有報價 | 一個按位置保存的金額 |
| `Rejected { code: i32 }` | 被拒絕 | 名為 code 的整數欄位 |

這一階段只建立值，尚未讀取附帶欄位。完整程式會在 enum 前加 `#[allow(dead_code)]`：沿用第 2.7 節的 allow 形式，只讓這個型別暫時未讀取的欄位不觸發 **dead_code** lint。它不會關掉型別或 ownership 檢查；下一節真正讀欄位的程式就不需要這項 allow。先看型別本身的定義：

```rust
--8<-- "examples/intro-workspace/patterns/examples/enums.rs:definition"
```

`enum Quote { ... }` 定義型別，逗號分隔它的三種 variant。

Missing 不需要硬塞一個假金額，Rejected 也不必同時有金額。三種形態仍屬於**同一個 Quote 型別**，所以接受 Quote 的函式可以收到其中任一種。型別保存了「情況與其所需資料」的對應。

## 定義形態，與建立一筆值

定義還不是具體報價。建立值時，用 `Quote::` 指明型別，再選 variant，提供它要求的資料：

下面幾個變數名稱以 `_` 開頭，表示目前刻意不讀它們，避免未使用變數的警告。`_amount` 仍是一個 binding，仍持有建立的值；底線開頭不會取消 ownership 規則。下一節再讀出 variant 中的資料。

```rust
--8<-- "examples/intro-workspace/patterns/examples/enums.rs:values"
```

`Quote::Amount(120)` 建立 Amount 形態，附帶 120；`Quote::Amount(0)` 也會是 Amount，與沒有附帶金額的 Missing 不同。Rejected 則用欄位名稱提供代碼，與上一章建立 struct 的欄位寫法相近。`::` 在這裡連起型別與 variant，不是在讀取某筆資料的欄位。

## 完整範例與執行

本章使用 `patterns/` 成員，package 名稱為 `intro-patterns`。每個例子獨立定義所需型別，沒有 library，也不會載入其他範例。完整 manifest 如下：

```toml
--8<-- "examples/intro-workspace/patterns/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/patterns/examples/enums.rs"
```

從教材 repo 根目錄執行：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example enums --locked
```

```text
已建立三筆報價：缺少、金額120、拒絕代碼37
```

這行固定文字標示三項建立工作已經完成；目前尚未依資料內容選擇顯示方式。建立時必須提供正確形狀與型別，例如 Amount 要收到 i32，Rejected 要提供 code；不會因為三者都叫 Quote，就能互換附帶資料的寫法。

## 練習與完整答案

定義 Shipment，包含無資料的 Waiting，以及帶有命名欄位 `count: i32` 的 Ready。分別建立等待出貨，以及可出貨 3 件的值。此時先保留這兩種情況，不把等待直接改存成數字 0。

答案沿用本頁 manifest，也只建立值，因此對這個 enum 採用相同的局部 dead_code allow：

```rust
--8<-- "examples/intro-workspace/patterns/examples/enums_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example enums_answer --locked
```

建立 Waiting 不需 count，建立 Ready 必須提供 count。下一節讓程式自己分辨是哪種形態，取出對應資料，再決定要做的工作。

[下一節：match](04-02-lesson.md)
