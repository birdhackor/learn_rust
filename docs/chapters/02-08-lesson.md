# 2.8 完整組合計價與加總 library

這一節把本章已做過的工作接成一個完整小程式：library 保存規則，binary 顯示結果，測試保存預期答案。先從規格和檔案分工重建一次，再對照同頁的全部來源；本節不增加新的 Rust 語法。

## 先固定要保留的規則

訂單使用非負的小整數，乘法與加法不超出 `i32` 範圍。library 提供三個公開函式：

| 函式 | 要保留的行為 |
| --- | --- |
| `subtotal(price, count)` | 單價乘數量，交出商品小計 |
| `total_with_shipping(price, count)` | 空訂單回傳 0；其他訂單小計至少 300 免運，低於 300 加 20 |
| `sum_three(values)` | 加總三項 `i32`；這項功能允許負數，仍約定總和不溢位 |

先選能區分規則的材料：`(80, 2)` 的總額是 180；`(300, 1)` 是 300；`(120, 0)` 是 0。加總 `[120, 80, 200]` 為 400，`[0, -5, 5]` 為 0。這些是重建後要保留的答案。

## 先接計算，再接呼叫者

計價函式先處理空訂單，再呼叫商品小計與私有運費函式。每一步都沿用已教的條件、return 與函式呼叫：

```rust
--8<-- "examples/intro-workspace/control/src/pricing.rs:total"
```

`sum_three` 讓每一項資料加入同一個累積值，最後將結果交回：

```rust
--8<-- "examples/intro-workspace/control/src/pricing.rs:sum"
```

這些函式放在 `pricing.rs`，由 library root 的 `pub mod pricing;` 接入。binary 不再重寫乘法或迴圈，只取得公開結果：

```rust
--8<-- "examples/intro-workspace/control/src/main.rs"
```

同一份 `pricing.rs` 因而同時被 main 與測試使用。規則改變時，修改計算函式，再用測試比對；若只想改顯示文字，則找 main。

## 完整設定與原始碼

下列四份檔案就是這個版本的設定、library root、計算 module 與 binary root，全部位於 `examples/intro-workspace/control/`。入門 workspace 的組織與選 target 方式沿用第 1.3 節；本頁指令只選 `intro-control`，不需要打開其他成員的程式。

??? example "展開完整設定與程式"

    `Cargo.toml`：

    ```toml
    --8<-- "examples/intro-workspace/control/Cargo.toml"
    ```

    `src/lib.rs`（包含全部四項 library 測試）：

    ```rust
    --8<-- "examples/intro-workspace/control/src/lib.rs"
    ```

    `src/pricing.rs`：

    ```rust
    --8<-- "examples/intro-workspace/control/src/pricing.rs"
    ```

    `src/main.rs`：

    ```rust
    --8<-- "examples/intro-workspace/control/src/main.rs"
    ```

先執行，再跑 library 測試：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --lib --locked
```

主要程式應顯示 `訂單總額=180，價目總和=400`；四項 library 測試應全部通過。兩項結果各自回答不同問題：前者看這次啟動如何組合顯示，後者比對選定輸入是否仍符合規格。

## 練習：自己接一份訂單報告

在自己的練習副本中，先不看答案，新增 `control/examples/my_report.rs`。它有自己的 main，透過 use 使用 library 的三個公開函式，不複製計價規則。請完成：

1. 顯示單價 140、數量 2 的商品小計與含運總額，再顯示三項價目 `[120, 80, 200]` 的總和。
2. 在這個範例自己的 tests module 中，確認小計 280、含運總額 300、恰好 300 的訂單免運、空訂單總額 0。
3. 再用兩組 array 確認加總 400 與 0。

你可以先寫出 main，觀察顯示；再加入 `#[cfg(test)]`、同檔 module 與測試函式。每項預期值都依上面的規格判斷，不以「程式目前印多少」取代規格。

在 repo 根目錄選自己的檔名執行與測試：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --example my_report --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --example my_report --locked
```

## 完整參考答案

答案保存為 `control/examples/integration_answer.rs`，不會覆蓋你自己的檔案。它使用的完整 manifest、lib 與 pricing 都已在本頁內嵌；下面是答案 target 的全部來源：

```rust
--8<-- "examples/intro-workspace/control/examples/integration_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --example integration_answer --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --example integration_answer --locked
```

```text
商品小計=280，含運總額=300
價目總和=400
```

這個 target 有兩項測試。它和 library 的四項測試都呼叫同一份公開功能；選 `--example integration_answer` 時，摘要只顯示這個範例的兩項。

## 檢查完整 package

現在已讀過這個 package 的 library、主要 binary、本頁答案，以及第 2.7 節的 assertion 與 return 比較範例。此時可以把工具範圍擴大到完整 package。

先用第 2.7 節介紹的 fmt 整理排版，或加 `-- --check` 只檢查、不修改檔案：

```bash
cargo fmt --manifest-path examples/intro-workspace/Cargo.toml -p intro-control
cargo fmt --manifest-path examples/intro-workspace/Cargo.toml -p intro-control -- --check
```

Clippy 的 `--all-targets` 要求涵蓋所選 package 的 library、binary、範例與測試等 target，而不是只選其中一個。這次所有來源都已提供，包含刻意保留的具名 allow 與它的理由：

```bash
cargo clippy --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --all-targets --locked
```

排版、lint 與測試仍各有工作；全部工具通過後，仍要核對預期是否忠於規格。

現在這個小程式已有清楚的計算規則、呼叫者與檢查位置。下一章會替一筆資料的各個欄位命名，接著研究值如何交給其他函式，以及何時可以借給另一段工作使用。

[下一章：ownership、reference 與 borrowing](03-01-lesson.md)
