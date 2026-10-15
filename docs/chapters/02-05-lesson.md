# 2.5 用 loop、break 與 continue 控制中途去向

for 在材料用完時結束，while 在條件不成立時結束。有時候我們想在 body 裡的某個位置說「到這裡就停止」，或者「這一項略過，接著處理下一項」。這一節把這兩種去向分清楚。

## loop 持續執行，break 明確停止

`loop { ... }` 反覆執行 body，不會自行檢查結束條件。`break;` 會立刻結束目前最近一層迴圈，接著執行迴圈後的程式：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/loop_control.rs:plain-break"
```

剩餘量依序為 2、1、0。遇到 0 時 break，因此不會再減一次，最後顯示 0。這段也能用上一節的 while 表達；此處刻意把條件與停止位置寫在 body，觀察 break 的作用。如果沒有任何能到達的停止路徑，loop 就會持續執行。

## 從 loop 交出一個值

`loop` 也能作為 expression。`break 值;` 在停止的同時，讓整個 loop 交出該值：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/loop_control.rs:break-value"
```

`current` 由 0 依次增加；1、2 都繼續，3 時執行 `break current;`，所以 `found` 得到 3。尾端 `};` 的分號結束的是 `let found = ...;` 宣告。

本章以這種方式從 `loop` 取得數字，各個交出數值的 break 出口要提供相容型別。for 和 while 則不能使用 `break 數值` 交出這種結果；它們仍可用沒有數值的 `break;` 停止。

`break` 結束迴圈後，這個 `main` 還會繼續顯示、做下一段工作。第 2.2 節的 `return` 則結束目前函式；兩者離開的範圍不同。

## continue 只跳過本次剩餘工作

若整個走訪還要繼續，只想略過某一項，使用 `continue;`。它立刻跳過 body 剩餘部分，進入下一次：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/loop_control.rs:continue"
```

取得 2 時，continue 使 `total += number;` 這一行不執行；其他項照常相加，所以總和是 0+1+3=4。若換成 break，2 之後的 3 也不會處理，總和就只有 0+1=1。

continue 在 for 會接著取下一項；在 while 會回去重查條件。若 while 的更新寫在 continue 可能跳過的位置，條件就可能一直不變。例如剩餘量為 1 時直接 continue，卻把減法留在後面，就會反覆看到同一個 1。安排更新時，要確保每條繼續路徑都有機會朝停止條件前進。

### 這三段的完整程式

```toml
--8<-- "examples/intro-workspace/control-steps/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/control-steps/examples/loop_control.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example loop_control --locked
```

```text
停止後剩餘=0
loop交出的值=3
跳過2的總和=4
```

## 巢狀迴圈：用 label 指明哪一層

迴圈裡可以再有迴圈。現在用兩段整數 range 表示三列、每列三格：外層取得列號，內層取得格號，依序檢查 `(1, 1)`、`(1, 2)`、`(1, 3)`，再到 `(2, 1)`。這裡沒有引入新的表格資料型別，只使用兩個整數名稱描述位置。

沒寫標籤的 break、continue 作用於**所在位置最近的迴圈**。要從內層直接結束外層，可以在外層前寫 **loop label**（迴圈標籤），例如 `'search:`，再用 `break 'search;` 指向它。開頭有單引號，宣告後有冒號；它是迴圈的名稱。

```rust
--8<-- "examples/intro-workspace/control-steps/examples/loop_labels.rs:labeled-break"
```

找到第 2 列第 2 格時，已檢查第 1 列三格與第 2 列兩格，共 5 格。`break 'search` 離開外層，因此第 2 列後面的格子和第 3 列都不再檢查。若只寫 `break;`，就只離開內層，外層仍會走到下一列。

同樣地，`continue 'rows;` 會開始指定外層迴圈的下一次。在下面另一段獨立走訪中，每列遇到第 2 格就直接換下一列：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/loop_labels.rs:labeled-continue"
```

每列只有第 1 格會加進 `processed`，共處理 3 格。這與只寫 continue 不同：沒有標籤就只跳過內層第 2 格，第 3 格仍會處理。

### 巢狀控制的完整程式

這份程式沿用本頁同一份 manifest：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/loop_labels.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example loop_labels --locked
```

```text
停止搜尋前檢查5格
處理第1列第1格
處理第2列第1格
處理第3列第1格
跳列範例共處理3格
```

第一段提早結束整個搜尋，第二段保留三列、只略過每列後面的工作；兩個計數代表各自那段走訪完成的範圍。

## 練習與完整答案

先完成兩項基本控制：從 1 開始逐次增加，用 loop 找出第一個正偶數並交給變數；再加總 1 到 5，但用 continue 跳過 3。預期答案是 2 與 12。

```rust
--8<-- "examples/intro-workspace/control-steps/examples/loop_control_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example loop_control_answer --locked
```

再改剛才的巢狀控制：兩段都改成三列、每列四格。搜尋仍在第 2 列第 2 格結束；另一段則改成遇到第 3 格就換列。預測各自的計數，再對照完整答案：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/loop_labels_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example loop_labels_answer --locked
```

搜尋會檢查 4+2=6 格；換列範例每列處理前兩格，也是 6 格。數字恰好相同，但前者在第 2 列結束，後者走過三列，不能只看計數就認定去向相同。兩份答案都沿用本頁 manifest。

[下一節：module、use、pub 與 library](02-06-lesson.md)
