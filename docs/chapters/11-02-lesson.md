# 11.2 Box 與遞迴資料：用 owned 間接存放收束大小

第 11.1 節分開了型別的固定大小與所借資料的大小。現在要表示一條數字鏈：每個連結保存一個數字，以及下一段鏈；最後用 End 收尾。實際資料只有 80、120 兩項，但同一型別也希望能表示空鏈或更多項。

這份數列仍可放在 Vec。這次改用連結，是要理解一個存放問題：型別的欄位又包含同一型別時，如何取得固定大小的表示？這個問題也會出現在包含子節點的樹狀模型；本節先只處理有限短鏈，總和不超過 i32 範圍。

## 直接包含自己，大小就無法收束

**遞迴型別（recursive type）** 指型別沿著自己的欄位關係，又回到自身。這裡的 Chain 有 End 與 Link；若 Link 的 next 直接保存另一份完整 Chain，就得到以下失敗例：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/recursive_inline.rs"
```

放進獨立練習專案執行 `cargo check`，得到 **E0072**。要決定一份 Chain 多大，編譯器得考慮 Link；Link 包含 i32 與完整 Chain，而那份 Chain 又要能容納同樣的 Link，於是問題不斷回到自身。

End 可以讓某筆**資料**結束，卻沒有替 Chain 的**型別表示**取得固定大小。即使只打算建立兩項，Rust 仍須先決定這個型別的表示。完整 `.expected` 如下：

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/recursive_inline.expected"
```

要改的是 next 的存放方式：目前連結只保存一個固定大小的 handle，讓下一份完整 Chain 放在另一處。這裡還希望目前連結管理下一份的 ownership，因此使用 Box。

## Box 管理另一處的 owned 值

**`Box<T>`** 是標準函式庫提供的 owned pointer 型別，負責它管理的 T 與配置。Box 由 prelude 提供，不用另加 use。對本節這些非零大小的值，**`Box::new(value)`** 取得 value，為它安排 heap 空間，交回 Box。

當 T 是 Sized，`Box<T>` 自身以固定大小的單一 pointer 表示；T 的內容另外存放。因此移轉 Box 不需要複製一份完整的 T。Box 自身也可以放在另一個 struct 或 heap 資料中，不必然在 stack。

先用一個 i32 看清楚如何借讀、借改與交接，再把這些操作用回數字鏈。`*boxed` 找到 Box 管理的值；`&*boxed` 借讀它，`&mut *boxed` 借改它：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/boxes.rs:number-box"
```

view 最後讀完 120，才建立 writable 將原數字改成 140。writable 用完後，`let second = first;` 把整份 Box move 給 second；Box 不會因為裡面是可 Copy 的 i32，就自行變成 Copy。

此時 `*second` 讀出的是 i32，可以 Copy 來顯示與比較。只有 second 繼續管理這份配置，原 first 不能再用。

### 持有 Box 時，也能把 owned 值取出來

若持有 `Box<Amount>`，而 Amount 沒有 Copy，Box 仍支援用 `*boxed` 把裡面的完整 Amount move 出來：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/boxes.rs:take-owned"
```

taken 接手 Amount，原 boxed 不再可用。這項 Box 契約不同於第 3.6 節的 shared reference：借讀者沒有權限將非 Copy 的原值移走；這裡本來就持有整份 Box。也不能從這個例子推論每種 pointer 都允許 move 出內部值。

完整 manifest 與程式如下，沒有其他 helper：

```toml
--8<-- "examples/intro-workspace/type-stages/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/type-stages/examples/boxes.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example boxes --locked
```

```text
借讀 Box 中的數字=120
接手 Box 後的數字=140
從 Box 取回的資料=80
```

Box 的最後 owner 結束時，會依 ownership 規則清理仍由它管理的值與資源。這裡的重點是分清接手、借用與取出，不是為單一 i32 配置 heap 空間來改善計算。

### 轉交後，原名稱不能再讀

下面完整失敗例先把 first 交給 second，又嘗試讀 first：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/box_after_move.rs"
```

執行 `cargo check` 得到 **E0382**，延續第 3.2 節的 move 規則。完整 `.expected` 是：

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/box_after_move.expected"
```

## next 改存 Box，下一份 Chain 就不再直接內嵌

回到開頭的連結。把 next 改成 `Box<Chain>`：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/chain_support/mod.rs:chain-type"
```

每一份 Link 現在只直接保存 i32 與 Box handle；handle 管理另一處的 Chain。決定目前連結的表示時，不必再將下一份完整 Chain 不斷內嵌，因此編譯器能決定固定大小。

下圖對應稍後程式的 80、120、End。實線箭頭表示 Box 管理下一份 owned Chain；它不是 shared reference 的借用箭頭。框與距離是關係示意，不是按比例的記憶體布局：

![chain 中的 80 經由 next Box 管理另一處的 120，120 再經由 Box 管理另一份 End](../assets/types-v3/boxed-chain.svg)

最外層 Chain 由 main 的 chain 持有；第二份 Link 與最後的 End，分別由前一份 next Box 管理。三個位置共同表示有限的兩項資料，型別的直接內容卻不再隨鏈長無止境展開。

## 沿連結加總：接手目前 Chain，再取下一份

into_total 接受 owned Chain。沿用第 4 章的 match 解構，Link 分支接手數字 value 與 Box next；累加後用 `*next` 取得下一份 owned Chain，重新指派給 chain。End 分支直接回傳總和：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/chain_support/mod.rs:into-total"
```

| 目前的 Chain | 累加後的 total | 下一步 |
| --- | --- | --- |
| Link，value=80 | 80 | 從 next 取出 value=120 的 Link。 |
| Link，value=120 | 200 | 從 next 取出 End。 |
| End | 200 | return，結束函式。 |

這裡每輪接手、拆開目前連結，再交接下一份，原鏈因此被消耗。遞迴的是**型別關係**；處理函式使用已教的 loop，沒有另外加入遞迴呼叫。空鏈第一次就遇到 End，直接交回初始總和 0。

### 全部 source 與測試

這一步沒有 library。`examples/chain.rs` 與本頁練習答案都明寫 `mod chain_support;`，只載入同層 `chain_support/mod.rs`。以下是完整 helper，只有 Chain、into_total 與兩項測試：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/chain_support/mod.rs"
```

第一項測試分別驗證空鏈、一項、兩項；第二項確認 0、-5、5 都作為資料參與加總，不把 0 當成結束標記。真正的結束標記是 enum 的 End。

完整呼叫者從 End 向外建立 120，再建立 80；設定沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/chain.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example chain --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example chain --locked
```

```text
沿連結加總=200
空鏈加總=0
```

兩項 test 也應通過。Box 回答了如何以 owned 間接存放取得固定表示；數字仍按原順序相加。這沒有證明連結表示比 Vec 快，String 與 Vec 也不需要再套 Box 才能自行管理 heap 資料。

## 練習與完整答案

建立 40、80、120 三項鏈，以 End 收尾，交給同一份 into_total，確認總和 240。先從最內層的 End 往外組，說出每個 next Box 分別管理哪份 Chain。

下面是完整答案，使用本頁已完整提供的 chain_support 與 manifest，沒有其他 helper：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/chain_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example chain_answer --locked
```

輸出 `三個連結的總和=240`。改成三項後，每份 Chain 仍使用相同的型別表示；增加的是由 Box 管理的下一份資料，不是直接內嵌的型別大小。

下一節具體觀察 owner 結束時的收尾順序，再為自訂型別安排結束前要做的工作。

[下一節：用 Drop 觀察 ownership 的收尾](11-03-lesson.md)
