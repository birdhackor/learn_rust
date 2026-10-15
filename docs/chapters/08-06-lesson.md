# 8.6 ? 與錯誤傳遞：提早回傳，讓呼叫者決定後續

第 8.5 節把文字解析分成「取得整數」與「驗證件數」。任何一步失敗，都把具體原因交給呼叫者。現在縮短這項重複的錯誤傳回工作，同時保留兩件事：第一個失敗發生後不做後面的步驟，呼叫者仍可選擇報告後繼續下一筆。

## 先看完整的手動 match

`parse_count_manual(text: &str) -> Result<Count, ParseCountError>` 借讀文字，成功交回 Count，失敗交回前節定義的兩種原因。第一個 match 只在整數成功時取得 value；第二個只在件數成功時取得 count。Err 分支都以 `return Err(...)` **提早離開這個函式**，不是停止呼叫者或整個程式：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/propagation/propagation/mod.rs:manual-propagation"
```

成功走完兩個步驟，才執行最後的 Ok(count)。在此函式不選報告政策，而是把資訊傳回呼叫者，稱為**錯誤傳遞**。

## Result 的 ? 取值或提早回傳

**`?`** 是放在結果後面的運算子。對這裡的 Result，它做兩種工作：

| 取得的結果 | `?`的工作 | 函式接著做什麼 |
| --- | --- | --- |
| `Ok(value)` | 交出裡面的 value | 繼續後面的步驟 |
| `Err(error)` | 以 From 轉成目前函式的錯誤型別，將 Err 提早回傳 | 不執行此函式餘下步驟，回到呼叫者 |

因此使用前要先讀**目前函式的回傳型別**，並確認來源錯誤能轉成它需要的型別。上一節已實作兩份 From：ParseIntError 放進 Integer，CountError 放進 Range；所以這裡能回傳同一種 ParseCountError。

下面是相同工作的縮寫版。`i32::from_str(text)?` 在成功時交出i32，`Count::try_from(value)?`再交出Count；尾端的Ok(count)仍須寫出：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/propagation/propagation/mod.rs:question-propagation"
```

| 輸入 | 兩個 `?` 依序做什麼 | 整個 helper 的結果 |
| --- | --- | --- |
| `"3"` | 第一個取得3；第二個取得Count | Ok中的有效件數3 |
| `"abc"` | 第一個整數失敗，轉成Integer並早退；第二個不執行 | Err中的Integer原因 |
| `"101"` | 第一個取得101；第二個件數失敗，轉成Range並早退 | Err中的Range原因 |

縮寫減少的是「遇到錯誤就轉換並回傳」的重複match，沒有改錯誤分類，也沒有自動列印訊息或呼叫panic。

## helper 早退後，main 仍可繼續

這個完整呼叫者同時執行手動版與?版，先比較兩份Result，再決定如何報告。它的target根載入本章已教完的六個完整module；全部來源都列於本頁後面。

```rust
--8<-- "examples/intro-workspace/error-stages/examples/propagation/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example propagation --locked
```

```text
0：件數 0
3：件數 3
100：件數 100
abc：無法解析整數：invalid digit found in string
-1：件數不符規則：件數 -1 不能為負數
101：件數不符規則：件數 101 超過批次上限100
呼叫者已決定報告失敗並繼續下一筆
```

abc使這一次parse_count早退；main的match取得Err後，回報原因、繼續−1與101，最後一句也有執行。每個函式處理的是自己的回傳；?沒有替整批資料決定停止政策。

## 回傳型別與 From 條件都要成立

下面是完整的預期編譯失敗程式。這份main沒有宣告結果回傳型別，原本交回unit `()`，卻想用?把解析失敗傳出去：

```rust
--8<-- "examples/intro-workspace/error-stages/fixtures/compile-fail/question_in_unit.rs"
```

放進第 1.2 節的獨立練習專案執行`cargo check`，得到 **E0277**。本章把可能失敗的步驟放在回傳相容Result的helper，main再用match選政策，不把?直接放進這種unit回傳的main。

第二份完整fixture已回傳Result，但錯誤型別是CountError。整數解析的來源錯誤是ParseIntError，程式沒有提供`From<ParseIntError> for CountError`，因此無法在這個?處轉換：

