# 3.7 slice：借用連續的一段資料

第 2.3 節用固定長度的 array 計算總和。現在同一個加總函式還要處理五項價格的中間三項、全部五項，以及另一份只有兩項的資料。我們可以讓函式接受 **slice**：原資料中連續的一段，不必為每種 array 長度重寫函式。

## 起點與長度決定借到哪一段

array 型別 `[i32; 5]` 把長度 5 寫進型別；slice 型別 `[i32]` 不把固定長度寫進型別。本節透過 shared reference 使用它，參數型別寫成 `&[i32]`。這份 slice reference 帶著起點與長度，供程式定位原資料中的連續區間；它不另建一份元素的 array。

沿用價格 `[80, 120, 200, 60, 40]`。`&prices[1..4]` 借出原索引 1、2、3，包含起點 1，不包含終點 4，得到 120、200、60 三項。

![五項價格中借出原索引 1 到 3，slice 自己的索引由 0 開始](../assets/ch02/slice-part.svg)

圖中的 selected 不是從原索引 0 開始。因此 `selected[0]` 讀的是原本 `prices[1]` 的 120，`selected.len()` 得到長度 3。len 是查詢長度的方法，回傳第一章已用於索引的 usize。呼叫用 `selected.len()`，不需要另給參數。

## 同一個函式接不同長度

函式只需逐項加總，不需知道呼叫者原本的 array 長度，因此參數改成 `&[i32]`。對這個具體型別使用 `for value in values`，每次取得一份 `&i32`，依 slice 順序借讀一個元素。`*value` 讀出 Copy 的 i32 再加入 total：

```rust
--8<-- "examples/intro-workspace/ownership/examples/slices.rs:sum"
```

空 slice 沒有元素，迴圈一次也不執行，所以回傳初始總和 0。

有了這個函式，中間三項的借用與計算可以寫成：

```rust
--8<-- "examples/intro-workspace/ownership/examples/slices.rs:selection"
```

結果是長度 3、第一項 120，總和為 `120 + 200 + 60 = 380`。slice reference 本身可以 Copy；把 selected 傳進函式不會搬走原 array，也不會複製那三個元素。

## 整段、前段、後段與空段

在這份五項 array 上，省略範圍端點有以下意思：

| 寫法 | 原資料中的區間 | 內容 |
| --- | --- | --- |
| `&prices[..]` | 開頭到結尾 | 80、120、200、60、40 |
| `&prices[..3]` | 0 到 3，不含 3 | 80、120、200 |
| `&prices[3..]` | 3 到結尾 | 60、40 |
| `&prices[2..2]` | 起點等於終點 | 空段 |

對一般的 `start..end`，必須滿足 `start <= end`，且 end 不超過原長度；合法空段也包含結尾的 `5..5`。建立區間時超出界線，或索引時超出該 slice 的長度，都會 panic。

## 完整程式

```toml
--8<-- "examples/intro-workspace/ownership/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/ownership/examples/slices.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example slices --locked
```

```text
選到的長度=3，第一項=120
選到的總和=380
全部總和=500
空區間總和=0
兩項資料的總和=200
```

五項 array 的中段、整段、空段，以及兩項 array，現在都交給同一份 sum_values。它統一處理的是「這次借來的那一段」，原 array 的長度仍各自保留。

## slice 有自己的索引界線

下面是完整的預期執行失敗例。函式的 position 型別用 usize，因為它作為索引：

```rust
--8<-- "examples/intro-workspace/ownership/fixtures/runtime-fail/slice_bounds.rs"
```

它可以編譯，但執行時會因 `index out of bounds` panic：selected 長度是 3，合法索引是 0、1、2，索引 3 已經超出這份 slice。原 array 還有後面的價格，並不代表 slice 能讀到那些項目。

slice reference 同樣受第 3.5 節的借用規則約束。若 selected 稍後還要讀取，就不能在那之前衝突地修改它借讀的資料。

## 練習：用 mutable slice 修改指定區段

現在希望只替中間兩項各加 10，保留其餘價格。型別 `&mut [i32]` 提供這一段的獨占修改借用。建立時寫 `&mut prices[1..3]`，而 prices 自己要宣告為 mut。

對 `&mut [i32]` 使用 for，每次取得的是 `&mut i32`，因此可以經由 `*value` 修改原元素：

```rust
--8<-- "examples/intro-workspace/ownership/examples/slices_answer.rs:increase"
```

把完整答案寫成：先讓原索引 1、2 加 10，檢查得到 80、130、210、60、40，總和 520；再對空段做同樣操作，確認總和仍是 520。

```rust
--8<-- "examples/intro-workspace/ownership/examples/slices_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example slices_answer --locked
```

```text
五項價格=80、130、210、60、40
完整總和=520，空區間沒有修改資料
```

increase 完成借改後，main 才檢查原 array，所以讀取與修改不會重疊。下一節把 owned 資料與借用視圖的區別用到文字。

[下一節：String、str 與 &str](03-08-lesson.md)
