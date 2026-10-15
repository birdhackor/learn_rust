# 8.2 type alias：為既有型別取另一個名字

上一節的驗證函式交回 `Result<i32, CountError>`。同樣型別若出現在數個函式簽名，重複寫完整名稱會把每個函式的工作擠到後面。現在希望替這個已知結果型別取一個短名稱，保留原有成功、失敗與 ownership 規則。

## type 宣告的是型別名稱

**type alias**（型別別名）是既有型別的另一個名稱。`type CountResult = Result<i32, CountError>;` 讓 CountResult 表示右邊那個型別；`type` 宣告型別名稱，不是在執行時建立一份值。

`pub type` 讓其他 module 也能使用這個名稱。本例在 aliases module 取名字，接著讓 `count_with_alias` 使用它作回傳型別。它仍呼叫同一個 checked_count，成功與失敗的規則都保留：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/aliases/aliases/mod.rs"
```

這和第 6.4 節 trait 內的 `type Output;` 位置不同：那裡留下 associated type，由各份 impl 指定；這裡在 module 裡直接宣告一個已知型別的別名。

## 兩個名稱表示同一個型別

下面的根檔案以 `pub mod aliases;`、`pub mod policy;` 載入各自同層資料夾內的 mod.rs。所有載入來源的完整內容都列於本頁。

`short_name` 使用 CountResult，`original_name` 使用完整的 `Result<i32, CountError>`，可以直接把前者交給後者，不需轉換。這份結果沒有 Copy，指定會 move；後面只使用接收者 original_name，沒有因取別名而多複製一份結果：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/aliases/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example aliases --locked
```

```text
兩種型別名稱都表示=Ok(3)
```

assertion 和 Debug 顯示沿用第 8.1 節的 Result 契約。別名的用途是讓簽名用 CountResult 表達「件數驗證結果」，減少同一串型別的重複；它沒有把結果解析或轉換成另一種資料。

## 別名不建立新型別，也不增加驗證

若替 i32 取名 CountNumber，−1 仍是合法的 i32 值。名稱不會把前節 0 到 100 的件數規則加入型別，也不會自動呼叫 checked_count：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/alias_is_name.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example alias_is_name --locked
```

```text
alias沒有自動驗證，數字仍是=-1
```

這裡能直接交給普通 i32，正因兩個名稱表示同一型別；沒有另一個編譯檢查替你拒絕負數。需要驗證時，仍要實際執行驗證工作。

## 完整設定與載入來源

這些 targets 屬於沒有 library 的 intro-error-stages。完整 member manifest：

```toml
--8<-- "examples/intro-workspace/error-stages/Cargo.toml"
```

前面已列 aliases 的完整根檔案與完整 aliases/mod.rs；下面是它實際載入的完整 policy/mod.rs。內容沿用第 8.1 節，包含兩項邊界／失敗原因測試；沒有先載入後面的 Display 或轉換實作：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/aliases/policy/mod.rs"
```

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example aliases --locked
```

## 練習與完整答案

宣告 `PriceAndCount`，表示第 1.8 節已教的 `(i32, i32)` tuple；第一項為單價，第二項為數量。寫 subtotal 接收這個名字表示的型別，交回相乘結果。材料只用不會溢位的非負小整數。

用單價 80、數量 3 得到 240。先把同一份資料交給普通 `(i32, i32)` 變數，再傳給 subtotal，確認兩種名稱不需要轉換。`.0` 與 `.1` 仍依已教的 tuple 位置讀值；別名不會替這兩個位置新增欄位名稱。

下面是完整自足答案，沿用本頁 manifest，不載入其他 module：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/alias_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example alias_answer --locked
```

```text
使用另一個名稱後，小計=240
```

名稱只改善型別的閱讀方式；tuple 各位置的角色仍由接口說明。下一節將遇到標準函式庫的 `fmt::Result`，它同樣是已知 Result 型別的別名。

[下一節：Display 與格式化](08-03-lesson.md)
