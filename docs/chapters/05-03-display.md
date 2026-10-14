# 5.3 讓錯誤資料有自己的顯示契約

前節的錯誤可以用 Debug 顯示成 `Negative(-1)`，或讓呼叫者用 match 寫出說明。若數個呼叫者都要顯示相同錯誤，就可以把它「如何形成讀者能看懂的文字」定義在型別上，避免每個地方重寫報告規則。

## Display 回答 {} 要顯示什麼

**`Display`** 是標準函式庫的格式化 trait，提供 `{}` 與 `{error}` 這種格式的文字表示。它和 4.2 的 Debug 用途不同：Debug 方便開發者檢查型別與內容，Display 則由我們選擇符合使用者需求的顯示方式。

數字、String 和 str 已提供 Display，所以前面能用 `{}` 顯示它們。自訂的 CountError 要實作自己的 Display；這項文字規則不能由 derive 自動替我們決定。

當 T 提供 Display，shared reference `&T` 也能用 `{}` 顯示所指的值；這是標準函式庫對 reference 提供的格式化實作，不是取得所指資料的 ownership。因此稍後借用錯誤原因時，也可以直接將那份 reference 交給格式化工作。

`Display` 的必要 method 名為 `fmt`。它借用原值，也接收格式化流程提供的輸出狀態，將文字寫入那個出口。這裡用 `use std::fmt;` 引入 **fmt module**，以 `fmt::Display`、`fmt::Formatter` 和 `fmt::Result` 取得其中的項目；fmt 是 formatting 的縮寫。

## 先讀懂 Formatter 與 fmt::Result

**`Formatter`** 是一次格式化操作的狀態，包含這次文字要寫入的出口與格式設定。Rust 的格式化流程會提供它，作為 fmt method 的 `f` 參數；CountError 的做法只需透過這個參數把訊息交出去。

`f: &mut fmt::Formatter<'_>` 有兩個借用角色：外面的 `&mut` 讓 method 使用這次 Formatter 寫出文字；型別裡的 `<'_>` 則讓 Formatter 內部所需的借用關係依契約推導。這延續 4.5 分開「借用資料模型」與「模型自身借用資料」的觀念。

**`fmt::Result` 是 `Result<(), fmt::Error>` 的 type alias**。`Ok(())` 表示格式化工作完成，沒有另外交回數字；`fmt::Error` 表示寫入這次格式化出口時發生失敗，不保存 CountError 的業務原因。

這點會直接影響實作：顯示 Negative 本來就要成功產生「不能為負數」的文字，不能因為正在顯示的值是一個錯誤，就回傳 fmt::Error。Display::fmt 只在 Formatter 的寫入回報失敗時傳遞該失敗。

## write! 把文字交給給定的出口

**`write!`** 接收「要寫入的對象、格式字串、插入值」，回傳寫入工作的結果。對目前的 Formatter，這個結果就是 fmt::Result；它沒有像 println 一樣固定把文字顯示到終端機並加換行。

我們在 fmt 中借用 CountError 做 match，pattern 取出的 value 是 `&i32`，因此以已教的 `*value` 取得數字。每個 arm 的最後一項直接是 write 的結果，整個 match 也就交出正確的 fmt::Result；不需要提前引入新的錯誤傳遞語法。

**`format!`** 使用同樣的格式字串規則，把結果保存成一個 owned String 並回傳給呼叫者；它不顯示到終端機，也不自動追加換行。下面的 module 測試會用它保存產物，再對照預期訊息。

實作放在本章 library 的 `formatting` module，載入這個 module 後，CountError 就有這項 trait 實作。它仍保有前節的 Debug，沒有改驗證規則：

```rust
--8<-- "examples/ch05-errors/src/formatting.rs"
```

## 當場顯示，或保存成 String

上面的測試用 format 保存產物，println 則立即顯示文字。我們可以依下一段工作需要的形式選擇。

這讓我們可以先建立一段完整訊息，再交給其他工作使用。下面用同一個 error 依序 Debug 檢查、Display 顯示與 format 保存。格式化借用原值，沒有把 error 的 ownership 交出去；保存得到的 String 則由 saved 管理：

```rust
--8<-- "examples/ch05-errors/examples/display.rs"
```

```bash
cargo run -p ch05-errors --example display --locked
```

```text
開發者檢查=Negative(-3)
使用者訊息=件數 -3 不能為負數
存成String=處理失敗：件數 -3 不能為負數
```

同一份錯誤資料可以依用途產生不同表示。Debug 保留 variant，Display 用選好的句子說明；format 保存的是由 Display 產生的內容，而非把整個 CountError 改成 String 型別。

上面的 module 測試也用 format 產生訊息，再透過已教的 as_str 與文字比較，確認兩種原因都保留被拒絕的值。它測的是本例的訊息契約，沒有模擬格式化出口的寫入失敗。

## 練習：上限也來自資料

建立 LimitError，欄位為 `value: i32`、`maximum: i32`。為它實作 Display，顯示 `件數 101 超過上限 100`；兩個數字都應從欄位取得，不寫死在 fmt body。

使用 value 101、maximum 100，先用 format 保存訊息，再比較保存文字與預期句子，最後顯示。若換 maximum，實作仍應使用新的欄位值；這項練習的重點是由型別資料形成訊息，而非為每個呼叫者另外寫 match。

完成後可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch05-errors/examples/display_solution.rs)，或執行 `cargo run -p ch05-errors --example display_solution --locked`。

下一節處理另一種介面約定：把值交給另一個型別時，哪些轉換應該保證成功，哪些必須保留失敗的可能性？
