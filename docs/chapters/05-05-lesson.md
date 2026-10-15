# 5.5 Vec 的 ownership：保留原資料，建立整理結果

上一節操作的是 i32，所以取出數字時容易看不見 ownership 的差別。這次先把 String 交進清單再取回，接著完成一個小工具：從借來的數列保留非負數，建立新的 Vec，維持原順序、重複次數及原資料。

## push 交入元素，pop 交回元素

`Vec<String>` 持有的是 owned String。push 以值接收元素，所以 `messages.push(message)` 會把 message 的文字交進清單，原 message 不能再用。pop 回傳 `Option<String>`；Some 中交回的是 owned 文字，清單不再持有被取出的那一項。

下面以第 4.7 節的 let else 取得 pop 的結果。Some(taken) 成功時，taken 持有 String；若清單為空，else 直接結束 main。顯示時再用 taken.as_str() 借讀，不需要另複製文字。

這份完整程式仍在沒有 library 的 data-steps，設定也是前面四節的三欄 manifest：

```toml
--8<-- "examples/intro-workspace/data-steps/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/data-steps/examples/vector_ownership.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data-steps --example vector_ownership --locked
```

```text
從清單取回=Rust
清單剩下的項目=0
```

String 的責任依序從 message 交進 messages，再交給 taken。整個 Vec 自己也沒有 Copy，即使元素全是 i32，指派整個 Vec 仍會 move。最後的 owner 結束時，Vec 會清理它仍持有的元素與存放區；String 元素需要的文字資源也按其型別規則清理。

## 元素 reference 還在使用時，不能增長清單

Vec 的 push 在空間不足時，可能取得較大的存放區，把元素交到新位置。若原本借出的元素 reference 還要繼續使用，就不能讓這份衝突的修改借用同時發生。

下面先以 get 借讀第 0 項，保存的是 `Option<&i32>`；push 後的 if let 還要使用裡面的 reference，因此借用區間重疊：

```rust
--8<-- "examples/intro-workspace/data-steps/fixtures/compile-fail/vector_borrow_overlap.rs"
```

這份完整失敗例會得到 **E0502**。Some 中包著 reference，仍受第 3.5 節的借用規則限制。不能依賴某次 push 恰好沒有換存放區；應先完成元素借讀的最後使用，再修改清單。

## 整理規則：只把非負數放進新結果

輸入是借來的 &[i32]，輸出是新的 `Vec<i32>`。規則是值至少為零就保留；負數略過，不將它改成零；原先重複的值仍保留原次數。

先用120、−5、0、80看逐項結果：

| 目前讀到 | 要做的事 | 目前保留的內容 |
| --- | --- | --- |
| 120 | push 進新 Vec | 120 |
| −5 | 略過 | 120 |
| 0 | push 進新 Vec | 120、0 |
| 80 | push 進新 Vec | 120、0、80 |

函式先建立空 Vec。position 從0開始，每次 while 先確認它小於輸入長度，才索引那一項；符合條件便 push，最後 position 加一。push 修改的是另一份結果清單，輸入只被借讀。

這個公開函式放在 list module；pub、回傳型別、while 與尾端交回值都沿用已教的安排：

```rust
--8<-- "examples/intro-workspace/data/src/list.rs:list-rule"
```

按原順序讀取，也按相同順序 push，所以結果保留順序。空輸入的 while 一次也不做，交回空 Vec；全負數仍逐項讀取，只是不 push。結果的長度由資料決定，沒有固定成三項。

## 呼叫者與測試各自使用同一份規則

完整版本放在另一個 member：`data/`，package 名為 **intro-data**。它的 library 公開 text 與 list，text 保留第 5.3 節已教的文字取得函式，list 提供本節整理。範例透過 `intro_data::list::keep_nonnegative` 使用它，沿用第 2.6 節的 library 路徑規則。

list_demo 借讀原 array，接收新的 Vec；顯示時先 as_slice，再按0..len()走訪有效索引。因為每個索引都小於長度，這裡不需要把已知有效的位置再包成查詢結果：

```rust
--8<-- "examples/intro-workspace/data/examples/list_demo.rs"
```

測試另選80、−5、80、0，確認兩個80都保留、零保留、原第二項仍是−5；再確認空輸入與全負數都得到空結果。空 array 的型別寫 `[i32; 0]`，值寫 `[]`，仍是第 1.8 節的元素型別／固定項數語法，只是這次項數為0。

長度的usize相等條件交給assert!；取出的i32則用assert_eq!，不要求直接顯示或比較整個Vec。完整list測試如下：

```rust
--8<-- "examples/intro-workspace/data/src/list.rs:list-tests"
```

## 完整設定與所有來源

目前這個版本的text、list都已教完。下列列出member設定、兩個root、兩份module；前面已列出完整list_demo。沒有依賴或其他隱藏helper。

`data/Cargo.toml`：

```toml
--8<-- "examples/intro-workspace/data/Cargo.toml"
```

`data/src/lib.rs`：

```rust
--8<-- "examples/intro-workspace/data/src/lib.rs"
```

`data/src/main.rs`保留簡單文字入口；本頁計算使用上面的list_demo target：

```rust
--8<-- "examples/intro-workspace/data/src/main.rs"
```

??? example "展開完整 text 與 list，包含全部六項測試"

    `data/src/text.rs`與第 5.3 節的 text-only 版本相同，包含四項文字測試及已說明的局部 manual_map allow：

    ```rust
    --8<-- "examples/intro-workspace/data/src/text.rs"
    ```

    `data/src/list.rs`包含完整整理函式與兩項清單測試：

    ```rust
    --8<-- "examples/intro-workspace/data/src/list.rs"
    ```

從repo根目錄執行完整呼叫者，再跑library測試：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data --example list_demo --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-data --lib --locked
```

```text
保留的項目=3
索引0=120
索引1=0
索引2=80
```

library共六項測試：四項文字契約、兩項清單契約。這些是保存選定材料的規則檢查，不是證明所有未來清單工作都正確。

## 練習：保留後，再套用上限

寫 `capped_nonnegative(values: &[i32], ceiling: i32) -> Vec<i32>`。本題約定ceiling非負：負數略過，其餘值若大於ceiling，就在結果中改成ceiling。保留原順序、重複次數與輸入資料。

用Vec保存120、−5、0、80，借它的as_slice傳入函式，以ceiling=100得到100、0、80；原清單第一項仍應是120。先略過負數，再限制保留值；若把負數也變成零，就改變了本題規則。

完整答案沿用本頁intro-data設定。它自己定義練習函式，不改動library原本的keep_nonnegative：

```rust
--8<-- "examples/intro-workspace/data/examples/list_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data --example list_answer --locked
```

```text
索引0=100
索引1=0
索引2=80
```

第5章現在把共用資料形狀、完整結果型別、文字範圍與可增長清單接起來。下一章會用trait明確表達型別具備哪些能力，讓顯示、比較與複製的契約也能寫進型別安排。

[下一節：trait、impl 與 trait bound](06-01-lesson.md)
