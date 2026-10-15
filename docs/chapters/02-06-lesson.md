# 2.6 用 module、use 與 pub 組成 library

目前已會計價與加總，也有負責顯示的 `main`。若另一個程式也想使用運費規則，我們希望兩個呼叫者都能找到同一份計算，而不用複製函式。這一節保持輸入與答案，將計算放進 library，再讓 binary 呼叫它。

## 兩個 crate 各有自己的根

第一章區分過 package 與 crate。現在建立的 package 叫 `intro-modules-basic`，資料夾是 `examples/intro-workspace/modules-basic/`。它同時有兩種 crate：

| 根檔案 | 角色 |
| --- | --- |
| `src/lib.rs` | library crate 的根，提供可被其他 crate 使用的功能，沒有 `main` 入口 |
| `src/main.rs` | binary crate 的根，從 `main` 啟動，再呼叫 library |

Cargo 會依這兩個檔名辨認它們，不必另外列 target 設定。Rust 程式內的 library 名稱是 `intro_modules_basic`：package 名稱的連字號換成底線，讓它成為 Rust 可使用的名稱。

接下來的檔案關係如下。`mod` 箭頭表示將 module 納入 library，呼叫箭頭則表示執行時使用公開函式；它們是不同關係：

```text
src/lib.rs ──mod 宣告──→ src/pricing.rs 的計算函式
                                  ↑
src/main.rs 的 main ──呼叫公開函式──┘
```

## module 組織名稱，也決定公開邊界

**module** 將相關程式收在同一個名稱範圍裡。在 `lib.rs` 寫 `mod pricing;`，宣告名為 `pricing` 的 module，讓編譯器載入相鄰的 `pricing.rs`。只把一個 `.rs` 檔放在資料夾裡，不會使它自動成為這個 crate 的 module。

預設情況下，module 和裡面的函式不對外公開。`pub` 是公開的意思：`pub mod pricing;` 公開 module，`pub fn subtotal(...)` 則公開其中的函式。公開 module 不會自動公開全部內容，每個要給外面使用的函式仍要加 `pub`。

現在 `lib.rs` 的完整內容只有這一行：

```rust
--8<-- "examples/intro-workspace/modules-basic/src/lib.rs"
```

`pricing.rs` 先提供已經會的乘法，唯一新增的是函式前面的公開標記：

```rust
--8<-- "examples/intro-workspace/modules-basic/src/pricing.rs:subtotal"
```

運費則保留為 module 內部細節，讓公開的完整計價函式使用：

```rust
--8<-- "examples/intro-workspace/modules-basic/src/pricing.rs:shipping"
--8<-- "examples/intro-workspace/modules-basic/src/pricing.rs:total"
```

`total_with_shipping` 與 `shipping` 位於同一 module，因此能直接呼叫它。外面的 binary 可以呼叫公開的 `total_with_shipping`，卻不能直接呼叫這個私有的 `shipping`。這讓使用者依賴完整計價功能，運費如何拆成 helper 由 module 自己安排。

同一份檔案也放入第 2.3 節已完成的 `sum_three`，並加上 `pub`。這些計算仍使用非負的小訂單數字；`sum_three` 本身則可加總三個不會讓結果溢位的 `i32`。

## 路徑找到項目，use 縮短名稱

Rust 用 `::` 連起名稱層級。例如 `intro_modules_basic::pricing::subtotal` 依序是 library crate、module、函式；它不是檔案路徑，不使用 `/`。

若在這個 library crate **內部**從根開始找，寫 `crate::pricing::subtotal`。`crate` 這個關鍵字表示「目前這個 crate 的根」，不是將 package 名稱寫成英文單字。`src/main.rs` 是另一個 crate，所以它從 library 名稱 `intro_modules_basic` 開始找。

`use 完整路徑;` 將項目引入目前的名稱範圍，後面就能用短名稱。例如引入 `sum_three` 後，可以直接寫 `sum_three(...)`。它仍遵守公開規則；use 不會執行函式，也不負責下載或宣告 module。

```rust
--8<-- "examples/intro-workspace/modules-basic/src/main.rs"
```

這個 main 現在只組合與顯示結果。乘法、運費、迴圈仍在 library；多一個呼叫者時，只需要使用同樣的公開路徑。

## 完整檔案與執行

完整設定如下，仍是第一章已教的三個欄位：

```toml
--8<-- "examples/intro-workspace/modules-basic/Cargo.toml"
```

上面已顯示完整 `src/lib.rs`、`src/main.rs`；下面是它們實際載入的完整 `src/pricing.rs`，包含加總函式：

```rust
--8<-- "examples/intro-workspace/modules-basic/src/pricing.rs"
```

從 repo 根目錄執行主要 binary，這次沒有 `--example`：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-modules-basic --locked
```

```text
訂單總額=180，價目總和=400
```

拆檔後答案相同，但現在可以分別找到「計算規則」與「啟動時顯示什麼」。小程式也能放在同一檔案，這次拆分的用途是讓計算能被多個呼叫者使用。

## 練習：另一個呼叫者，以及它自己的 module

再寫一個範例，使用同一份 library 計算單價 140、數量 2 的商品小計與含運總額。預期為 280 與 300。讓範例自己的顯示函式 `show_order(goods: i32, total: i32)` 放進 `display` module，與 library 的計價工作分開。

先認識本題採用的檔名規則。Cargo 不只辨認 `examples/名稱.rs`，也會將 `examples/名稱/main.rs` 視為名為「名稱」的範例 target；資料夾裡就能保存只屬於這個範例的 helper。

```text
modules-basic/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── pricing.rs
│   └── main.rs
└── examples/
    └── order_answer/
        ├── main.rs
        └── display/
            └── mod.rs
```

在這個範例的 `main.rs` 寫 `mod display;`，可以從同一層的 `display.rs` **或** `display/mod.rs` 載入內容；這兩種是替代路徑，同一個 module 只能選一種。`mod.rs` 是 Rust 仍支援的 module 檔名形式，本題選它，因此建立 `display/mod.rs`，不再建立 `display.rs`。

這項規則也適用於前面的 `src/lib.rs`：`mod pricing;` 可以對應 `src/pricing.rs` 或 `src/pricing/mod.rs`。若在一個 module 內再宣告子 module，檔案則放在以父 module 命名的資料夾下，例如 `pricing.rs` 中的 `mod fees;` 對應 `pricing/fees.rs` 或 `pricing/fees/mod.rs`。module 樹由宣告決定，不是看到檔案就自動加入。

### 完整答案

答案沿用本頁完整 manifest、library root 與 `pricing.rs`，不複製計價規則。範例自己的根檔案是 `examples/order_answer/main.rs`：

```rust
--8<-- "examples/intro-workspace/modules-basic/examples/order_answer/main.rs"
```

顯示 helper 的完整 `examples/order_answer/display/mod.rs`：

```rust
--8<-- "examples/intro-workspace/modules-basic/examples/order_answer/display/mod.rs"
```

`mod display;` 只在這個範例建立自己的 module；`display::show_order` 呼叫它的公開函式。兩行 use 則從另一個 crate——計價 library——取得函式。若 helper 函式不加 pub，位於上層的範例 main 就不能直接呼叫它。

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-modules-basic --example order_answer --locked
```

```text
商品小計=280，含運總額=300
```

目前兩個呼叫者都能使用同一個 library。下一節再加入能自動比對預期答案的測試，這一節的無測試版本會保留原樣。

[下一節：測試、診斷與工具回饋](02-07-lesson.md)
