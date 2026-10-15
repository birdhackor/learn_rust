# 選讀：Python、JavaScript 與 Rust 的可變性

本頁接在第 3.8 節之後。若你熟悉 Python 或 JavaScript，可以用三個分開的問題對照：名稱能否重新指派、物件內容能否修改、目前這份借用允許什麼操作。主文不依賴本頁。

## Python：名稱重新綁定與物件修改

Python 的指派讓名稱綁到物件；一般名稱之後可以重新綁定。物件本身則依型別有不同規則：list 可以修改內容，str 不可以修改既有字串內容。這裡的 mutable 或 immutable 描述物件，不是名稱能否重新指派。

下面是完整程式。`[1, 2]` 建立 list；append 在同一份 list 的末尾追加元素。把 items 指派給 alias，兩個名稱會指到同一物件，並沒有 Rust 的 ownership move。

```python
--8<-- "examples/intro-workspace/ownership/fixtures/comparisons/names_and_objects.py"
```

可在 repo 根目錄執行：

```bash
python3 examples/intro-workspace/ownership/fixtures/comparisons/names_and_objects.py
```

```text
[1, 2, 3]
[1, 2, 3]
[9]
Rust
Rust 教學
```

前三行區分兩件事：`alias.append(3)` 修改兩個名稱共同指到的 list；`alias = [9]` 則讓 alias 改綁另一份 list，items 仍指到原本的那份。

最後兩行中，Python str 不可原地修改。`name += " 教學"` 得到串接後的字串，再讓 name 綁到結果；old_name 仍綁到原字串。不能把 `+=` 一律解釋成修改原物件：它的行為還要看物件型別。這與 Rust 的 `let mut` 分屬不同規則，不能把 Python immutable str 直接翻成 Rust immutable binding。

## JavaScript const：binding 固定，物件屬性仍可變

JavaScript 的 const 宣告要求初始化，之後不能把該 binding 重新指派成另一個值。若值是 object，const 本身不會禁止修改 object 的屬性。

下面的 `{ count: 2 }` 建立有 count 屬性的 object。alias 與 order 指到同一個 object；`alias.count += 1` 改的是這個 object 的屬性。 `console.log(...)` 將括號內的值顯示到終端。

```javascript
--8<-- "examples/intro-workspace/ownership/fixtures/comparisons/const_binding.js"
```

若已安裝 Node.js，可以執行：

```bash
node examples/intro-workspace/ownership/fixtures/comparisons/const_binding.js
```

結果是 `3`。const 不允許把 order 重新指派成另一個 object，但本例完全沒有重新指派 order。MDN 有時以「immutable reference」描述 const 的 binding；這裡的 reference 用語不是 Rust 的 shared reference `&T`，不能由此推論成 Rust 的借用權限。

## Rust：binding 與借用權限各自有作用

第 3.4 節已把這兩個問題分開：名稱前的 `mut` 允許重新指派 binding；`&mut` 提供經由借用修改所指資料的權限。修改 owned String 的方法，需要取得它的修改借用，所以持有者 text 宣告成 mut。

這份完整 Rust 程式沿用第 3.8 節的追加工作：

```toml
--8<-- "examples/intro-workspace/ownership/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/ownership/examples/mutability_comparison.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example mutability_comparison --locked
```

```text
同一份文字=Rust 教學
借讀視圖=Rust 教學
借讀完成後=Rust 教學，完成
```

reference 這個 binding 沒有 mut，也能提供 append 所需的修改借用，因為它持有的是 &mut String。它沒有換成另一份 reference，修改的是借來的 String。接著 view 是 &str，只借讀；main 在最後一次使用 view 之後才再追加文字，避免與仍在使用的 shared 借用衝突。

| 要問的事 | Python 本例 | JavaScript 本例 | Rust 本例 |
| --- | --- | --- | --- |
| 名稱能否重新指派？ | items、alias、name 可以 | const binding 不可以 | text 有 mut；reference 沒有 mut |
| 內容能否修改？ | list 可以；既有 str 不可以 | object 的 count 可以 | String 可經由修改借用追加 |
| 另一個名稱／reference 帶來什麼規則？ | alias 可共同指到同一 list | alias 可共同指到同一 object | &mut 提供獨占修改借用；&str 提供 shared 借讀，使用區間受檢查 |

因此讀到「不可變」時，先確定它指的是哪一個對象。Rust 在本章處理的普通數字、訂單與 String，還要進一步檢查借用之間的存取是否衝突；單靠另一個語言的 immutable 或 const 名稱，推不出這個檢查結果。

資料依據：Python 官方 [assignment statements](https://docs.python.org/3/reference/simple_stmts.html#assignment-statements)（[繁中](https://docs.python.org/zh-tw/3/reference/simple_stmts.html#assignment-statements)）、[objects, values and types](https://docs.python.org/3/reference/datamodel.html#objects-values-and-types)（[繁中](https://docs.python.org/zh-tw/3/reference/datamodel.html#objects-values-and-types)）；MDN [const](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/const)（[繁中](https://developer.mozilla.org/zh-TW/docs/Web/JavaScript/Reference/Statements/const)）。

[回到第 3.8 節](../chapters/03-08-lesson.md)
