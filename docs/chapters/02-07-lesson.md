# 2.7 用測試、診斷、fmt 與 Clippy 確認程式

計價程式已能顯示幾個正確答案。現在想保存依規則算出的預期值，讓下一次改運費條件時，工具能重新檢查這些答案。本節在另一個 package `intro-control` 保存有測試的版本，資料夾為 `examples/intro-workspace/control/`；第 2.6 節的無測試 library 仍保留。

## assertion 把預期寫成檢查

**assertion** 是程式中「這項條件應該成立」的檢查。`assert!(條件)` 要求一個 `bool`：為 true 時繼續，為 false 時 panic。第一章已看到 panic 會停止正常執行；若發生在測試函式，該測試就會失敗。

`assert_eq!(左值, 右值)` 檢查兩個值是否相等，失敗時還會列出兩個值，方便找出差異。本章只比較已教的 `i32` 與 `bool`。這些與 `println!` 一樣是帶 `!` 的 macro，現在使用已說明的呼叫形式即可。

下面是之後測試中的一小段。商品小計 280，含 20 元運費後為 300，因此兩項檢查都應通過：

```rust
--8<-- "examples/intro-workspace/control/examples/assertions_answer.rs:answer-checks"
```

本書習慣將實際結果放左邊，預期值放右邊；Rust 的訊息稱它們為 left、right，並不強制哪邊一定是預期值。assertion 成功時不顯示結果；測試材料與預期值要依規則選定，不能只把目前程式的輸出原封不動當成規則。

## 屬性、測試函式與測試 module

**屬性**（attribute）是附在 Rust 程式項目前面的標記，告訴工具如何處理該項目。`#[...]` 是此處屬性的外框；它是 Rust 語法，和 TOML 區段的中括號不同。

在函式前加 `#[test]`，會將它標記為測試函式。這裡的測試函式沒有參數，回傳 unit；`cargo test` 建立的**測試執行器**會呼叫它、記錄通過或失敗，不需要在 main 手動列出每個測試。

測試也需要整理名稱。`mod tests { ... }` 是 module 的同檔形式：內容直接放在花括號內；第 2.6 節的 `mod pricing;` 則到另一份檔案載入內容。`tests` 只是慣用的 module 名稱，讓執行器辨認測試的是函式前的 `#[test]`。

在整個 module 前加 `#[cfg(test)]`，表示只在測試建置時納入它。`cfg` 來自 **configuration**，意指依建置條件選擇程式部分；這次的條件是 `test`，由 Rust 的測試建置啟用。本書透過 `cargo test` 建置它；一般 `cargo run` 不納入這個 module。

現在把這些標記與已知函式連在一起：

```rust
--8<-- "examples/intro-workspace/control/src/lib.rs:unit-tests"
```

`crate::pricing::...` 從**目前這個 library** 的根找計價 module，這是第 2.6 節已教的路徑規則。三行 use 各自把函式引入 tests 的範圍；外層的名稱不會因為我們想用，就自動成為這個 module 的短名稱。

四個測試各處理一項已知規則：小訂單加運費、299 與 300 的門檻邊界、空訂單、三項全部參與加總。最後的 `assert!` 則示範直接檢查 bool 條件。這些測試各自建立輸入、呼叫函式，不使用其他測試留下的狀態，也不依賴列出名稱的順序。

## 先看到通過，再讓一個錯誤被抓到

這次用 `--lib` 只選這個 package 的 library 測試。在 repo 根目錄執行：

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --lib --locked
```

摘要應是四項通過、零項失敗。這與 `cargo run` 顯示業務結果不同：這次是執行器記錄各項 assertion 是否成立。

接著在自己的練習副本中，把 `examples/intro-workspace/control/src/pricing.rs` 的免運條件從 `subtotal >= 300` 暫時改為 `subtotal > 300`，再跑相同指令。恰好 300 的訂單會被多收 20，`shipping_threshold_includes_300` 應失敗，差異為：

```text
assertion `left == right` failed
  left: 320
 right: 300
