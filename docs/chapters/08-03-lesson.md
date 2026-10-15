# 8.3 Display 與格式化：替錯誤資料定義顯示契約

第 8.1 節用 Debug 顯示 `Negative(-1)`，也讓呼叫者用 match 寫出拒絕原因。若數個呼叫者都要給使用者相同的說明，可以把這項文字規則放在型別上，避免每個地方重寫一次。

## Display 決定 {} 要顯示什麼

**`Display`** 是標準函式庫的格式化 trait，提供 `{}`、`{error}` 這類格式所用的表示。第 6.2 節的 Debug 方便開發者檢查型別與內容；Display 則由實作者選定給使用者看的文字。

數字、String、str 已提供 Display，所以前面能用 `{}` 顯示。自訂的 CountError 則要自己實作；Display 不能 derive，因為編譯器不知道我們希望使用者看見哪種句子。

T 有 Display 時，shared reference `&T` 也能用 `{}` 顯示被借值。這是標準函式庫的實作，不會取得被借資料的 ownership；所以稍後可以借讀錯誤原因來形成訊息。

Display 的必要方法叫 `fmt`：借讀要顯示的值，接收本次格式化操作的輸出狀態，將文字交給它。本節用 `use std::fmt;` 引入 **fmt module**；fmt 是 formatting（格式化）的縮寫，`fmt::Display` 等路徑都從這個 module 取名稱。

## Formatter 與 fmt::Result 各自負責什麼

**`Formatter`** 保存一次格式化操作所需的狀態，包括文字寫入的出口與格式設定。Rust 的格式化流程會提供這份狀態，不需要我們手動建立。

fmt 的參數寫成 `f: &mut fmt::Formatter<'_>`，分開看兩層借用：

| 部分 | 本次用途 |
| --- | --- |
| 外層 `&mut` | 借改這次 Formatter，才能把文字寫到它提供的出口。 |
| 型別內的 `<'_>` | Formatter 自己有內部借用；依第 7.2、7.3 節的 placeholder／含借用模型寫法，讓 Rust 依契約推導，不替它另取 lifetime 名稱。 |

借改模型與模型內部借用是不同角色；這裡沒有要求兩者是同一個範圍。

**`fmt::Result`** 是 `Result<(), fmt::Error>` 的 type alias，沿用第 8.2 節：它沒有建立另一種結果容器。`Ok(())` 表示文字寫入完成，沒有另外交回數字；`fmt::Error` 則代表寫入這次格式化出口時失敗。

CountError 保存的「件數不合法」與這項寫入失敗不同。顯示 Negative 本來就應成功產生「不能為負數」的句子，不能因為它是業務錯誤，就回傳 fmt::Error。Display::fmt 只在 Formatter 的寫入回報失敗時傳回該失敗。

## write! 寫進給定出口，format! 保存成 String

**`write!`** 接收「寫入對象、格式字串、插入值」，把文字交給指定對象。對這裡的 Formatter，它交回 fmt::Result；它不固定向終端機輸出，也不自動加換行。

下面實作借讀 CountError，以 match 選句子。`self` 是 `&CountError`，依第 4.5 節的 pattern 規則，取出的 value 是 `&i32`；`*value` 取得其中數字。每個 arm 尾端直接交回 write 的結果，整個 match 因而符合 fmt::Result，不需要新控制流程語法：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/display/formatting/mod.rs:count-display"
```

**`format!`** 使用相同的格式字串規則，產生一份 owned String。它不直接顯示到終端機，也不自動加換行。和 println 一樣，格式化會借讀插入的值；產生的新 String 則可獨立保存。

## 同一份資料，依用途顯示或保存

完整呼叫者如下。`pub mod formatting;` 載入上面的 trait impl 後，CountError 就有 Display；不需要先呼叫某個初始化函式。這個 target 只載入 policy 與 formatting，完整模組都在本頁後面。

main 先用 Debug 檢查，再用 Display 顯示，最後用 format 保存完整訊息。這些操作只借讀 error，所以同一份值能依序使用：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/display/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example display --locked
```

```text
開發者檢查=Negative(-3)
使用者訊息=件數 -3 不能為負數
存成String=處理失敗：件數 -3 不能為負數
```

Debug 保留 variant 的檢查形式，Display 使用選好的句子；saved 管理的是新 String，原 error 沒有被改成 String。要立即報告就用 println，要把完整文字交給後續工作就用 format。

Display 的一般契約沒有保證輸出能再解析回原值。本例給使用者的錯誤句子也不是一個數字格式；之後學解析時，要另外決定接受的文字規則。

## 完整來源與訊息測試

這些程式仍用 no-lib 的 intro-error-stages，完整 member manifest 如下：

```toml
--8<-- "examples/intro-workspace/error-stages/Cargo.toml"
```

前面已列完整 display/main.rs。這是它載入的完整 policy/mod.rs，只有第 8.1 節的驗證定義與兩項測試：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/display/policy/mod.rs"
```

下面是完整 formatting/mod.rs。新增的測試用 format 保存兩種錯誤訊息，再以第 6.2 節已教的 &str 內容比較，核對原值與原因都沒有漏掉：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/display/formatting/mod.rs"
```

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example display --locked
```

這個 target 共有三項測試：policy 兩項、formatting 一項。訊息測試核對本例選定的文字契約，沒有模擬格式化出口的寫入失敗。

## 練習與完整答案

建立 LimitError，保存 `value: i32`、`maximum: i32`。實作 Display，顯示 `件數 101 超過上限 100`；兩個數字都從欄位取得。再用 value 151、maximum 150，確認同一份實作顯示 `件數 151 超過上限 150`。

兩次都先用 format 保存，再比較完整文字，最後顯示。變動兩個欄位，才能檢查 fmt body 沒把任一數字寫死。完整答案自足，不載入額外 module，沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/display_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example display_answer --locked
```

```text
件數 101 超過上限 100
件數 151 超過上限 150
```

現在文字規則留在 Display 實作，呼叫者只選當場顯示或保存成 String。下一節回到件數本身：普通整數要怎樣轉成已通過驗證的型別？

[下一節：From、Into 與 TryFrom](08-04-lesson.md)
