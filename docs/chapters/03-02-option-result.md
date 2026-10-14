# 3.2 把缺值與失敗留在結果中

上一節自行定義了 Quote。很多工作不需要專屬 variant 名稱，只要表達「有一個答案或沒有答案」。另一類工作則需要「成功取得答案，或失敗並保留原因」。Rust 標準函式庫分別提供 **Option** 與 **Result** 這兩種 enum，本節先處理裡面放的是具體數字的情況。

## Option：有數字，不代表一定有數字

Option 的 **Some** variant 保存一個值，**None** variant 表示沒有值。`Some(120)` 因而是「有一個數字 120」，None 是「沒有數字」，和數字 0 不同。

這些名稱已經由 Rust 自動帶入一般程式可用的範圍，不需要像自訂 enum 一樣寫 `Quote::`，也不需要下載外部程式庫。標準提供的 Some、None 仍是 variant，不是兩個可任意混用的數字。

取得 Option 裡的數字，要先用已教的 pattern 分辨情況。`Some(amount)` arm 拆出金額，None arm 決定缺值時的工作。不能直接把整份 Option 當成金額乘數量；缺值的可能性還沒有被處理。

目前程式會用兩種建立方式：一份直接來自 `Some(120)`；另一份依 bool 條件選 Some 或 None。兩個 `if` 分支屬於同一種「可能有數字的結果」，因此可以一起作為變數的初始值。編譯器會從 Some 中的數字推導所保存的資料型別。

## Result：失敗時，還有原因可以處理

1.4 已知道不能以零作整數除數。現在要在計算前先檢查，而不是讓非法除法開始執行。Result 的 **Ok** variant 保存成功值，**Err** variant 保存失敗資訊。這裡成功值是整數商，失敗資訊是自訂的 `DivideError::ZeroDenominator`。

`Ok(6)` 表示成功得到 6；`Err(DivideError::ZeroDenominator)` 表示沒有成功數字，原因是除數為零。Err 中保存的是我們設計的資料；它不會自動顯示錯誤或停止程式。接收結果的程式可以用 match 決定怎麼處理。

下面先展示 Some／None，再以同一份除法材料 12、兩個除數 2 與 0，展示 Ok／Err。只有非零分支真正執行 `/`：

```rust
--8<-- "examples/ch03-data/examples/optional_result.rs"
```

```bash
cargo run -p ch03-data --example optional_result --locked
```

```text
有報價=120
沒有報價
整數商=6
不能用零作除數
```

第一個 match 取得 Some 中的 120；第二個 match 選 None。除數 2 產生 Ok(6)，除數 0 產生 Err，接著由 Err arm 顯示原因。因此沒有任何一步要「從失敗裡取出一個假數字」來完成除法。

本章的除法材料仍採前文的小整數約定，不涵蓋所有 i32 邊界運算。這份例子建立的是零除數的處理路徑，不是完整數值函式的全部錯誤政策。

## 選擇依照需要保存的資訊

| 要表達的工作結果 | 表示方式 | 呼叫者必須決定的事 |
| --- | --- | --- |
| 可能沒有報價，沒有額外的失敗原因 | Some／None | 有值怎麼使用、缺值怎麼處理 |
| 計算可能失敗，原因要交給下一段工作 | Ok／Err | 成功值怎麼使用、失敗資訊怎麼處理 |

None 不會告訴你是誰拒絕、資料格式錯在哪裡；Err 則可以保存這些資訊。因此不是「有錯誤就一律用 None」，也不是「沒有值就是程式崩潰」。結果的資料形狀，要和接收者需要回答的問題相符。

如果只關心有值或成功，也能用上一節的 `if let Some(...)` 或 `if let Ok(...)`。但若失敗原因影響處理結果，就要把 Err 的工作明確寫出，不能為了縮短程式把它略過。

## 練習：零元有效，負數無效

用自訂 `AmountError::Negative` 表示「金額為負數」。在迴圈依序處理 0、−5：非負時建立 Ok，負數時建立 Err，再用 match 顯示。

你應看到 `接受金額=0`，再看到 `拒絕負數金額`。這兩組材料特別確認「沒有成功值」和「成功得到零」不同。先不要把原工作拆成回傳 Option／Result 的函式，下一節會把它們的完整型別寫清楚。

完成後可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch03-data/examples/optional_solution.rs)，或執行 `cargo run -p ch03-data --example optional_solution --locked`。接下來會學泛型，讓上述相同的結果形狀能保存不同型別的內容。
