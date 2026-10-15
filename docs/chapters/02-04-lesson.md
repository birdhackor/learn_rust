# 2.4 用 while 依條件繼續工作

現在有 7 項工作，每一批最多處理 3 項，最後一批只處理剩下的部分。我們想每批完成後重新查看剩餘量；只要還有工作，就再做一批，直到剩餘量為零。

## 每次進 body 前先檢查

`while 條件 { 工作 }` 在每次執行 body **之前**檢查條件。為 true 就做一次，body 結束後回到條件重新檢查；為 false 則離開 while，接著執行後面的程式。如果一開始已是 false，body 一次也不做。

這次用 `remaining > 0` 表達「還有工作」。`remaining` 必須是 mutable，才能在每批完成後減少。批次數 `batches` 也在 while 外建立，保存到目前完成了幾批。

```rust
--8<-- "examples/intro-workspace/control-steps/examples/while_work.rs:batch-function"
```

`work` 採用非負的小整數。每次 `processed` 由當下剩餘量決定：至少 3 項就做 3 項，不足 3 項則做完剩下的部分。`remaining -= processed;` 是第一章已用過的減法並重新指派。

從 7 開始時，`remaining` 是剩餘量、`processed` 是本次處理量、`batches` 是完成批次，依序發生：

| 檢查前剩餘量 | 本次判斷與更新 | 完成批次 |
| --- | --- | --- |
| 7 | true；處理 3 項，剩 4 項 | 1 |
| 4 | true；處理 3 項，剩 1 項 | 2 |
| 1 | true；處理 1 項，剩 0 項 | 3 |
| 0 | false；不執行 body，仍剩 0 項 | 3 |

最後一列只檢查，沒有再多算一批。條件與更新接在一起，才表達出何時停止；只寫 `remaining > 0`，卻不在 body 減少剩餘量，就會一直成立。

## 零次執行也有明確答案

`batch_count(0)` 先讓 `remaining` 得到 0，第一次條件檢查就為 false。因此不會顯示任何「完成第幾批」，函式直接交回原本的 `batches`，也就是 0。

`processed` 是 body 裡當次使用的名稱；每次進入 body 才根據新剩餘量決定它。`remaining` 與 `batches` 則由外層保存，讓下一次檢查和最後的回傳都能使用更新後的值。

## 完整範例與執行

```toml
--8<-- "examples/intro-workspace/control-steps/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/control-steps/examples/while_work.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example while_work --locked
```

```text
完成第1批，這批處理3，剩餘4
完成第2批，這批處理3，剩餘1
完成第3批，這批處理1，剩餘0
七項工作共3批
零項工作共0批
```

上一節的 for 直接沿已有材料逐項處理；這次 while 直接表達「重新檢查剩餘量，還有工作就繼續」。這不表示 for 無法完成同樣計算，而是兩種寫法各讓不同的工作安排更直接可見。

## 練習與完整答案

把每批上限改成參數，定義 `batch_count(work: i32, capacity: i32) -> i32`。本題約定 `work >= 0`、`capacity > 0`，且都是不會讓計數溢位的小整數。分別計算 `(7, 3)`、`(6, 3)`、`(0, 3)`，預期為 3、2、0 批。

先想清楚：為什麼這裡要求上限為正？若剩餘量為 7、上限卻是 0，每次只減掉 0，條件永遠仍成立，函式就不會交回答案。輸入約定在這裡直接支撐停止規則。

```rust
--8<-- "examples/intro-workspace/control-steps/examples/while_work_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example while_work_answer --locked
```

完整答案沿用本頁 manifest，最後一批會調整為剩餘量，因此不會多處理或把剩餘量減成負數。下一節會學在 body 中途決定停止或跳過本次剩餘工作。

[下一節：loop、break 與 continue](02-05-lesson.md)
