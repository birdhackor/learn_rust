# 第一個 Rust 程式

## 學習目標

- 理解 main 是程式的執行起點。
- 使用 println! 顯示訊息。
- 用 Cargo 編譯並執行範例。

## 程式碼

```rust title="examples/hello-world/src/main.rs" linenums="1"
--8<-- "examples/hello-world/src/main.rs"
```

fn main() 宣告 main 函式，函式內的程式碼位於大括號之間。

let language = "Rust"; 建立名為 language 的變數。println! 是巨集；它將字串中的 {language} 替換成變數的值，再輸出一行文字。

## 執行與結果

在儲存庫根目錄執行：

```bash
cargo run -p hello-world --locked
```

程式的輸出為：

```text
Hello, Rust!
```

Cargo 第一次執行時會先編譯程式，之後可重用未改動的建置結果。

## 練習

1. 把 "Rust" 改成自己的名字，重新執行並觀察輸出。
2. 試著移除 let 那一行結尾的分號，閱讀編譯器訊息。
3. 完成後還原範例，再執行 cargo test -p hello-world --locked 驗證教材的預期輸出。

接著閱讀 [變數與型別](../fundamentals/variables.md)。
