# 11.12 用 DropGuard 在批次失敗時回復原清單

前面已能用 closure 保存工作，以 `?` 提早回報失敗，並透過 Drop 安排收尾。現在把它們用在同一個需要：**把一批文字解析成 i32，全部成功才保留新增資料；中途任何一筆失敗，原清單保持不變。**

原清單先有 `[80]`。追加 `"120"`、`"40"` 應得到 `[80, 120, 40]`；接著嘗試 `"60"`、`"不是數字"`、`"100"` 時，60 雖已追加，遇到錯誤仍須回到這次呼叫前的 `[80, 120, 40]`。

## 在開始修改前，先保存回復工作

**`std::mem::DropGuard`** 是 Rust 1.100 的標準型別，包住一份值，在它被 drop 時將這份值交給一個 closure。這讓一次性的收尾工作可以寫在使用位置，不需另外定義只用一次的 struct 與 Drop impl。

本例包住的是 `&mut Vec<i32>`：原 Vec 仍由呼叫者持有，guard 保存這份獨佔借用。closure 保存原長度，失敗時刪去這次新增的尾段。

先認識本次需要的操作：

| 操作 | 收到什麼、完成什麼 |
| --- | --- |
| `DropGuard::new(value, closure)` | 保存 value 與收尾 closure；此時尚不執行 closure。 |
| guard 被 drop | 將保存的 value 交給 closure，執行一次收尾；這需要第 9.2 節的 FnOnce 能力。 |
| `guarded.push(value)` | DropGuard 的 DerefMut 讓 method 查找能借改包住的 Vec，沿第 11.4 節的規則。 |
| `counts.truncate(len)` | 將 Vec 縮到 len 個元素，清理多出的尾段；若 len 不小於目前長度就不改變它，不會補新元素。 |
| `DropGuard::dismiss(guard)` | 接手 guard，交回包住的值，不執行收尾 closure。這是 associated function，不寫成 `guard.dismiss()`。 |

本例只做 push，沒有覆寫舊元素，所以縮回原長度就能還原。若操作會修改原有元素，只截去尾段就不足以回復，必須另外設計保存與還原方式。

## `?` 提早返回，guard 就負責截去新尾段

以下函式的錯誤型別 **`std::num::ParseIntError`** 是標準整數解析錯誤，第 8.5 節的 `parse::<i32>()` 失敗時就交回它。函式仍使用已教的 `Result<(), 錯誤型別>` 及 `?`：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/drop_guard.rs:append"
```

`original_len` 是 usize 數字，`move` closure 按值保存它。guard 自己保存的是 counts 的 mutable reference，不是另一份複製的 Vec。

每次先解析，成功才 push。解析失敗時，`?` 交回 Err，尚未 dismiss 的 guard 隨離開函式而 drop，closure 便把尾段截回原長度。只有迴圈全部完成，才執行 dismiss，取消這次回復工作並交回包住的 reference；本函式不再使用它，接著回 Ok。

| 路徑 | 離開前的動作 | 呼叫者看到什麼 |
| --- | --- | --- |
| 每筆都解析成功 | dismiss，保留新增尾段 | Ok，以及完整的新清單 |
| 追加一筆後，下一筆失敗 | `?` 返回，guard drop 並 truncate | Err，以及本次呼叫前的清單 |
| 空批次 | 沒有 push，仍 dismiss | Ok，以及原清單 |

這個安排保留了逐筆修改方式，並把失敗時的回復集中在開始修改的位置。若只需要從整批文字建立一份新的 Vec，第 9.5 節的 Result collect 已可完成；本節選擇修改既有清單，才有回復本次新增尾段的需要。

## 完整程式與實際驗收

仍使用沒有 library 的 intro-type-stages，所有 helper、main 及測試都在這份 example 中：

```toml
--8<-- "examples/intro-workspace/type-stages/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/type-stages/examples/drop_guard.rs"
```

從教材 repo 根目錄執行：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example drop_guard --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example drop_guard --locked
```

```text
成功後=[80, 120, 40]
中途失敗後=[80, 120, 40]
```

第二行保留第一批已完成的資料，沒有留下失敗批次的60，也沒有清空原清單。三項測試分別核對成功保留、中途失敗回復與空批次。成功測試若省去 dismiss 就會失敗；失敗測試若沒有回復 closure，也會發現多出的60。

## 練習：回復的界線是哪一次呼叫？

先不執行程式，假設原清單是 `[80]`，第一次追加 `"0"`、`"20"` 成功；第二次追加 `"60"`、`"bad"` 失敗。最後清單會只有80，還是也保留0、20？再用同頁完整程式替換 main 的兩組材料確認。

??? example "答案與理由"

    最後是 `[80, 0, 20]`。第二次進入 append_counts 時原長度已是3，回復只截去這次新增的60；不會撤銷第一次已完成的結果。main 中的對應預期值也要一起改成 `[80, 0, 20]`，再執行相同的 cargo run 命令。

本節驗證的是正常完成與 `?` 返回時的記憶體內資料變化；這不是寫入外部儲存的持久化交易，也不保證程序被外部直接終止時還能執行清理。

延伸閱讀：[DropGuard 1.100 文件](https://doc.rust-lang.org/1.100.0/std/mem/struct.DropGuard.html)、[Rust Book：Drop](https://doc.rust-lang.org/book/ch15-03-drop.html)、[正體中文對應章](https://rust-lang.tw/book-tw/ch15-03-drop.html)。

下一章把視角移到多個 package 的共同設定與不同建置，並逐步安排檔案處理工作。

[下一節：Cargo 工程設定](12-01-lesson.md)