```

299 的訂單仍收運費，只有剛好到門檻的材料揭露這個差異。這項變更沒有違反型別規則，所以單純編譯不會抓到；測試補上的是選定規則與答案的檢查。

觀察後將條件改回 `>=`，再確認四項通過。這些材料仍不等於證明所有 `i32` 輸入都正確；訂單採用非負、小數值的輸入約定。

## fmt 與 Clippy 各負責什麼

`cargo fmt` 使用 `rustfmt` 整理縮排與排版，會修改檔案。若只想檢查，用 `-- --check`；第一個 `--` 把後面的選項交給 rustfmt，`--check` 要求它只報告排版差異。

這裡的 `--check` 是 rustfmt 選項，和獨立的 `cargo check` 不同。排版一致不表示運費算對。完整 package 的排版命令會在第 2.8 節所有範例都提供後執行；這裡先認識工具角色。

`cargo clippy` 使用 **Clippy** 檢查常見寫法問題。這種針對可能不理想寫法的檢查叫作 **lint**；它可能指出多餘操作或容易出錯的安排，不能代替運費規則的測試。

先沿用本節已教的 `--lib`，只檢查 library 與它載入的 `pricing.rs`：

```bash
cargo clippy --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --lib --locked
```

若固定工具鏈缺少元件，`rustup component add rustfmt clippy` 可補裝這兩個工具；教材的固定工具鏈已包含它們。

### 先讀訊息，再決定改法

先找檔案與行號，再讀訊息主句，回頭對照該處的工作。例如第一章重新指派 immutable binding 的錯誤，是語言規則不允許；本節測試的 left/right 差異，則是執行結果不符合預期。兩者需要修的原因不同。

Clippy 的建議也要看用途。若函式最後一行寫 `return price * count;`，它會報告 `clippy::needless_return`：這個位置已能用尾端 expression 交出值，因此可移除 return 與分號。一般程式直接採用尾端 expression 即可。

有時教材刻意並列兩種寫法，要保留它們供比較。這時可以在**那個函式**前加 `#[allow(clippy::needless_return)]`：只讓這個具名 lint 在此函式內不報告。`allow` 不會使型別錯誤合法，也不會保證計算正確；理由應寫在相鄰說明或註解中，不能為了消除所有提醒而一概關掉檢查。

下面是完整的語法比較程式。第一個函式刻意保留 return，因此只有它帶精確的 allow；第二個函式直接使用尾端值。程式中的 `//` 開始一行註解，供人閱讀、不參與執行，這裡用來留下保留寫法的理由：

```rust
--8<-- "examples/intro-workspace/control/examples/lint_return.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --example lint_return --locked
```

兩個版本都得到 160。這項 allow 的用途只是保留當下比較材料。以後遇到建議使用尚未學過方法的 lint，也先核對要保留什麼行為、為何保留目前寫法，再決定是否只對該處使用精確的 allow。

現在已讀過這份完整範例與 allow 的理由，再用 Clippy 明確選它檢查：

```bash
cargo clippy --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --example lint_return --locked
```

## 有測試的計價程式：全部來源

這是本頁實際使用的 member manifest、library、計價 module 與主要 binary。各檔案都在 `examples/intro-workspace/control/`：

??? example "展開完整設定與所有程式檔案"

    `Cargo.toml`：

    ```toml
    --8<-- "examples/intro-workspace/control/Cargo.toml"
    ```

    `src/lib.rs`：

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

主要程式仍可用下列指令執行，得到 `訂單總額=180，價目總和=400`：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --locked
```

## 練習與完整答案

單價 140、數量 2，商品小計 280，加運費後總額剛好 300。請替這個情況新增測試，分別確認小計 280 與完整總額 300。免運判斷使用**商品小計**，不能因加運費後等於 300，就倒過來認定原訂單應免運。

完整答案沿用同頁的 manifest 與 library 檔案，再建立 `control/examples/assertions_answer.rs`。這是 library 外的範例 crate，因此它和其 tests module 都用 `intro_control::...` 找 library；在這份範例寫 `crate::...` 會從範例自己的根開始，並不是 library 根。

```rust
--8<-- "examples/intro-workspace/control/examples/assertions_answer.rs"
```

這份範例 main 顯示數字；其測試函式負責 assertion。用 `--example` 選它來跑測試：

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-control --example assertions_answer --locked
```

應有一項 `shipping_uses_goods_subtotal` 通過；它是剛才四個 library 測試之外的檢查。若改用 `cargo run` 配合同一個 `--example assertions_answer`，則執行 main，顯示 `商品小計=280，含運總額=300`。

[下一節：完整組合計價與加總 library](02-08-lesson.md)
