# 3.8 String、str 與 &str：建立及借讀訂單文字

訂單現在只有數字。若還要逐步組成「訂單A：已備妥」這種文字，就需要一份由程式持有、能追加內容的值；而顯示文字的函式通常只需要借讀。這裡分別使用 String 與 &str，沿用本章的 owned 值和 reference 契約。

## 文字內容與借用視圖

Rust 的 **str** 是 UTF-8 編碼的文字 slice。UTF-8 是把 Unicode 文字編成 bytes 的方式，接回第 1.7 節的編碼與儲存單位。本節先整段顯示、追加，不切中文字；第 5.3 節再把 char、bytes 與切字界線連起來。

我們在這裡透過 **&str** 使用 str。它是一份 shared reference，帶著文字區段的位置與 byte 長度，讓函式借讀一段有效的 UTF-8 文字。`"Rust"` 這種一般字串字面值的型別就是 &str；文字保存在程式的靜態資料中，不是每次都建立 owned 的 String。

先寫一個只需要借讀的函式：

```rust
--8<-- "examples/intro-workspace/ownership/examples/strings.rs:show"
```

參數 text 是 &str，println 可直接把這種文字用 `{text}` 顯示。show 不修改、不接手文字；呼叫完成後，提供文字的一方仍可使用自己的資料。

## String 持有可以追加的文字

**String** 是標準函式庫提供的 owned 文字型別。它持有文字的儲存資源，內容需要空間時使用 heap；值在最後的 owner 結束時負責清理資源。空的 String 可以尚未配置 heap 空間，不必把每次建立空值都想成已配置一大塊記憶體。

用第 3.6 節學過的 associated function 呼叫形式，`String::new()` 建立空文字。`push_str` 是修改方法，接受一段 &str，將它的文字內容追加到末尾，因此接收者要能借改。`as_str` 則借讀 String，取得目前內容的 &str：

```rust
--8<-- "examples/intro-workspace/ownership/examples/strings.rs:build"
```

greeting 先是空的，追加 `"Hello, "` 後再追加 `"Rust"`，show 顯示 `Hello, Rust`。`as_str()` 沒有把文字複製成另一份 String；提供視圖的 String 在借用使用期間必須仍然有效。

![greeting 持有 Hello, Rust 的文字資源，as_str 提供借讀同一段文字的 &str 視圖](../assets/ch03/string-view.svg)

圖中的 String 負責資源，&str 只借讀其中一段。兩者在此看到相同內容，責任並不相同。

接著把另一段 &str 追加進去：

```rust
--8<-- "examples/intro-workspace/ownership/examples/strings.rs:append"
```

push_str 接收 shared reference，並把來源內容追加進 greeting；它不接手 ending 的文字。結果是 `Hello, Rust!`。

## move 的規則繼續適用

String 沒有 Copy。把 greeting 指派給 moved，ownership 便交給新 binding：

```rust
--8<-- "examples/intro-workspace/ownership/examples/strings.rs:move"
```

moved 現在負責這份文字資源；原 greeting 不能再用。這次沒有建立第二份獨立文字。至於 &str，像上一節的 shared slice reference 一樣，reference 本身可以 Copy；複製的是視圖資訊，不是另做一份 owned 文字。

## 完整程式

```toml
--8<-- "examples/intro-workspace/ownership/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/ownership/examples/strings.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example strings --locked
```

```text
借來的文字=Hello, Rust
借來的文字=Hello, Rust!
借來的文字=Hello, Rust!
```

每次 show 的借用都在呼叫完成後不再使用，所以之後可以追加，最後也可以把 String move 給 moved。

## 兩份完整失敗例

第一份在 move 後又使用 first，會得到 **E0382**：

```rust
--8<-- "examples/intro-workspace/ownership/fixtures/compile-fail/string_after_move.rs"
```

第二份保留 as_str 借來的 view，想先追加，再顯示舊 view：

```rust
--8<-- "examples/intro-workspace/ownership/fixtures/compile-fail/string_borrow_overlap.rs"
```

view 在最後的 println 還要使用，所以那份 shared 借用涵蓋中間的 push_str。push_str 需要修改借用，兩者衝突，編譯器回報 **E0502**。把這份程式放進獨立練習專案執行 `cargo check` 即可觀察；若先顯示 view，後面不再用它，才追加，就符合第 3.5 節的順序。

## 章末完整模型：訂單連同文字標籤

要讓訂單帶著自己的標籤，struct 的 label 欄位用 String。建立函式接受 owned 的 String，將它 move 到欄位，訂單便負責持有它。其餘單價、數量與原本相同。

模型保留 subtotal 的借讀與 add_count 的借改，再加入兩個操作：`append_label(&mut self, part: &str)` 修改欄位中的 String；`into_label(self) -> String` 接手整筆訂單，交出其中的 owned 文字。後者可以 move 出 label，因為方法收到的是 owned 的 self，而不是 &self。呼叫者在接手之後不再使用原訂單。

下面是完整模型與 main，不依賴其他章的 library：

```rust
--8<-- "examples/intro-workspace/ownership/examples/text_order.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example text_order --locked
```

```text
標籤=訂單A：已備妥，商品小計=240
接手模型後留下的文字=訂單A：已備妥
```

main 先建立 label，再交給訂單；追加標籤只借用來源文字，修改的是訂單持有的 String；最後整筆訂單交給 into_label，讓外層接回文字。建立、借讀、借改與接手，現在都能在同一份模型中看見。

## 練習與完整答案

寫 `append_label(output: &mut String, label: &str)`，向呼叫者的 String 追加文字。從空值開始，依序追加 `"Rust"` 與 `" 教學"`，最後顯示 `Rust 教學`。output 需要修改借用；label 只需借讀來源。

```rust
--8<-- "examples/intro-workspace/ownership/examples/strings_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example strings_answer --locked
```

這份函式完成後，呼叫者仍持有 output。想把這些規則與其他語言對照，可以在本節之後讀[選讀：Python、JavaScript 與 Rust 的可變性](../supplements/python-javascript-mutability.md)；後續主文不以選讀為先備。

下一章讓資料在幾種形態中擇一，再學習如何依形態拆開處理。

[下一節：enum](04-01-lesson.md)
