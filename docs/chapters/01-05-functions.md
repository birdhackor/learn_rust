# 1.5 把計算變成函式

上一節的訂單總額是單價乘數量。若有兩筆訂單，我們可以寫兩次乘法；但當計算規則改變，就要記得找出每一處重複的做法。這一節要把同一項工作命名，明確列出它需要的材料與交出的答案。

## 函式的輸入與輸出

函式定義的一般形式是 `fn 名稱(參數名稱: 型別, ...) -> 回傳型別 { 內容 }`。這裡的 `...` 只是說明「可繼續列參數」，不是要輸入的 Rust 語法。

**參數**是函式收到輸入時使用的名稱，型別要明確寫出。**回傳值**則是函式交給呼叫位置的結果，`-> i32` 表示結果是一個 `i32`。呼叫時寫 `subtotal(120, 3)`，括號中的資料依位置對應到第一與第二個參數；呼叫者不需要和參數使用相同的變數名稱。

要看懂函式如何交出結果，還需要分辨兩種程式片段：**expression** 會算出一個值，例如 `price * count`；**statement** 完成一項宣告或工作，例如 `let total = ...;`。函式 body 是一個 **block**，也就是用 `{}` 圍起來的範圍。若最後放一個不帶分號的 expression，block 就交出它的值，作為函式回傳值。

不是每個函式都需要交出數字。Rust 用 **unit 型別 `()`** 表示「沒有額外內容的結果」，它只有一個值，也寫成 `()`。省略函式簽名的 `-> ...` 時，回傳型別就是 `()`；body 若沒有尾端值，則得到 unit。這仍然是一個型別，不能當成 `i32` 來相加。

下面的 `subtotal` 收到兩個 `i32`，回傳相乘的 `i32`；`show_total` 收到一個數字，只負責顯示，回傳 unit：

```rust
--8<-- "examples/ch01-basics/examples/functions.rs"
```

```bash
cargo run -p ch01-basics --example functions --locked
```

```text
總額=360
總額=160
```

先追蹤第一筆：`120` 對應 `price`、`3` 對應 `count`，相乘得到 `360`，它在呼叫位置成為 `first` 的初始值；再把 `first` 傳給 `show_total`。第二筆只是輸入換成 `80` 與 `2`，計算工作不需重寫。

`subtotal` 和 `show_total` 寫在 `main` 前面是為了閱讀；同一範圍的函式可以呼叫彼此，不要求像逐行執行的宣告那樣，一定先跑過函式定義。程式仍從 `main` 開始，定義函式本身不會立刻顯示東西。

## 分號會改變交出的結果

比較 `price * count` 和 `price * count;`：前者的值留給 block；後者完成運算後把結果丟掉，若後面沒有別的尾端值，block 交出 unit。

下面故意在計算後加上分號，放在 compile-fail fixture。簽名仍承諾交出 `i32`，body 卻得到 `()`，所以不能通過編譯：

```rust
--8<-- "examples/ch01-basics/fixtures/compile-fail/return_semicolon.rs"
```

錯誤重點是 `expected i32, found ()`：應得到整數，卻得到 unit。讀到這種型別不符訊息時，先對照函式承諾和 body 尾端，不要看到分號提示就把所有行末分號一起刪掉。`let` 宣告和完成顯示的工作仍以分號結束。

## block 也可以在函式內算出一個值

現在要在商品小計上加 20 元運費。我們可以在一個 block 裡先命名小計，再用尾端 expression 交出完整總額。`let total = { ... };` 最外面的分號結束變數宣告；block 內最後的 `subtotal + 20` 不加分號，才能把計算值交給 `total`。

block 也界定名稱的 **scope**，也就是名稱可以使用的程式範圍。裡面可以讀到外面的 `price` 和 `count`；裡面宣告的 `subtotal` 只在這個 block 中可用。外面仍能使用 block 交出的數字，因為它已經保存到 `total`。

```rust
--8<-- "examples/ch01-basics/examples/blocks.rs"
```

```bash
cargo run -p ch01-basics --example blocks --locked
```

結果是 380。這裡的兩種括號用途一致：都圈出一段工作；差別是內部 block 把最後的數字交回外層，而 `main` 完成顯示後回傳 unit。

## 練習：把運費也當成材料

寫一個 `with_shipping` 函式，接收單價、數量、運費三個 `i32`，回傳完整總額。先測兩筆資料：`120, 3, 20` 應得到 380；`80, 2, 20` 應得到 180。呼叫兩次同一函式，最後顯示兩個數字。

如果你發現自己只改數字，卻再複製一次函式內容，請回到輸入／輸出的分工：不同資料應從呼叫位置提供，同一規則留在函式裡。這不是說乘法不能重複寫，而是讓規則有一個容易修改的位置。

完成後可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch01-basics/examples/functions_solution.rs)，或執行 `cargo run -p ch01-basics --example functions_solution --locked`。
