# 4.4 組合 pattern，加入範圍、guard 與 @

上一節已能依形狀取出資料。接下來還有四種小需要：兩種形態做同一件事、按連續數值分類、與執行時的門檻比較，以及保存通過檢查的實際值。每次只加一項寫法。

## 用 | 讓幾種 pattern 共用一個 arm

這次 Quote 除了確認的 Amount，也可能是預估的 Estimate；兩者都附帶 i32。本次加總規則接受這兩種金額，Missing 則貢獻 0。

**or pattern** 用 `|` 連起幾種 pattern，只要其中一個符合就選同一個 arm。`Quote::Amount(value) | Quote::Estimate(value)` 會在兩種情況下都建立名為 value 的 i32，後面的工作便能共用它。`|` 在這裡組合 pattern，與第 2 章在 bool expression 中使用的 `||` 不同。

```toml
--8<-- "examples/intro-workspace/patterns/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/patterns/examples/or_patterns.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example or_patterns --locked
```

結果是確認金額 120、預估金額 80、缺值貢獻 0。同一個 arm 的各個 or 選項必須建立相同名稱、相同型別與借用方式的 binding，才能讓共用工作知道手上有什麼。下面完整失敗例在 Missing 那側沒有取得 value，會得到 **E0408**：

```rust
--8<-- "examples/intro-workspace/patterns/fixtures/compile-fail/or_missing_binding.rs"
```

因此不能只因兩個情況想放同一行，就要求沒有附帶資料的 Missing 交出金額。

## 用 range pattern 檢查連續的值

沿用第 2 章的運費分級：小計 0 到 299 收 20，300 到 499 收 10，至少 500 免運。本節材料採不會溢位的非負小整數。

`0..=299` 放在 pattern 位置，表示符合包含兩端的整數範圍；它只檢查**這一個**小計，沒有像 for 中的 range 一樣逐項走訪。同樣可以用已學的 `char` 寫 `'a'..='z'`，檢查一個字元是否在該範圍。

```rust
--8<-- "examples/intro-workspace/patterns/examples/range_patterns.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example range_patterns --locked
```

299、300 的結果分別是 20、10；499、500 則是 10、0。`..=` 的等號使端點包含在該 arm 裡，與第 2 章的範圍約定一致。

## 名稱 pattern 建立 binding，不是讀外面的同名值

第 1.5 節的 shadowing 也適用於 pattern。下面 Amount(limit) 在 arm 內建立新的 limit；它不是「只符合外面那個門檻」：

```rust
--8<-- "examples/intro-workspace/patterns/examples/shadow_patterns.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example shadow_patterns --locked
```

傳入 Amount(50) 與門檻 100 時，分支內顯示 50，分支外仍顯示 100。要與外部門檻比較，需要下一項額外條件。

## guard 先有符合的形狀，再檢查 bool

**match guard** 是寫在 arm 的 pattern 後、`=>` 前的 `if 條件`。先配對 pattern，再算 guard 的 bool；兩項都成功才執行該 arm。若 guard 是 false，便繼續考慮後面的 arm。

在 `Quote::Amount(amount) if amount >= minimum` 中，amount 是新 binding，minimum 則是函式收到的外部門檻。pattern 取得金額，guard 才使用比較運算，沒有把 minimum 寫成另一個 binding。

guard 也會套用在**整個** or pattern。例如 `1 | 2 if ready` 要求數量是 1 或 2，而且 ready 為 true；不是只有數量 2 才檢查 ready。

```rust
--8<-- "examples/intro-workspace/patterns/examples/guards.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example guards --locked
```

120 通過 100 門檻；80 先符合 Amount，但 guard 不成立，後面的 Amount(_) 將它算為 0。數量 1、尚未備妥時也得到 0，直接看見 ready 同樣限制 or 的第一個選項。

guard 不取代完整涵蓋。編譯器不把「帶 guard 的 Amount」當成已處理所有 Amount 值，因此下面仍會得到 **E0004**：

```rust
--8<-- "examples/intro-workspace/patterns/fixtures/compile-fail/guard_not_exhaustive.rs"
```

要像成功例那樣，為未通過條件的金額安排後續 arm，而不是假定 guard 一定成立。

## @ 同時檢查並保留實際值

只有 `1..=3` 的 pattern 能篩選，卻沒有建立一個新名稱保存符合的是哪個數字。**`名稱 @ pattern`** 則同時完成兩件事：檢查右邊的 pattern，成功後讓左邊名稱保存所配對的值。

例如 `actual @ 1..=3` 讓 actual 取得實際的 1、2 或 3，不是讓 actual 保存整段 range：

```rust
--8<-- "examples/intro-workspace/patterns/examples/at_bindings.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example at_bindings --locked
```

輸入 1、3 各保留自己的值，0、4 則走後面的 arm，交出 0。名稱只在符合的 arm 內使用，仍遵守先前的 scope 規則。

## 練習與完整答案

寫一個選出數量的函式：已備妥且數量在 1 到 3，保留實際數量；已備妥且數量是 4 或 5，這次最多選 3；其他情況選 0。先預測 `(2, true)`、`(4, true)`、`(4, false)`、`(0, true)`。

完整答案沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/patterns/examples/combinations_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example combinations_answer --locked
```

結果依序為 2、3、0、0。第三筆保留數量 4，只改是否備妥，就能確認 guard 的確限制整個 `4 | 5` arm；第一筆則確認 @ 保留的是實際數量。

[下一節：pattern 中的 ownership 與 borrowing](04-05-lesson.md)
