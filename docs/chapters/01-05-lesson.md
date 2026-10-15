# 1.5 immutable、mutable 與 shadowing

訂單原本 360 元，加入運費後變成 380。這可以是「同一個總額繼續更新」，也可以是「把下一個階段的答案另存起來」。Rust 用不同寫法表達這兩種安排。

## 預設不能重新指派

上一節的 `let total: i32 = 360;` 是 **immutable** binding：初始值指定後，不能再經由這個 binding 重新指派。下面是完整的預期失敗程式，並非可執行答案：

```rust
--8<-- "examples/intro-workspace/steps/fixtures/compile-fail/immutable.rs"
```

第二次 `total = 380;` 會在編譯時遭到拒絕，重點是 `cannot assign twice to immutable variable`。程式還沒有開始顯示或計算。這讓「這個總額在此階段保持不變」成為編譯器可檢查的規則。

## 用 mut 表達接下來要更新

若確實要繼續累計，宣告時在名稱前加 `mut`，讓 binding 成為 **mutable**。重新指派時不再寫 `let`，而是以 `名稱 = 新值;` 更新。若新值是原值加上某個數，`total += 20;` 是 `total = total + 20;` 的縮寫；它先算右邊所需的加法，再寫回同一變數：

```rust
--8<-- "examples/intro-workspace/steps/examples/mutability.rs:mutable"
```

`total += 20;` 先讀原有的 360，加上 20，再把 380 指派回同一個變數。因此兩次輸出依序是 360 與 380。

`mut` 允許重新指派，不會讓型別改變。以下完整程式刻意把 `3.5` 指派給已確定是 `i32` 的變數，所以得到型別不符的編譯錯誤：

```rust
--8<-- "examples/intro-workspace/steps/fixtures/compile-fail/type_change.rs"
```

## 用 let 表達新的一個 binding

另一種需要是沿用同一名稱表示下一階段的材料。再次寫 `let count = ...;` 會建立**新的 binding**，讓後面的同名使用找到新的值；這叫 **shadowing**（遮蔽）。

```rust
--8<-- "examples/intro-workspace/steps/examples/mutability.rs:shadow"
```

第二個宣告的右側仍讀到第一個 `count` 的 3，算出 4 後，新的 `count` 才開始供後面的程式使用。它沒有修改舊 binding，因此不需要 `mut`。

第三個宣告又建立一個新 binding，這次型別是 `f64`，值為 `4.0`。這是換了一個 binding，所以可以選不同型別；與剛才試圖把浮點數指派給原有 `i32` 的失敗情況不同。三個宣告都沒有 `mut`，各自建立後仍不能重新指派。

更新同一項累計狀態時，`mut` 讓更新意圖直接可見；把材料轉成下一個階段、之後保持不變時，新的 `let` 可以保留這項限制。也可以替新階段取不同名稱，例如 `next_count`，依工作意義選擇。

## 完整範例與執行

```rust
--8<-- "examples/intro-workspace/steps/examples/mutability.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-steps --example mutability --locked
```

```text
加運費前=360
加運費後=380
下一階段數量=4
新型別的數量=4
```

最後的 `4` 是浮點數的顯示結果；少了小數點不會把型別改成整數。型別由程式決定，不是由顯示的外觀決定。

## 練習與完整答案

用同一個 mutable `total` 從 160 加上 20 運費，再扣掉 10 折扣，各顯示一次。扣除可用 `total -= 10;`，它是 `total = total - 10;` 的縮寫。最後用 shadowing 建立同名的下一階段總額，值為折扣後的兩倍；最後這個 binding 不需要 `mut`。

```rust
--8<-- "examples/intro-workspace/steps/examples/mutability_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-steps --example mutability_answer --locked
```

輸出依序是 180、170、340。前兩步重新指派同一個 `i32`；最後一步以 `let` 建立新的 immutable binding。

[下一節：函式、expression、block 與 scope](01-06-lesson.md)