```rust
--8<-- "examples/intro-workspace/error-stages/fixtures/compile-fail/question_without_conversion.rs"
```

同樣得到 **E0277**。要依實際工作設計錯誤型別與轉換，例如前節ParseCountError的Integer／Range，而不是只看見函式寫Result就認為所有錯誤都能傳入。

## Option 的 ?：缺值也可提早回傳

`?`也可用於這種回傳Option的函式：Some時取出內容，None時從目前函式提早回傳None。這裡沒有Err或錯誤轉換；函式簽名必須允許這項缺值回傳。

`values.get(1)`的結果是`Option<&i32>`。Some時?交出reference，`*value`讀取數字，再用Some交回owned i32；None時直接交回None，不執行後面的Some：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/option_question.rs:second-number"
```

這份例子自足，不載入件數解析模組。完整程式如下，空array的`[i32; 0]`沿用第 5.5 節表示零個元素的寫法；整份Option的Debug顯示已在第 6.2 節教過：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/option_question.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example option_question --locked
```

```text
零筆：None
一筆：None
兩筆：Some(20)
```

零筆、一筆都沒有索引1，兩筆才取得20。None沒有附帶原因，也不會因為外層想要Result，就自動轉成任意Err。若需要某種錯誤原因，先明確match缺值並建立那個錯誤。

## 完整來源與三條路徑測試

全章仍使用no-lib的intro-error-stages。完整member設定：

```toml
--8<-- "examples/intro-workspace/error-stages/Cargo.toml"
```

下面是propagation target完整的propagation/mod.rs：兩份解析函式、第二筆查詢，以及兩項測試。第一項逐筆比較手動／?版，涵蓋成功、整數失敗、件數失敗；第二項比較缺值與Some(20)：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/propagation/propagation/mod.rs"
```

前面已列完整main；其餘五個module也都已教完。以下是這個target真正載入的完整來源，parsing測試保留已教的block尾端owned結果：

??? note "propagation target 的其餘五份完整 helper"

    `aliases/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/propagation/aliases/mod.rs"
    ```

    `policy/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/propagation/policy/mod.rs"
    ```

    `formatting/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/propagation/formatting/mod.rs"
    ```

    `conversions/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/propagation/conversions/mod.rs"
    ```

    `parsing/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/propagation/parsing/mod.rs"
    ```

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example propagation --locked
```

這個target共十二項測試，含前幾節十項與本節兩項。早期targets仍保留自己的完整快照，不因這裡加入?而改變載入內容。

## 練習與完整答案

寫`count_pair(first: &str, second: &str) -> Result<(Count, Count), ParseCountError>`，依序呼叫parse_count解析兩筆，兩筆都成功才交回tuple。若第一筆失敗，不解析第二筆；若第二筆失敗，不交回只有第一筆的半份結果。

這裡呼叫parse_count取得的錯誤已是ParseCountError。標準函式庫提供**相同型別到自己的From轉換**，原樣交回值；因此?不需你再寫一份`From<ParseCountError> for ParseCountError`。

先用`"0"`、`"100"`檢查兩項成功值；再用`"abc"`、`"101"`，應保留第一筆的Integer原因；最後用`"3"`、`"101"`確認第二筆的Range／TooLarge(101)。完整答案的`let (first, second) = ...`沿用第 4.3 節tuple解構，先取得兩個Count才各自讀值：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/propagation_answer/main.rs"
```

??? note "答案實際載入的全部六份完整 helper"

    `propagation/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/propagation_answer/propagation/mod.rs"
    ```

    `aliases/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/propagation_answer/aliases/mod.rs"
    ```

    `policy/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/propagation_answer/policy/mod.rs"
    ```

    `formatting/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/propagation_answer/formatting/mod.rs"
    ```

    `conversions/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/propagation_answer/conversions/mod.rs"
    ```

    `parsing/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/propagation_answer/parsing/mod.rs"
    ```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example propagation_answer --locked
```

```text
第一筆解析失敗，整個函式已回傳
兩筆都成功才回傳 tuple，失敗保留原原因
```

本章把驗證、錯誤資料、顯示與傳遞分開：下層交回明確結果，呼叫者依自己的工作選擇回報、繼續或刻意停止。下一章再把小段工作保存成closure，逐筆整理資料。

[下一章：closure](09-01-lesson.md)
