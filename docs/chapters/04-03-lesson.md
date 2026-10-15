# 4.3 用 pattern 解構資料，命名需要的部分

上一節用 Amount(amount) 先確認形態，再把金額命名為 amount。第一章的 `let (price, count, ready) = order;` 也在做相近的事：按資料的形狀，替其中各部分建立名稱。這一節讓這些解構方式接成同一條規則。

## let 右側提供值，左側描述如何拆開

`let pattern = expression;` 先取得右側的值，再依左側 pattern 建立 binding。只有一個名稱的 `let price = 120;` 是最簡單的形式；tuple pattern 則把數個名稱放進相同的小括號形狀。

這次訂單 tuple 是 `(120, 3, true)`，只需要單價與數量：

`(price, count, _)` 依位置對應三格，建立 price=120、count=3，第三格用 `_` 忽略，不建立 ready 名稱。整個 pattern 的格數仍與 tuple 相符；忽略一格，不是把 tuple 改成兩格。

array pattern 用中括號。`[first, .., last]` 取得第一與最後一項，`..` 忽略中間未列出的其餘項目，因此 first=120、last=40。這裡 `..` 位於 pattern，是「忽略其餘」；第 2.3 節在 for 的 expression 裡用它建立 range，第 3.7 節則在索引中用它選區段，位置不同，用途也不同。

```rust
--8<-- "examples/intro-workspace/patterns/examples/destructure.rs:tuple-array"
```

一個 tuple 或 array pattern 裡，`..` 只能出現一次，位置也要讓保留的項目清楚對得上。例如首尾已經固定時，中間可全部略過；不能寫兩個 `..` 再期待 Rust 猜中間那個名稱要取得哪一項。

## struct 依欄位名對應，binding 可以另取名

Order 的兩個欄位已在第 3 章學過。pattern 也用 Order 加大括號，逐欄對應：

`unit_price: price` 的冒號左側是欄位名，右側是新 binding 名稱，因此 price 得到 80。`count` 是 `count: count` 的簡寫，新 binding 得到 2；列出欄位的順序不會交換它們的角色。

```rust
--8<-- "examples/intro-workspace/patterns/examples/destructure.rs:struct"
```

如果只需要單價，用 `..` 忽略沒有列出的其他欄位：

```rust
--8<-- "examples/intro-workspace/patterns/examples/destructure.rs:rest"
```

這次 unit_price 得到 200，沒有建立 count binding。本頁先拆解數字資料；String 欄位是否被 move 或借用，留到第 4.5 節用實際文字觀察。

### 完整的基本解構程式

本節各程式共用這份完整 manifest：

```toml
--8<-- "examples/intro-workspace/patterns/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/patterns/examples/destructure.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example destructure --locked
```

```text
tuple 的小計=360
array 的首尾=120、40
struct 的單價=80，數量=2
只取單價=200
```

## 參數與 for 也能用同樣的 pattern

函式參數名稱的位置也能放 pattern。例如參數收到一個 `(i32, i32)`，就在進入函式時把它拆成 price、count：

```rust
--8<-- "examples/intro-workspace/patterns/examples/pattern_places.rs:parameter"
```

簽名最外面的括號仍是參數清單，裡面的 `(price, count)` 則是**一個 tuple 參數**的 pattern；冒號後的 `(i32, i32)` 說明這一個參數的型別。呼叫要提供 `subtotal((120, 3))`，不是兩個分開的參數。

array 的每項也可以是相同型別的 tuple。下面的 orders 有兩項，每項都是 `(i32, i32)`；for 每次取得一個 tuple，再由 `(price, count)` 拆開：

```rust
--8<-- "examples/intro-workspace/patterns/examples/pattern_places.rs:for"
```

第一次的材料是 120、3，第二次是 80、2；加總的是兩筆小計 360+160=520。這沿用第 2.3 節的 array 走訪，只把 for 後原本的單一名稱換成 tuple pattern。

完整程式與執行如下，設定沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/patterns/examples/pattern_places.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example pattern_places --locked
```

輸出是參數解構的小計 360，以及兩筆小計合計 520。

## 外層是 enum，內層也可以是 struct

如果工作請求可能是「處理一筆 Order」或「取消」，可以定義 Request，把既有的 Order 放進其中一種 variant。建立時先建 Order，再將它放進 Request::Order；解構時也沿相同外框，由外向內對應：

```rust
--8<-- "examples/intro-workspace/patterns/examples/nested_patterns.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example nested_patterns --locked
```

第一筆符合 `Request::Order(Order { unit_price, count })`，所以內層兩個數字可用來算出 360。第二筆是 Cancel，只顯示取消，不嘗試尋找不存在的 Order。pattern 一層一層對到已有資料，不會因解構而多建立一筆訂單。

## 普通 let 的 pattern 必須能成功

tuple、array、struct 的上述解構，對該型別的每個值都能配對成功，稱為 **irrefutable pattern**。例如一個三格 tuple 一定有那三格，不能等執行後才突然變成兩格。

相對地，`Quote::Amount(amount)` 可能遇到 Missing 而失敗，稱為 **refutable pattern**。match 能在這個 arm 不符時繼續試下一個；普通 let、函式參數、for 的 pattern 則沒有這種失敗分支，所以必須使用不能配對失敗的 pattern。

下面完整例子不會通過編譯：

```rust
--8<-- "examples/intro-workspace/patterns/fixtures/compile-fail/refutable_let.rs"
```

在獨立練習專案執行 `cargo check`，錯誤 **E0005** 指出 `refutable pattern in local binding`。show 接受的是完整 Quote 型別，可能收到 Missing，不能無條件建立 amount。現在可以改用上一節的 match，為 Missing 安排明確工作；第 4.6、4.7 節會再教兩種適合單一成功形態的寫法。

## 練習與完整答案

把訂單 tuple 改成 `(80, 2, false)`，忽略第三格，將前兩格提供給剛才接收 tuple 的 subtotal，應得到 160。再從 `[80, 120, 200, 60]` 只取首尾並相加，應得到 140。最後建立相同單價與數量的 Order，將 unit_price 解構成 price，確認小計同樣是 160。

完整答案沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/patterns/examples/destructure_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example destructure_answer --locked
```

改資料後，pattern 的位置與欄位對應仍相同。現在已能取得需要的各部分，下一節再對它們加入範圍、組合與額外條件。

[下一節：pattern 的組合、範圍與 guard](04-04-lesson.md)
