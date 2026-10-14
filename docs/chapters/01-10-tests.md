# 1.10 用測試與工具回饋確認程式

程式能通過編譯、也印出幾個正確數字，還不能保證下一次修改運費門檻時沒有改錯。我們想保存這些已知答案，讓工具每次替我們重新比較。這就是本節的測試任務。

## assertion 讓預期答案成為可執行檢查

**assertion** 是程式中的斷言：宣告一項應當成立的條件，並在執行時檢查它。`assert!(條件)` 要求條件是一個 `bool`；為 `true` 時繼續，為 `false` 時 panic。`assert_eq!(實際值, 預期值)` 比較兩個值是否相等；本章只比較已經教過的 `i32`，不需要寫新的比較介面。

例如已經知道 `subtotal(80, 2)` 應是 160，就能用 `assert_eq!(subtotal(80, 2), 160);` 保存這項判準。它和 `println!` 一樣是帶 `!` 的 macro，但用途是檢查條件，成功時不顯示數字。

`assert_eq!` 發現不相等時，錯誤會列出左邊的實際值和右邊的預期值。這讓我們可以區分「程式根本沒通過編譯」與「程式跑起來了，但結果不符合測試」。

## 把檢查放在測試函式

`#[test]` 是寫在函式前面的**屬性**（attribute），告訴 Rust 這個函式是一個測試。`#[]` 是屬性的外框，`test` 是這項屬性的名稱，不是 TOML 的區段。此處測試函式沒有參數、回傳 unit，由 `cargo test` 產生的測試執行器呼叫，不需自己在 `main` 裡列出所有測試。

我們把測試收進 `mod tests { ... }`，這是 module 的同檔形式：花括號直接包住內容，和上節 `mod pricing;` 到另一檔案載入內容的形式不同。

module 前面的 `#[cfg(test)]` 表示只有在測試建置時才編譯這個 module。`cfg` 來自 configuration，用來按建置條件選擇程式部分；本次條件 `test` 由 Cargo 的測試工作啟用。正常 `cargo run` 不需要這些測試函式。

library 內的測試使用 `crate::pricing::...`，因為 `crate` 指向這個 library 的根；上節 binary 則用 `ch01_basics::...`，因為它是 library 之外的另一個 crate。兩邊最後呼叫的是相同的公開計算函式。

在 `src/lib.rs` 的 module 宣告後加入下面這段：

```rust
--8<-- "examples/ch01-basics/src/lib.rs:unit-tests"
```

先看測試材料怎麼回應運費規則：小訂單確定加 20；299 與 300 跨過免運邊界；數量零走提早回傳；兩組 array 檢查累積全部輸入。這些預期值是依規則算出的答案，不是把目前程式輸出直接抄回測試。

從 repo 根目錄執行：

```bash
cargo test -p ch01-basics --locked
```

測試執行器會列出測試名稱，四項測試應全部通過。它可能也列出 binary 或文件測試沒有測試項目的結果；這不會讓 library 的四項檢查失效。不要依賴列出名稱的順序，這些測試都各自呼叫函式，不使用其他測試留下的狀態。

### 讓測試抓一次有意義的錯誤

暫時把 `pricing.rs` 的免運條件由 `subtotal >= 300` 改成 `subtotal > 300`，再執行 `cargo test -p ch01-basics --locked`。299 的訂單仍正確，但恰好 300 的訂單會多收 20；`shipping_threshold_includes_300` 應失敗，實際值為 320，預期值為 300。

這個變化不會使型別出錯，所以 `cargo check` 仍可通過；測試提供的是規則與答案的檢查。完成觀察後把條件改回 `>=`，再次確認測試通過。測試只涵蓋這些材料及它們回應的規則，不代表已證明所有 `i32` 輸入都正確；訂單計價函式仍採用非負、小數值的輸入約定。

## 排版與常見問題也交給工具

`cargo fmt` 使用 `rustfmt` 整理縮排與排版，它會修改檔案。`cargo clippy` 使用 Clippy 檢查常見寫法問題；它的提醒可能指出不必要操作、容易混淆的寫法或潛在錯誤，不等於替你驗證運費政策。

`--all-targets` 表示檢查 package 的 library、binary、範例與測試等目標，不只主要程式。fmt 命令中的 `--` 把後面的選項交給格式化工具；`--check` 讓它只檢查排版是否符合規則，不修改檔案。這個 `--check` 是 rustfmt 的選項，和前面獨立的 `cargo check` 不是同一項工作。

```bash
cargo fmt -p ch01-basics
cargo fmt -p ch01-basics -- --check
cargo clippy -p ch01-basics --all-targets --locked
```

如果需要補裝工具，`rustup component add rustfmt clippy` 安裝工具鏈中的這兩個元件；教材固定工具鏈已經包含它們。工具沒有回報問題時，你仍需要根據任務選測試材料，不把「沒有警告」當成計算邏輯證明。

### 怎麼讀一則回饋

遇到編譯錯誤或工具提醒，先找檔案位置與訊息主句，再回到那個位置對照已教的規則。例如 1.4 的 immutable 第二次指派、1.5 的函式承諾 `i32` 卻交出 `()`，都是用規則定位原因。編譯器的建議可以幫你修改語法，卻不一定知道你原本想做哪個工作；先說清楚要保留的結果，再採用修改。

## 練習：300 是什麼的門檻

單價 140、數量 2 時，商品小計 280，加運費後總額 300。請新增測試，確認仍要收運費：門檻判斷的是**商品小計**，不是加運費後的總額。用 `assert_eq!` 分別確認小計 280 與總額 300；不要只檢查總額等於門檻。

參考答案的關鍵檢查如下；`#[test]` 與放入 `tests` module 的方法和上面相同。`goods` 與 `total` 分別是呼叫 `subtotal(140, 2)` 和 `total_with_shipping(140, 2)` 得到的數字：

```rust
--8<-- "examples/ch01-basics/examples/assertions_solution.rs:answer-checks"
```

也可以執行完整的 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch01-basics/examples/assertions_solution.rs)：`cargo run -p ch01-basics --example assertions_solution --locked`。如果兩項 assertion 都成功，它會顯示小計與完整總額。

你已經完成這一章的成果：一個 Cargo 管理的數值 library、呼叫它的 binary、明確的 module 分工，以及能抓到門檻修改錯誤的測試。接下來會把數個有角色的欄位放進自訂資料模型，並開始討論值由誰管理、什麼時候能交給另一段工作。
