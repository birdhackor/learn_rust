# 4.6 用 if let 與 while let 處理一種形態

有些工作要分別處理所有形態，match 會要求我們完整列出答案。有些工作則只是「有金額就顯示，沒有就略過」，或「只要還取得下一批工作，就繼續處理」。這一節讓單一 pattern 直接表達這兩種需要。

## if let 符合才執行，不符就略過

這裡先把 Quote 保留為 Amount(i32) 與 Missing，只觀察有沒有金額。**`if let pattern = expression { ... }`** 先算右側，再以左側 pattern 配對；成功才進入 body，pattern 建立的名稱也只在成功的 body 中使用。不成功時跳過 body，接著做 if let 後面的工作。

本例的右側是 &Quote，所以 Amount(amount) 取得 &i32，沿用上一節的自動借用規則。它與 `match` 的 Amount arm 顯示金額、其他情況不做事，是同一項工作。若還要在不符時做事，可以加 `else`，把所有不符情況送到那個 block。

```toml
--8<-- "examples/intro-workspace/patterns/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/patterns/examples/if_let.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example if_let --locked
```

```text
match 讀到金額=120
if let 讀到金額=120
這筆沒有金額
```

對 Missing 呼叫 show_with_if_let，沒有多印一行；最後那行來自有 else 的 show_or_missing。if let 適合刻意只關心一種形態的工作，它不會要求你為其他 variant 分別安排 arm。若 Missing、Rejected 本來需要不同處理，就應保留 match 的完整分類。

## 先準備會更新狀態的下一批函式

第 2.4 節曾把 7 項工作分成每批最多 3 項。現在定義 enum Batch，讓每次取得的結果也是資料：More(i32) 表示這次有一批，附帶處理量；Done 表示已經沒有工作。

本節輸入是非負小整數。next_batch 接受 &mut i32，能更新呼叫者保存的剩餘量：已是 0 就交出 Done；否則選最多 3 項、先扣掉這次處理量，再交出 More(processed)。這裡的 helper 完整定義如下：

```rust
--8<-- "examples/intro-workspace/patterns/examples/while_let.rs:next"
```

這個函式每呼叫一次才取得一批。它沒有保存一列待取出的資料，也沒有使用新的容器或其他 API；剩餘量就是呼叫者的那個數字。

## while let 每次重新取得，再配對

**`while let pattern = expression { ... }`** 在每次進 body 前重新計算右側。若 pattern 符合，就建立本次 binding 並做一次 body；body 結束後再算右側、再配對。不符合時，直接結束 while let。

因此下面每次都呼叫 next_batch，符合 More 時才建立 processed。當函式回傳 Done，body 不執行，沒有再多算一批：

```rust
--8<-- "examples/intro-workspace/patterns/examples/while_let.rs:loop"
```

| 呼叫前剩餘量 | 回傳與後續動作 | 呼叫後剩餘量 |
| --- | --- | --- |
| 7 | More(3)，執行 body | 4 |
| 4 | More(3)，執行 body | 1 |
| 1 | More(1)，執行 body | 0 |
| 0 | Done，不進 body，結束 | 0 |

`processed` 只在本次 body 中使用；remaining、batches 則由外層保存，跨次更新。迴圈能停止，是因為 next_batch 讓正的剩餘量持續減少，最後交出 Done；while let 本身不會替一個永遠回傳 More 的函式製造停止條件。

### 完整程式與執行

下面是 enum、helper、迴圈與 main 的全部來源，設定沿用同頁 manifest：

```rust
--8<-- "examples/intro-workspace/patterns/examples/while_let.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example while_let --locked
```

```text
本批處理3，剩餘4
本批處理3，剩餘1
本批處理1，剩餘0
七項共3批，零項共0批
```

零項工作第一次就取得 Done，所以沒有再顯示處理一批。這與前面的 if let 相同，都有「不符合就不進 body」的路徑；差別是 while let 會在每次 body 後重新取得結果。

## 練習與完整答案

把每批上限改成參數 capacity，仍讓下一批函式回傳 Batch。要求 work 非負、capacity 為正；若上限為零，剩餘量就可能一直不變。用 `(7, 3)`、`(6, 3)`、`(0, 3)` 確認分別得到 3、2、0 批。

完整答案沿用本頁 manifest，並在開始工作前以 assertion 檢查這兩項輸入約定：

```rust
--8<-- "examples/intro-workspace/patterns/examples/while_let_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example while_let_answer --locked
```

最後一批仍只取剩餘量。已能在 pattern 不符時略過或停止，下一節再把「失敗先離開，成功名稱留給後面」直接寫成 let else。

[下一節：let else](04-07-lesson.md)
