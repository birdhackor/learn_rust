# 9.7 比較與排序：產生順序明確的報表

上一節已得到正確的名稱次數，但 HashMap 的走訪順序沒有保證。現在要每次都顯示同一份報表：次數由多到少，同次數再按名稱由小到大。

Rust 兩次、Python 與教學各一次，應依序是 `Rust=2`、`Python=1`、`教學=1`。分行、空行與名稱身分規則都沿用上一節；這次增加的是一份完整的比較規則。

每個 example 都保存自己的完整 helper，沒有 library 或其他章程式。完整設定仍為：

```toml
--8<-- "examples/intro-workspace/iterators/Cargo.toml"
```

## 比較不只回答 true／false：Ordering、PartialOrd 與 Ord

排序需要知道兩項資料的比較結果是較小、相等，還是較大。標準 enum **`std::cmp::Ordering`** 有三個 variant：**Less、Equal、Greater**。它們表示第一個比較對象相對第二個的順序；這個型別有 Debug／PartialEq，可顯示與核對結果。

**`PartialOrd`** 提供部分順序的比較，`partial_cmp` shared 借用兩個值，回傳 `Option<Ordering>`。能比較就是 Some 包住相應順序，不能比較就是 None；None 不等於 Equal。先前的 `<`、`<=` 等操作也由這項能力支持，判斷必須和 PartialEq 的相等規則一致。

**`Ord`** 進一步承諾所有同型別值都能依一致的完整順序比較，也要符合 Eq。它的 `cmp` shared 借用兩個值，直接交回 Ordering。反轉左右，Less 應變 Greater；相等對應 Equal，連續的小於關係也要一致。排序時不能依 callback 第幾次執行任意改變答案。

i32／usize 的 Ord 採數值順序，str／String 採內容 bytes 的字典順序：從第一個不同的位置判斷，前綴相同則短的先。這保留大小寫與內容差異，不是依台灣語言習慣作字典排序。Ord／PartialOrd 已在 prelude，Ordering 則以 `use std::cmp::Ordering;` 引入。

浮點數則有 **NaN**（Not a Number，非數值），不能和一般值取得這種順序。**`f64::NAN`** 是標準 f64 提供的 associated constant，即型別提供的固定值，直接用 `::` 讀取，不加呼叫括號；本例用它觀察 None。一般 f64 因這種情況沒有 Ord，不能只因能用 `<` 就假定它滿足完整排序契約：

```rust
--8<-- "examples/intro-workspace/iterators/examples/partial_order.rs"
```

執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example partial_order --locked`，一般浮點比較是 Some(Less)，NaN 比較是 None。若要為浮點資料排序，必須另外決定 NaN 的位置與比較規則；本節報表的名稱與 usize 次數已具備 Ord，沒有這項未定政策。

## sort、sort_by_key 與 sort_by 各交給誰決定順序

slice 的 sort、sort_by_key、sort_by 都需要 mutable borrow，修改原排列，回傳 `()`；空或一項資料也能處理。它們保留比較相等項目的原相對順序，這叫 **stable sort**（穩定排序）。

| method | 如何決定順序 | callback 契約 |
| --- | --- | --- |
| `sort()` | 元素自己的 Ord | 沒有 callback |
| `sort_by_key(callback)` | callback 取得排序鍵，鍵要有 Ord | `FnMut(&T) -> K` |
| `sort_by(callback)` | callback 直接比較兩項 | `FnMut(&T, &T) -> Ordering` |

callback 可能被多次呼叫，不保證每個元素只呼叫一次。本節將 Vec 明確借成 as_mut_slice，再使用這些 slice API。

下面的 tuple 第一欄當排序鍵，第二欄只用來觀察同鍵資料是否維持輸入順序。標準 tuple 各欄位有 Debug／PartialEq 時，也提供相應的整份顯示／逐欄比較能力，因而 Vec 的 Debug 能列出這些 tuple：

```rust
--8<-- "examples/intro-workspace/iterators/examples/sorting_keys.rs"
```

執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example sorting_keys --locked`，numbers 是 `[1, 1, 2, 3]`；rows 是 `[(1, 30), (1, 10), (2, 20)]`。第一欄相同的兩筆保留 30 在 10 前，沒有偷偷把第二欄也當排序鍵。

