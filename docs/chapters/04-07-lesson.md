# 4.7 用 let else 先排除不能繼續的情況

現在的工作是：報價有金額就加 20 元運費，沒有金額就結束這次顯示。如果後續還有幾個計算步驟，我們希望先取得 amount，讓成功路徑繼續往下讀；缺值的分支則在開頭離開。

## 成功的名稱留下，失敗的路徑離開

**`let pattern = expression else { ... };`** 先將右側值與 pattern 配對。成功時，pattern 建立的名稱留在**後面的目前 scope**中使用；失敗時執行 else，而 else 必須離開這條繼續往下的路徑。結尾的 `;` 結束整項 let 宣告。

下面的 Quote 沿用上一節的 Amount(i32)／Missing。參數是 &Quote，所以成功取得的 amount 是 &i32。失敗時用 `return;` 結束這個回傳 unit 的函式，後面便不會在缺少金額時繼續使用 amount：

```rust
--8<-- "examples/intro-workspace/patterns/examples/let_else.rs:let-else"
```

它沒有要求函式多包一層成功的 if block。amount 已在成功後建立，後面的加法與顯示可以照正常順序往下寫。

## 同一份資料，比較三種寫法

match 可以把 Amount 的內容交給外層變數，Missing 分支 return；if let 可以把計算放在成功 body；let else 則把失敗先排除。三種都能完成這項工作，差異在於如何安排分類與後續名稱範圍。

下面是三個函式及共同呼叫者的全部來源：

```toml
--8<-- "examples/intro-workspace/patterns/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/patterns/examples/let_else.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example let_else --locked
```

```text
match：含運=140
if let：含運=140
let else：含運=140
match：略過缺值
if let：略過缺值
let else：略過缺值
```

這個小例直接收 20 運費，先比較相同工作。if let 的 amount 在成功 body 中使用；另外兩種寫法把取得的 amount 留給後面的顯示。若工作需要每種形態各做不同計算，match 的分類仍較直接；若只在成功時做一小段工作，if let 已足夠。

## else 不能顯示一行後，就繼續使用不存在的 binding

編譯器用 **diverge** 表示不會沿這條路徑繼續到 let 後面的程式。只顯示文字仍會繼續，因此不足以處理 amount 沒有成功建立的情況；本例可以像成功版本一樣 return。

下面完整失敗例沒有離開 else，會得到 **E0308**，訊息指出 `else clause of let...else does not diverge`：

```rust
--8<-- "examples/intro-workspace/patterns/fixtures/compile-fail/let_else_must_leave.rs"
```

## 在迴圈裡，也可以 continue 或 break

離開這條路徑不一定要結束整個函式。第 2.5 節的 continue 會跳到下一次迴圈，break 會離開最近的迴圈，它們同樣能放在 let else 的失敗分支。

這次材料依序是 Amount(120)、Missing、Amount(80)。`&[Quote]` 像第 3.7 節的 &[i32] 一樣借讀連續資料，只是每項型別換成 Quote；for 每次取得 &Quote。對它用 Amount(amount) 配對，amount 因此是 &i32。

下面兩個函式只有缺值時的去向不同：第一個 continue，第二個 break。它們都在成功取得金額後才相加：

```rust
--8<-- "examples/intro-workspace/patterns/examples/let_else_loops.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example let_else_loops --locked
```

```text
略過缺值後合計=200，遇缺值停止合計=120
```

continue 讓最後的 80 仍被處理；break 在 Missing 就結束，沒有加上 80。兩者之後都回到函式尾端交出 total；若改用 return，離開的才會是整個函式。

## 章末完整程式：分類報價，再計算可用訂單

把本章的形態與上一章的文字訂單接起來。LabelledOrder 保存 owned 的 String 標籤、單價與數量；Quote 則分成 Missing、附帶整筆訂單的 Offered，以及附帶 code 的 Rejected。

本次沿用第 2 章的非負小訂單約定與計價規則：數量零總額為零；其他訂單小計至少 300 免運，否則收 20。先用 show_status 的 match 分別說明三種情況，再讓 total_if_offered 用 let else 只保留可計價的 Offered。對其餘形態，0 是本次計價統計的貢獻；原 Quote 仍保留缺少與拒絕的差別。

main 把三個 Quote 放在同型別的 array，再透過 slice 借讀。show_status、total_if_offered 都只接收 &Quote，因此顯示和計價後還能再次使用原資料。下面包含所有型別、helper 與呼叫者，設定仍是本頁 manifest：

```rust
--8<-- "examples/intro-workspace/patterns/examples/quote_report.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example quote_report --locked
```

```text
收到報價：訂單A
本次計價=180
尚無報價
本次計價=0
報價遭拒絕，代碼=37
本次計價=0
```

完整分類保存在 show_status；只有成功形態才需要的後續計算放在 total_if_offered。兩個函式各自選擇需要的 pattern 形式，沒有為了使用同一種語法而抹去不同工作的目的。

## 練習與完整答案

改成出貨資料：Shipment 的 Waiting 不附帶訂單，Ready 則直接附帶一筆數字 Order。寫 `total_if_ready(shipment: &Shipment) -> i32`，先用 let else 排除 Waiting，再按剛才相同規則計價。

驗證等待為 0、單價 80 數量 2 為 180、單價 300 數量 1 為 300、數量零為 0；再借用同一筆小訂單一次，仍應得到 180。完整答案沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/patterns/examples/let_else_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example let_else_answer --locked
```

Waiting 在 else 就離開，Ready 取得的 order 是 &Order；後面只讀它的數字欄位，不接手原訂單。下一章會把「可能有值／沒有值」等常見形態接到標準函式庫的型別。

[下一章：泛型與常見資料形態](05-01-lesson.md)