### 不需要等值項目原序時，sort_unstable 也是一種選擇

對只有數字的清單，這項工作只要升序結果，不再按原位置識別相等的數字，可以選 **sort_unstable**。它仍需要 mutable 借用 slice，按元素的 Ord 升序修改原排列，回傳 unit；排序工作本身不另配置 heap 暫存區，但不承諾相等項目的原相對順序。

```rust
--8<-- "examples/intro-workspace/iterators/examples/sorting_unstable.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example sorting_unstable --locked
```

結果仍是 `[1, 1, 2, 3]`。這裡兩個1沒有附帶不同身分，沒有需要觀察誰先誰後；若像前面 rows 一樣要保留同鍵項目的原順序，就選 stable 版本。不承諾保序也不表示每次執行一定會重新排列相等項目。

sort_by 要回傳 Ordering，不能只交回 bool。這個失敗範例以 `E0308` 指出輸出不符；完整順序還要區分兩項相等：

```rust
--8<-- "examples/intro-workspace/iterators/fixtures/compile-fail/sort_boolean.rs"
```

## 排出完整報表：先次數，再名稱

回到開頭的規則：次數降序，同次數才比名稱升序。先用 owned into_iter，把 map 的 `(String, usize)` 收成 `Vec<(String, usize)>`；排序 callback 借來的是兩筆 tuple，不會接手它們的 String。

tuple 的 `.0` 是名稱、`.1` 是次數。要讓次數大的左資料先出現，先比較 **`right.1.cmp(&left.1)`**。若左是 Rust 的 2、右是 Python 的 1，1 比 2 得到 Less，排序就把**左筆 Rust**放在右筆之前。次數 Equal，才用名稱正常的左右方向比較：

| 左筆／右筆 | 比較結果 | 報表順序 |
| --- | --- | --- |
| Rust 2／Python 1 | 先比右次數 1 對左次數 2，Less | Rust 在 Python 前 |
| 教學 1／Python 1 | 次數 Equal，再比名稱，Greater | Python 在教學前 |

```rust
--8<-- "examples/intro-workspace/iterators/examples/text_report.rs:sort-counts"
```

只按次數排序，Equal 的名稱會保留來源順序；但 HashMap 的來源順序是任意的。因此**stable 不足以保證這份報表每次順序相同**。補上名稱比較後，不同的 map 鍵都有明確位置，這才滿足固定輸出的需要。

把分行／計數／排序接成小函式，回傳 owned 的報表資料。完整檔同時列出上一節的 nonempty_names、count_names；後面的 assertion 另以不同插入順序核對相同報表，並確認來源結束後結果仍可用。名稱已由 to_owned 保存在 map，現在移入結果 Vec，所以原文字結束後，報表仍然有效：

```rust
--8<-- "examples/intro-workspace/iterators/examples/text_report.rs:report"
```

```rust
--8<-- "examples/intro-workspace/iterators/examples/text_report.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example text_report --locked
```

```text
Rust=2
Python=1
教學=1
```

## 練習：較少出現的名稱先顯示

重用 count_names，把報表改為**次數由少到多，同次數仍依名稱升序**。先說清楚哪個比較方向要改，不修改分行、空行或名稱身分規則，也不要為排序 clone 整份名稱。

Rust、Python、Rust、教學 應得到 Python=1、教學=1、Rust=2。使用本頁已說明的 tuple／Vec 比較，或用逐欄 assertion 核對，並再次確認空文字沒有報表資料。

下面是本頁獨立的完整答案，含 nonempty_names、count_names、rare_first 與 main；沒有從另一個檔案取得 helper，設定沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/iterators/examples/text_report_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example text_report_answer --locked
```

```text
Python=1
教學=1
Rust=2
```

次數改成 `left.1.cmp(&right.1)`，因此1在2之前；同為1時仍正常比較名稱，Python 在教學之前。各名稱由 map move 進結果 Vec，沒有為了排序 clone 文字；空輸入則交回空報表。

本章已把逐筆資料型別、borrowed／owned 選擇、錯誤政策和排序接成可驗證的整理流程。下一章再替這份核心工作接上 Cargo 生態系、檔案與命令列入口。

延伸閱讀：[PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)、[Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)、[slice 排序](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by)、[sort_unstable](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable)。
